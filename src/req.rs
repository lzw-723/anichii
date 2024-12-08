use crate::qb::Qbittorrent;
use rocket::serde::Deserialize;
use crate::ani::{Anime, Keyword};

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NewAnime<'a> {
    pub title: &'a str,
    pub sub: &'a str,
    pub rss: &'a str,
}

pub struct NewKeyword<'a> {
    pub keyword: &'a str,
    pub anime_id: u32,
}

impl From<NewKeyword<'_>> for Keyword {
    fn from(value: NewKeyword<'_>) -> Self {
        Keyword::new(
            String::from(value.keyword),
            value.anime_id,
        )
    }
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct TestQBittorrent<'a> {
    pub url: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}

impl From<TestQBittorrent<'_>> for Qbittorrent {
    fn from(value: TestQBittorrent) -> Self {
        Qbittorrent::new(
            String::from(value.url),
            String::from(value.username),
            String::from(value.password),
        )
    }
}


