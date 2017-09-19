use hyper::header::Headers;

use reqwest_helpers::{
    self, extract_html_elem,
    extract_url_from_text, extract_html_elem_in_vector
};
use reqwest_helpers::request::{
    self, send_form, send_req_command, send_ajax_req_command
};

use reqwest::Client;

use std::collections::HashMap;
use std::io::{self, Read};


error_chain!{
    links {
        Helper(reqwest_helpers::Error, reqwest_helpers::ErrorKind);
        Request(request::Error, request::ErrorKind);
    }
    foreign_links {
        IoError(io::Error);
    }
}

pub fn workout(client: &Client, headers: &Headers, xcsrftoken: &String) ->
Result<String> {
    let mut workout_resp = send_req_command(
        client, "https://www.endomondo.com/workouts/create",
        headers, None
    )?;

    let mut content = String::new();
    workout_resp.read_to_string(&mut content)?;

    let mut text = extract_html_elem(&content, "#ida", "onclick");
    let mut wurl = extract_url_from_text(&text)?;
    let mut w2url = format!("https://www.endomondo.com/{}&csrftoken={}&random=0.5534963460709013", wurl, xcsrftoken);

    let mut aux_resp = send_ajax_req_command(&client, &w2url, headers, None)?;

    let mut content = String::new();
    aux_resp.read_to_string(&mut content)?;

    wurl = extract_html_elem(&content, ".iframed", "src");
    w2url = format!("https://www.endomondo.com/{}", wurl);

    aux_resp = send_req_command(&client, &w2url, headers, None)?;

    content = String::new();
    aux_resp.read_to_string(&mut content)?;
    text = extract_html_elem(&content, ".next", "onclick");
    wurl = extract_url_from_text(&text)?;

    Ok(wurl)
}

pub fn other_workout(client: &Client, headers: &Headers, content: &String) ->
Result<()> {
    let text = extract_html_elem(content, ".prev", "onclick");
    let wurl = extract_url_from_text(&text)?;
    let w2url = format!("https://www.endomondo.com/{}&random=0.060340703880374935", wurl);
    let _ = send_ajax_req_command(&client, &w2url, headers, None)?;

    Ok(())
}

pub fn finalize_workout
(client: &Client, headers: &Headers, content: &String, xcsrftoken: &String) ->
Result<String> {
    let text = extract_html_elem(&content, ".next", "onclick");
    let wurl = extract_url_from_text(&text)?;
    let w2url = format!("https://www.endomondo.com/{}&random=0.060340703880374935", wurl);
    let mut params: HashMap<String, &str> = HashMap::new();

    params.insert(String::from("csrftoken"), &xcsrftoken);
    params.insert(String::from("reviewSumbit"), "1");

    let sports = extract_html_elem_in_vector(&content, "#workoutSport", "name");
    let mut count = 0;
    for x in &sports {
        params.insert(format!("workoutRow:{}:mark", count), "on");
        params.insert(x.clone(), "0");
        count += 1;
    }

    let mut content = String::new();
    let mut aux_resp = send_form(&client, &w2url, headers, &params)?;
    aux_resp.read_to_string(&mut content)?;

    Ok(content)
}
