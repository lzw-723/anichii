use std::io::{Error, ErrorKind};
use std::ops::Add;

#[derive(Debug)]
pub struct Qbittorrent {
    url: String,
    username: String,
    password: String,
    cookie: String,
}

impl Qbittorrent {
    pub fn new(url: String, username: String, password: String) -> Qbittorrent {
        Qbittorrent {
            url,
            username,
            password,
            cookie: "".to_string(),
        }
    }

    pub async fn login(&mut self) -> bool {
        let cookie = auth_login(
            self.url.clone(),
            self.username.clone(),
            self.password.clone(),
        )
        .await;
        match cookie {
            Ok(cookie) => {
                self.cookie = cookie;
                true
            }
            Err(_) => {
                println!("Login failed");
                false
            }
        }
    }

    pub async fn add_torrent(&mut self, torrent: String) -> bool {
        torrents_add(self.url.clone(), self.cookie.clone(), torrent, "".to_string()).await.is_ok()
    }

    pub fn add_magnet(&mut self, magnet: String) -> bool {
        todo!()
    }

    fn get_tasks(&self) -> Vec<String> {
        todo!()
    }

    pub fn set_cookie(&mut self, cookie: String) {
        self.cookie = cookie;
    }
}

async fn auth_login(
    url: String,
    username: String,
    password: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post(url.add("api/v2/auth/login"))
        .form(&[("username", username), ("password", password)])
        .send()
        .await?;
    let headers = response.headers().clone();
    let body = response.text().await?;
    match body.as_str() {
        "Ok." => {
            let cookie_header = headers.get("set-cookie").unwrap().to_str().unwrap();
            return Ok(String::from(cookie_header));
        }
        _ => {
            println!("Login failed");
        }
    }
    Err(Error::from(ErrorKind::NotFound).into())
}

async fn auth_logout(url: String, cookie: String) -> Result<bool, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post(url.add("api/v2/auth/logout"))
        .header("cookie", cookie)
        .send()
        .await?;
    let code = response.status().as_u16();
    match code {
        200 => {
            println!("Logout success");
            Ok(true)
        }
        _ => {
            println!("Logout failed");
            Ok(false)
        }
    }
}

async fn app_version(url: String, cookie: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post(url.add("api/v2/app/version"))
        .header("cookie", cookie)
        .send()
        .await?;
    let body = response.text().await?;
    // The response is a string with the application version, e.g. v4.1.3
    Ok(body)
}

async fn app_webapi_version(
    url: String,
    cookie: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post(url.add("api/v2/app/webapiVersion"))
        .header("cookie", cookie)
        .send()
        .await?;
    let body = response.text().await?;
    // The response is a string with the WebAPI version, e.g. 2.0
    Ok(body)
}

async fn app_build_info(url: String, cookie: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post(url.add("api/v2/app/buildInfo"))
        .header("cookie", cookie)
        .send()
        .await?;
    let body = response.text().await?;
    // The response is a JSON object containing the following fields
    //
    // Property	Type	Description
    // qt	string	QT version
    // libtorrent	string	libtorrent version
    // boost	string	Boost version
    // openssl	string	OpenSSL version
    // bitness	int	Application bitness (e.g. 64-bit)
    Ok(body)
}

async fn app_default_save_path(
    url: String,
    cookie: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post(url.add("api/v2/app/defaultSavePath"))
        .header("cookie", cookie)
        .send()
        .await?;
    let body = response.text().await?;
    // The response is a string with the default save path, e.g. C:/Users/Dayman/Downloads.
    Ok(body)
}

async fn torrents_add(
    url: String,
    cookie: String,
    torrent: String,
    save_path: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let form = reqwest::multipart::Form::new()
        .text("urls", torrent)
        .text("savepath", save_path)
        .text("tags", "ani-chii")
        .text("cookie", cookie.clone())
        // .text("category", "value5")
        .text("paused", "false")
        .text("skip_checking", "false")
        .text("root_folder", "true");
    let response = client
        .post(url.add("api/v2/torrents/add"))
        .header("cookie", cookie)
        .multipart(form)
        .send()
        .await?;
    let code = response.status().as_u16();
    match code {
        200 => Ok(()),
        _ => {
            // 415	Torrent file is not valid
            Err(Error::from(ErrorKind::NotFound).into())
        }
    }
}
