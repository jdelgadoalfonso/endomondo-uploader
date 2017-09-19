use config;

use reqwest_helpers::request::{self, send_req_command};

use reqwest::{self, Client, Response};

use std::collections::HashMap;
use std::env;


error_chain!{
    links {
        Request(request::Error, request::ErrorKind);
    }
    foreign_links {
        ReqError(reqwest::Error);
    }
}

pub fn login(client: &Client) -> Result<Response> {
    let initial_resp = client.get("https://www.endomondo.com/").send()?;

    match env::home_dir() {
        Some(path) => println!("{}", path.display()),
        None => println!("Impossible to get your home dir!"),
    }

    let home = env::home_dir().unwrap();
    let mut settings = config::Config::default();
    settings
        // Add in `./Settings.toml`
        .merge(config::File::with_name(
            &format!("{}/.endomondo", home.display()
        ))).unwrap();

    let email = settings.get::<String>("email").unwrap();
    let password = settings.get::<String>("password").unwrap();
    let remember = String::from("true");

    let mut json_data = HashMap::new();

    json_data.insert("email", &email);
    json_data.insert("password", &password);
    json_data.insert("remember", &remember);

    let login_resp = send_req_command(
        &client, "https://www.endomondo.com/rest/session",
        initial_resp.headers(), Some(json_data)
    )?;

    let home_resp = send_req_command(
        &client, "https://www.endomondo.com/home",
        login_resp.headers(), None
    )?;

    Ok(home_resp)
}

