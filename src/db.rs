use crate::ani::{Anime, Episode, Keyword};
use rusqlite::{params, Connection};
use std::fs;
use std::sync::{Arc, Mutex};

pub struct DB {
    conn: Arc<Mutex<Connection>>,
    mem_conn: Arc<Mutex<Connection>>,
}

impl DB {
    pub fn new() -> Result<DB, rusqlite::Error> {
        fs::create_dir_all("./config").expect("创建数据库文件目录失败");
        let conn = Connection::open("./config/ani-chii.db")?;
        let mem_conn = Connection::open_in_memory()?;
        let db = DB {
            conn: Arc::new(Mutex::new(conn)),
            mem_conn: Arc::new(Mutex::new(mem_conn)),
        };
        db.init();
        Ok(db)
    }

    pub fn init(&self) -> bool {
        self.mem_conn.lock().unwrap().execute("CREATE TABLE IF NOT EXISTS Episode (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT, link TEXT, torrent TEXT, description TEXT, anime_id INTEGER NOT NULL)", []).unwrap();
        self.conn.lock().unwrap().execute("CREATE TABLE IF NOT EXISTS Anime (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT, sub TEXT, rss TEXT)", []).unwrap();
        self.conn.lock().unwrap().execute("CREATE TABLE IF NOT EXISTS Keyword (id INTEGER PRIMARY KEY AUTOINCREMENT, keyword TEXT NOT NULL, anime_id INTEGER)", []).unwrap();
        true
    }

    pub fn save_anime(&self, anime: Anime) -> bool {
        self.conn
            .lock()
            .unwrap()
            .execute(
                "INSERT INTO Anime (title, sub, rss) VALUES (?,?,?)",
                [&anime.title, &anime.sub, &anime.rss],
            )
            .is_ok()
    }

    pub fn get_animes(&self) -> Vec<Anime> {
        self.conn
            .lock()
            .unwrap()
            .prepare("SELECT id, title, sub, rss FROM Anime")
            .unwrap()
            .query_map([], |row| {
                let anime = Anime::new(
                    Some(row.get(0)?),
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                );
                Ok(anime)
            })
            .unwrap()
            .collect::<Result<Vec<Anime>, rusqlite::Error>>()
            .unwrap()
    }

    pub fn get_anime(&self, id: u32) -> Option<Anime> {
        self.conn
            .lock()
            .unwrap()
            .query_row(
                "SELECT id, title, sub, rss FROM Anime WHERE id =?",
                [id],
                |row| {
                    let mut anime = Anime::new(
                        Some(row.get(0)?),
                        row.get(1)?,
                        row.get(2)?,
                        row.get(3)?,
                    );
                    Ok(anime)
                },
            )
            .ok()
    }

    pub fn delete_anime(&self, id: u32) -> bool {
        self.conn
            .lock()
            .unwrap()
            .execute("DELETE FROM Anime WHERE id =?", [id])
            .is_ok()
    }

    pub fn save_episode(&self, episode: Episode) -> bool {
        self.mem_conn
            .lock()
            .unwrap()
            .execute(
                "INSERT INTO Episode (title, link, torrent, description, anime_id) VALUES (?,?,?,?,?)",
                params![
                    episode.get_title(),
                    episode.get_link(),
                    episode.get_torrent(),
                    episode.get_description(),
                    episode.get_anime_id(),
                ],
            )
            .is_ok()
    }

    pub fn get_episode(&self, id: u32) -> Option<Episode> {
        self.mem_conn
            .lock()
            .unwrap()
            .query_row(
                "SELECT id, title, link, torrent, description, anime_id FROM Episode WHERE id =?",
                [id],
                |row| {
                    let mut episode =
                        Episode::new(row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?);
                    episode.set_id(row.get(0)?);
                    episode.set_anime_id(row.get(5)?);
                    Ok(episode)
                },
            )
            .ok()
    }
    pub fn delete_episode(&self, id: u32) -> bool {
        self.mem_conn
            .lock()
            .unwrap()
            .execute("DELETE FROM Episode WHERE id =?", [id])
            .is_ok()
    }

    pub fn delete_episodes_by_anime_id(&self, id: u32) -> bool {
        self.mem_conn
            .lock()
            .unwrap()
            .execute("DELETE FROM Episode WHERE anime_id =?", [id])
            .is_ok()
    }

    // pub fn list_episodes(&self) -> Vec<Episode> {
    //     self.mem_conn
    //         .lock()
    //         .unwrap()
    //         .prepare("SELECT id, title, link, torrent, description, anime_id FROM Episode")
    //         .unwrap()
    //         .query_map([], |row| {
    //             let mut episode = Episode::new(row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?);
    //             episode.set_id(row.get(0)?);
    //             episode.set_anime_id(row.get(5)?);
    //             Ok(episode)
    //         })
    //         .unwrap()
    //         .collect::<Result<Vec<Episode>, rusqlite::Error>>()
    //         .unwrap()
    // }

    pub fn list_episodes_by_anime_id(&self, id: u32) -> Vec<Episode> {
        self.mem_conn
            .lock()
            .unwrap()
            .prepare("SELECT id, title, link, torrent, description, anime_id FROM Episode WHERE anime_id =?")
            .unwrap()
            .query_map([id], |row| {
                let mut episode = Episode::new(
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                );
                episode.set_id(row.get(0)?);
                episode.set_anime_id(row.get(5)?);
                Ok(episode)
            })
            .unwrap()
            .collect::<Result<Vec<Episode>, rusqlite::Error>>()
            .unwrap()
    }

    pub fn save_keyword(&self, keyword: Keyword) -> bool {
        self.conn
            .lock()
            .unwrap()
            .execute(
                "INSERT INTO Keyword (keyword, anime_id) VALUES (?,?)",
                params![keyword.keyword, keyword.anime_id],
            )
            .is_ok()
    }
    
    pub fn delete_keyword_by_id(&self, id: u32) -> bool {
        self.conn
            .lock()
            .unwrap()
            .execute("DELETE FROM Keyword WHERE id =?", params![id])
            .is_ok()
    }
    
    pub fn delete_keywords_by_anime_id(&self, anime_id: u32) -> bool {
        self.conn
          .lock()
          .unwrap()
          .execute("DELETE FROM Keyword WHERE anime_id =?", params![anime_id])
          .is_ok()
    }

    pub fn get_keywords_by_anime_id(&self, anime_id: u32) -> Vec<Keyword> {
        self.conn
            .lock()
            .unwrap()
            .prepare("SELECT id, keyword, anime_id FROM Keyword WHERE anime_id =?")
            .unwrap()
            .query_map([anime_id], |row| {
                Ok(Keyword {
                    id: Some(row.get(0)?),
                    keyword: row.get(1)?,
                    anime_id: row.get(2)?,
                })
            })
            .unwrap()
            .collect::<Result<Vec<Keyword>, rusqlite::Error>>()
            .unwrap()
    }
}
