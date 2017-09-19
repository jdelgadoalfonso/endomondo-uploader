#![feature(inclusive_range_syntax)]
extern crate config;

#[macro_use] extern crate error_chain;

#[macro_use] extern crate hyper;
extern crate kuchiki;
#[macro_use] extern crate lazy_static;
extern crate pbr;
extern crate regex;
extern crate reqwest;

use file_utils::list_of_files;

use pbr::ProgressBar;

use reqwest::Client;

use reqwest_helpers::{create_header, extract_cokies_from_header};
use reqwest_helpers::request::send_file;
use reqwest_helpers::login::login;
use reqwest_helpers::workout::{workout, other_workout, finalize_workout};

use std::time::Duration;
use std::thread;

pub mod file_utils;

pub mod reqwest_helpers;


error_chain! {
    links {
        Helper(reqwest_helpers::Error,
            reqwest_helpers::ErrorKind);
        Login(reqwest_helpers::login::Error,
            reqwest_helpers::login::ErrorKind);
        Request(reqwest_helpers::request::Error,
            reqwest_helpers::request::ErrorKind);
        Workout(reqwest_helpers::workout::Error,
            reqwest_helpers::workout::ErrorKind);
    }
}

fn run() -> Result<()> {
    let f = list_of_files();
    let client = Client::new();

    let login_resp = login(&client)?;
    let login_resp_headers = login_resp.headers();
    let cookie = extract_cokies_from_header(login_resp_headers);
    let xcsrftoken = String::from(cookie.get("CSRF_TOKEN").unwrap());

    println!("LOGIN DONE");

    let wurl = workout(&client, login_resp_headers, &xcsrftoken)?;

    println!("DIALOG DONE");

    let mut p = f.len();
    let mut pb = ProgressBar::new(p as u64);
    pb.format("╢▌▌░╟");

    println!("STARTING UPLOAD");

    for file in f.into_iter() {
        thread::sleep(Duration::new(3, 0));

        pb.message(format!("{}  ", file).as_ref());

        let content = send_file(&client, login_resp_headers, &wurl, &xcsrftoken, &file)?;

        pb.inc();
        p -= 1;

        if p == 0 {
            let resp = finalize_workout(&client, login_resp_headers, &content, &xcsrftoken)?;
            println!("\nFINAL RESPONSE: {}", resp);
            pb.finish_print("DONE");
            break;
        }

        other_workout(&client, login_resp_headers, &content)?;
    }
 
    Ok(())
}

fn main() {
    run().unwrap();
}
