use hyper::header::{Cookie, Headers, SetCookie};

use kuchiki;
use kuchiki::traits::TendrilSink;

use regex::Regex;

use std::fmt::{self, Write};

pub mod login;

pub mod request;

pub mod workout;


error_chain!{
    foreign_links {
        FmtError(fmt::Error);
    }
}

header! { (XCSRFTOKEN, "X-CSRF-TOKEN") => [String] }

pub fn extract_html_elem(content: &str, elem: &str, attr: &str) -> String {
    let document = kuchiki::parse_html().one(content);
    let mut wurl = String::new();

    for css_match in document.select(elem).unwrap() {
        let as_node = css_match.as_node();
        let as_elem = as_node.as_element().unwrap().clone();
        wurl = String::from(as_elem.attributes.into_inner().get(attr).unwrap());
    }

    wurl
}

pub fn extract_html_elem_in_vector(content: &str, elem: &str, attr: &str) ->
Vec<String> {
    let document = kuchiki::parse_html().one(content);
    let mut vector = Vec::new();

    for css_match in document.select(elem).unwrap() {
        let as_node = css_match.as_node();
        let as_elem = as_node.as_element().unwrap().clone();
        let text = String::from(as_elem.attributes.into_inner().get(attr).unwrap());

        vector.push(text);
    }

    vector
}

pub fn extract_url_from_text(text: &str) -> Result<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\?x=[\w\-\*\\]+").unwrap();
    }
    let mut wurl = String::new();
    let result = RE.find_iter(text);
  
    for m in result {
        wurl.write_str(&text[m.start()..m.end()])?;
    }

    Ok(wurl)
}

pub fn extract_cokies_from_header(headers: &Headers) -> Cookie {
    let mut cookie = Cookie::new();
    if let Some(&SetCookie(ref content)) = headers.get() {
        for def in content {
            let fields = def.to_string();
            let fields_string = fields.split(';').collect::<Vec<&str>>();
            for f in fields_string {
                let fi = f.to_string().clone();
                let mut split = fi.split(|c: char|
                    if c == '=' || c == ';' { 
                        true
                    } else {
                        false
                    }
                );
                let key = split.next().unwrap();
                let value = split.next().unwrap_or("");
                match key.as_ref() {
                    "CSRF_TOKEN" => cookie.set(key.to_string(), value.to_string()),
                    "JSESSIONID" => cookie.set(key.to_string(), value.to_string()),
                    "AWSELB" => cookie.set(key.to_string(), value.to_string()),
                    "EndomondoApplication_USER" => cookie.set(key.to_string(), value.to_string()),
                    "EndomondoApplication_AUTH" => cookie.set(key.to_string(), value.to_string()),
                    _ => (),
                }
            }
        }
        cookie.set("acceptCookies", "1");
    }
    cookie
}

pub fn create_header(cookie: &Cookie) -> Headers {
    let mut headers = Headers::new();

    headers.set(XCSRFTOKEN(
        String::from(cookie.get("CSRF_TOKEN").unwrap_or(""))
    ));
    headers.set(cookie.clone());

    headers
}
