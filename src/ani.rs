use crate::qb::Qbittorrent;
use rocket::serde::json::Json;
use rocket::serde::{json, Deserialize, Serialize};
use rusqlite::Error;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Anime {
    pub id: Option<u32>,
    pub title: String,
    pub sub: String,
    pub rss: String,
    episodes: Vec<Episode>,
    keywords: Vec<Keyword>,
}

impl Anime {
    pub fn new(id: Option<u32>, title: String, sub: String, rss: String) -> Anime {
        Anime {
            id,
            title,
            sub,
            rss,
            episodes: Vec::new(),
            keywords: Vec::new(),
        }
    }

    pub fn get_episodes(&self) -> Vec<Episode> {
        self.episodes.clone()
    }

    pub fn add_episode(&mut self, episode: Episode) {
        self.episodes.push(episode);
    }

    pub fn set_episodes(&mut self, episodes: Vec<Episode>) {
        self.episodes.clear();
        self.episodes.extend(episodes);
    }

    pub fn get_keywords(&self) -> Vec<Keyword> {
        self.keywords.clone()
    }

    pub fn add_keyword(&mut self, keyword: Keyword) {
        self.keywords.push(keyword);
    }

    pub fn set_keywords(&mut self, keywords: Vec<Keyword>) {
        self.keywords.clear();
        self.keywords.extend(keywords);
    }
}

// 定义FromIterator特征
trait FromIterator<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self;
}

// 为Vec<Anime>实现FromIterator<Result<Anime, MyError>>特征
impl FromIterator<Result<Anime, Error>> for Vec<Anime> {
    fn from_iter<I: IntoIterator<Item = Result<Anime, Error>>>(iter: I) -> Self {
        iter.into_iter().filter_map(|result| result.ok()).collect()
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
#[derive(Clone)]
pub struct Episode {
    id: Option<u32>,
    title: String,
    link: String,
    torrent: String,
    description: String,
    anime_id: Option<u32>,
    ignore: Option<bool>,
}

impl Episode {
    pub fn new(title: String, link: String, torrent: String, description: String) -> Episode {
        Episode {
            id: None,
            title,
            link,
            torrent,
            description,
            anime_id: None,
            ignore: None,
        }
    }
    pub fn get_id(&self) -> u32 {
        self.id.unwrap()
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = Some(id);
    }

    pub fn get_anime_id(&self) -> u32 {
        self.anime_id.unwrap()
    }

    pub fn set_anime_id(&mut self, id: u32) {
        self.anime_id = Some(id);
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn get_link(&self) -> String {
        self.link.clone()
    }
    pub fn set_link(&mut self, link: String) {
        self.link = link;
    }

    pub fn get_torrent(&self) -> String {
        self.torrent.clone()
    }
    pub fn set_torrent(&mut self, torrent: String) {
        self.torrent = torrent;
    }
    pub fn get_description(&self) -> String {
        self.description.clone()
    }
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
    pub fn set_ignore(&mut self, ignore: bool) {
        self.ignore = Some(ignore);
    }
    pub fn get_ignore(&self) -> bool {
        self.ignore.unwrap()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[derive(Clone)]
pub struct Keyword {
    pub id: Option<u32>,
    pub keyword: String,
    pub anime_id: u32,
}

impl Keyword {
    pub fn new(keyword: String, anime_id: u32) -> Keyword {
        Keyword {
            id: None,
            keyword,
            anime_id,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Setting {
    qb_url: String,
    qb_username: String,
    qb_password: String,
    qb_save_path: String,
    qb_cookie: Option<String>,
}
impl Setting {
    pub fn new(
        qb_url: String,
        qb_username: String,
        qb_password: String,
        qb_save_path: String,
    ) -> Setting {
        Setting {
            qb_url,
            qb_username,
            qb_password,
            qb_save_path,
            qb_cookie: Option::None,
        }
    }
    pub fn get_qb_url(&self) -> String {
        self.qb_url.clone()
    }
    pub fn get_qb_username(&self) -> String {
        self.qb_username.clone()
    }
    pub fn get_qb_password(&self) -> String {
        self.qb_password.clone()
    }
    pub fn get_qb_save_path(&self) -> String {
        self.qb_save_path.clone()
    }
    pub fn set_qb_url(&mut self, qb_url: String) {
        self.qb_url = qb_url;
    }
    pub fn set_qb_username(&mut self, username: String) {
        self.qb_username = username;
    }
    pub fn set_qb_password(&mut self, password: String) {
        self.qb_password = password;
    }
    pub fn set_qb_save_path(&mut self, path: String) {
        self.qb_save_path = path;
    }
    pub fn set_qb_cookie(&mut self, cookie: String) {
        self.qb_cookie = Option::Some(cookie);
    }
    pub fn get_qb_cookie(&self) -> Option<String> {
        self.qb_cookie.clone()
    }

    pub fn save(&self) {
        let str = json::to_string(&self).unwrap();
        let mut file = File::create("./config/setting.json").unwrap();
        file.write_all(str.as_bytes()).unwrap();
    }

    pub fn load() -> Setting {
        match File::open("./config/setting.json") {
            Ok(mut file) => {
                let mut str = String::new();
                file.read_to_string(&mut str).unwrap();
                let setting: Setting = json::from_str(&*str).unwrap();
                setting
            }
            Err(_) => {
                let setting = Setting::new(
                    String::from(""),
                    String::from(""),
                    String::from(""),
                    String::from(""),
                );
                setting.save();
                setting
            }
        }
    }

    pub async fn get_qb(&self) -> Qbittorrent {
        let mut qb = Qbittorrent::new(
            self.qb_url.clone(),
            self.qb_username.clone(),
            self.qb_password.clone(),
        );
        match self.qb_cookie {
            Some(ref cookie) => {
                qb.set_cookie(cookie.clone());
            }
            _ => {
                qb.login().await;
            }
        }
        qb
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub anime_id: u32,
    pub title: String,
    pub link: String,
    pub torrent: String,
}
