mod ani;
mod db;
mod qb;
mod req;
mod resp;
mod rss;

#[macro_use]
extern crate rocket;

use crate::ani::{Anime, Episode, Keyword, Setting, Task};
use crate::db::DB;
use crate::qb::Qbittorrent;
use crate::req::{NewAnime, TestQBittorrent};
use crate::rss::{parse_rss, parse_title};
use reqwest::Error;
use rocket::serde::json::Json;
use rocket::serde::{json, Deserialize, Serialize};
use rocket::State;
use std::clone::Clone;
use std::path::PathBuf;
use std::sync::Arc;
use include_dir::{include_dir, Dir};
use rocket::http::ContentType;

#[get("/version")]
async fn version() -> &'static str {
    "0.1.0"
}

#[get("/anime")]
async fn list_anime(db: &State<Arc<DB>>) -> Json<Vec<Anime>> {
    let mut animes = db.get_animes();
    for mut a in &mut animes {
        a.set_episodes(db.list_episodes_by_anime_id(a.id.unwrap()));
        a.set_keywords(db.get_keywords_by_anime_id(a.id.unwrap()));
    }
    Json(animes)
}

#[get("/anime/<id>")]
async fn get_anime(id: u32, db: &State<Arc<DB>>) -> Json<Anime> {
    let mut anime = db.get_anime(id).unwrap();
    anime.set_episodes(db.list_episodes_by_anime_id(id));
    anime.set_keywords(db.get_keywords_by_anime_id(id));
    Json(anime)
}

#[post("/anime", data = "<anime>")]
async fn add_anime(mut anime: Json<NewAnime<'_>>, db: &State<Arc<DB>>) {
    let mut eps = fetch_rss(String::from(anime.rss)).await.unwrap();
    let ep = eps.get(0).unwrap();
    let t = ep.get_title();
    let (title, sub) = parse_title(&*t);
    if anime.title.is_empty() {
        anime.title = title;
    }
    if anime.sub.is_empty() {
        anime.sub = sub;
    }
    db.save_anime(Anime::new(
        None,
        String::from(anime.title),
        String::from(anime.sub),
        String::from(anime.rss),
    ));
}

#[delete("/anime/<id>")]
fn del_anime(id: u32, db: &State<Arc<DB>>) {
    println!("del anime {}", id);
    db.delete_anime(id);
}

#[put("/anime", data = "<anime>")]
fn modify_anime(anime: Json<Anime>, db: &State<Arc<DB>>) {
    println!("modify anime");
    db.save_anime(anime.into_inner());
}

async fn fetch_rss(rss: String) -> Result<Vec<Episode>, Error> {
    let client = reqwest::Client::new();
    // "https://mikanani.me/RSS/Bangumi?bangumiId=3443&subgroupid=370"
    let rep = client.get(rss).send().await?;
    let body = rep.text().await?;
    let eps = parse_rss(body.as_str());
    Ok(eps)
}

#[get("/setting")]
fn get_setting() -> Json<Setting> {
    Json(Setting::load())
}

#[post("/setting", data = "<setting>")]
fn set_setting(setting: Json<Setting>) {
    setting.save();
}

#[get("/anime/<anime_id>/episode")]
fn get_episodes(anime_id: u32, db: &State<Arc<DB>>) -> Json<Vec<Episode>> {
    let mut episodes = db.list_episodes_by_anime_id(anime_id);
    let keywords = db.get_keywords_by_anime_id(anime_id);
    for e in &mut episodes {
        e.set_ignore(false);
        keywords.iter().for_each(|k| {
            if e.get_title().contains(k.keyword.as_str()) {
                e.set_ignore(true);
            }
        });
    }
    Json(episodes)
}

#[get("/anime/<anime_id>/keyword")]
fn get_keywords(anime_id: u32, db: &State<Arc<DB>>) -> Json<Vec<Keyword>> {
    let keywords = db.get_keywords_by_anime_id(anime_id);
    Json(keywords)
}
#[post("/anime/<anime_id>/keyword", data = "<keyword>")]
fn add_keyword(anime_id: u32, keyword: Json<Keyword>, db: &State<Arc<DB>>) {
    db.save_keyword(keyword.into_inner());
}
#[delete("/keyword/<id>")]
fn del_keyword(id: u32, db: &State<Arc<DB>>) {
    db.delete_keyword_by_id(id);
}

#[post("/test/qb", data = "<qb>")]
async fn test_qb(qb: Json<TestQBittorrent<'_>>) -> Result<(), resp::Error> {
    let mut qbittorrent = Qbittorrent::from(qb.into_inner());
    if qbittorrent.login().await {
        Ok(())
    } else {
        Err(resp::Error::TestQBittorrentError(()))
    }
}

async fn add_tasks(task: Task) {
    let mut qbittorrent = Setting::load().get_qb().await;
    match qbittorrent.add_torrent(task.torrent).await {
        true => {
            println!("添加任务成功");
        }
        false => {
            println!("添加任务失败");
        }
    };
}


// 修改自https://github.com/rwf2/Rocket/discussions/2005#discussioncomment-9243023
#[rocket::get("/<path..>", rank = 100)]
async fn static_files(mut path: PathBuf) -> Option<(ContentType, &'static [u8])> {
    if path == PathBuf::from("") {
        path = PathBuf::from("index.html");
    }
    static PROJECT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/webui/dist");

    let data = PROJECT_DIR.get_file(&path)
        .map(|file| file.contents())?;

    let content_type = path.extension()
        .and_then(|e| e.to_str())
        .and_then(ContentType::from_extension)
        .unwrap_or(ContentType::Binary);

    Some((content_type, data))
}

#[launch]
async fn rocket() -> _ {
    let db = Arc::new(DB::new().unwrap());
    let db2 = db.clone();
    tokio::task::spawn(async move {
        loop {
            println!("拉取rss");
            for a in db.get_animes() {
                if let Ok(eps) = fetch_rss(a.rss.clone()).await {
                    println!("{:#?}", a.title);
                    db.delete_episodes_by_anime_id(a.id.unwrap());
                    for mut ep in eps {
                        ep.set_anime_id(a.id.unwrap());
                        ep.set_ignore(false);
                        if db
                            .get_keywords_by_anime_id(a.id.unwrap())
                            .iter()
                            .any(|k| ep.get_title().contains(k.keyword.as_str()))
                        {
                            println!("已忽略{}", ep.get_title());
                            ep.set_ignore(true);
                        }
                        db.save_episode(ep.clone());
                        if ep.get_ignore() {
                            continue;
                        }
                        println!("已添加{}", ep.get_title());
                        add_tasks(Task {
                            anime_id: a.id.unwrap(),
                            title: ep.get_title().to_string(),
                            link: ep.get_link().to_string(),
                            torrent: ep.get_torrent().to_string(),
                        })
                        .await;
                    }
                } else {
                    println!("fetch rss failed");
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(45)).await;
        }
    });
    rocket::build()
        .mount(
            "/api/v1",
            routes![
                version,
                list_anime,
                get_anime,
                add_anime,
                del_anime,
                modify_anime,
                get_episodes,
                get_keywords,
                add_keyword,
                del_keyword,
                get_setting,
                set_setting,
                test_qb,
            ],
        ).mount(
            "/",
            routes![static_files],)
        .manage(db2)
}
