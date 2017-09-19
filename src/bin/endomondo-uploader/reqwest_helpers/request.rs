use {extract_cokies_from_header, create_header};

use hyper::header::Headers;

use reqwest::{self, Client, RequestBuilder, Response};
use reqwest::multipart::Form;

use std::collections::HashMap;
use std::io::{self, Read};


error_chain!{
    foreign_links {
        ReqError(reqwest::Error);
        IoError(io::Error);
    }
}

header! { (WICKETAJAX, "wicket-ajax") => [String] }

pub type OptJson<'a> = Option<HashMap<&'a str, &'a String>>;

pub fn send_request(rb: &mut RequestBuilder, data: OptJson) -> Result<Response> {
    let resp = match data {
        Some(x) => rb.json(&x).send()?,
        None    => rb.send()?,
    };

    Ok(resp)
}

pub fn send_req_command
(client: &Client, url: &str, resp_headers: &Headers, data: OptJson) ->
Result<Response> {
    let cookie = extract_cokies_from_header(resp_headers);
    let headers = create_header(&cookie);
    let mut rb = match data {
        Some(_) => client.post(url),
        None    => client.get(url)
    };
    
    rb.headers(headers);

    send_request(&mut rb, data)
}

pub fn send_ajax_req_command
(client: &Client, url: &str, resp_headers: &Headers, data: OptJson) ->
Result<Response> {
    let cookie = extract_cokies_from_header(resp_headers);
    let mut headers = create_header(&cookie);
    let mut rb = match data {
        Some(_) => client.post(url),
        None    => client.get(url)
    };

    headers.set(WICKETAJAX(String::from("true")));
    
    rb.headers(headers);

    send_request(&mut rb, data)
}

pub fn send_file
(client: &Client, headers: &Headers, wurl: &String, xcsrftoken: &String, file: &String) ->
Result<String> {
    let w2url = format!("https://www.endomondo.com/{}&wicket:ajax=true", wurl);
    let form = Form::new()
        .text("csrftoken", xcsrftoken.clone())
        .text("uploadSumbit", "1")
        .file("uploadFile", file)?;
    let mut aux_resp = send_file_request(&client, &w2url, headers, form)?;
    let mut content = String::new();

    aux_resp.read_to_string(&mut content)?;

    Ok(content)
}

pub fn send_form
(client: &Client, url: &str, resp_headers: &Headers, params: &HashMap<String, &str>) ->
Result<Response> {
    let cookie = extract_cokies_from_header(resp_headers);
    let mut headers = Headers::new();
    let mut req = client.post(url);

    headers.set(cookie.clone());
    headers.set(WICKETAJAX(String::from("true")));

    req.headers(headers);
    req.form(params);

    let resp = req.send()?;

    Ok(resp)
}

fn send_file_request
(client: &Client, url: &str, resp_headers: &Headers, form: Form) ->
Result<Response> {
    let cookie = extract_cokies_from_header(resp_headers);
    let headers = create_header(&cookie);

    let resp = client.post(url)
        .headers(headers)
        .multipart(form)
        .send()?;

    Ok(resp)
}
