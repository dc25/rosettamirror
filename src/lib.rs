extern crate reqwest;
extern crate url;
extern crate rustc_serialize;

use std::fs::*;
use std::io::prelude::*;

use std::io::Read;
use self::url::Url;
use rustc_serialize::json::{self, Json};

struct Task {
    page_id: u64,
    pub title: String,
}

pub enum ParseError {
    /// Something went wrong with the HTTP request to the API.
    Http(reqwest::Error),
 
    /// There was a problem parsing the API response into JSON.
    Json(json::ParserError),
 
    /// There was a problem parsing the API response into JSON.
    Io(std::io::Error),
 
    /// Unexpected JSON format from response
    UnexpectedFormat,
}
impl From<json::ParserError> for ParseError {
    fn from(error: json::ParserError) -> Self {
        ParseError::Json(error)
    }
}
 
impl From<reqwest::Error> for ParseError {
    fn from(error: reqwest::Error) -> Self {
        ParseError::Http(error)
    }
}
 
impl From<std::io::Error> for ParseError {
    fn from(error: std::io::Error) -> Self {
        ParseError::Io(error)
    }
}
 
fn construct_query_category(category: &str) -> Url {
    let mut base_url = Url::parse("http://rosettacode.org/mw/api.php").unwrap();
    let cat = format!("Category:{}", category);
    let query_pairs = vec![("action", "query"),
                           ("format", "json"),
                           ("list", "categorymembers"),
                           ("cmlimit", "500"),
                           ("cmtitle", &cat),
                           ("continue", "")];
    base_url.query_pairs_mut().extend_pairs(query_pairs.into_iter());
    base_url
}
 
fn construct_query_task_content(task_id: &str) -> Url {
    let mut base_url = Url::parse("http://rosettacode.org/mw/api.php").unwrap();
    let mut query_pairs =
        vec![("action", "query"), ("format", "json"), ("prop", "revisions"), ("rvprop", "content")];
    query_pairs.push(("pageids", task_id));
    base_url.query_pairs_mut().extend_pairs(query_pairs.into_iter());
    base_url
}
 
fn query_api(url: Url) -> Result<Json, ParseError> {
    let mut response = (reqwest::get(url.as_str()))?;
    // Build JSON
    let mut body = String::new();
    response.read_to_string(&mut body)?;
 
    Ok((Json::from_str(&body))?)
}
 
fn parse_all_tasks(reply: &Json) -> Result<Vec<Task>, ParseError> {
    let json_to_task = |json: &Json| -> Result<Task, ParseError> {
        let page_id: u64 = (json.find("pageid")
            .and_then(|id| id.as_u64())
            .ok_or(ParseError::UnexpectedFormat))?;
        let title: &str = (json.find("title")
            .and_then(|title| title.as_string())
            .ok_or(ParseError::UnexpectedFormat))?;
 
        Ok(Task {
            page_id: page_id,
            title: title.to_owned(),
        })
    };
    let tasks_json = (reply.find_path(&["query", "categorymembers"])
        .and_then(|tasks| tasks.as_array())
        .ok_or(ParseError::UnexpectedFormat))?;
 
    // Convert into own type
    tasks_json.iter().map(json_to_task).collect()
}
fn get_task(task: &Json, task_id: u64) -> Result<String, ParseError> {
    let revisions =
        (task.find_path(&["query", "pages", task_id.to_string().as_str(), "revisions"])
            .and_then(|content| content.as_array())
            .ok_or(ParseError::UnexpectedFormat))?;
    let content = (revisions[0]
        .find("*")
        .and_then(|content| content.as_string())
        .ok_or(ParseError::UnexpectedFormat))?;
    Ok(String::from(content))
}
 
fn query_all_tasks() -> Result<Vec<Task>, ParseError> {
    let query = construct_query_category("Programming_Tasks");
    let json = query_api(query)?;
    parse_all_tasks(&json)
}
 
fn query_a_task(task: &Task) -> Result<String, ParseError> {
    let query = construct_query_task_content(&task.page_id.to_string());
    let json = query_api(query)?;
    get_task(&json, task.page_id)
}


pub fn run(dir: &str) -> Result<(), ParseError> {
    let all_tasks = query_all_tasks()?;
    for task in &all_tasks {
        let content = query_a_task(task)?;

        let mut path = dir.to_owned(); 
        path.push_str("/");
        path.push_str(&task.title);

        DirBuilder::new()
            .recursive(true)
            .create(&path)?;

        let mut file = (File::create(path + "/task"))?;
        file.write_all(content.as_bytes())?;
    }
    Ok(())
}
