extern crate reqwest;
extern crate url;
extern crate rustc_serialize;

use std::fs;
use std::io::prelude::*;
use std::collections::*;

use rustc_serialize::json::{self, Json};

struct Task {
    page_id: u64,
    title: String,
}

pub enum Error {
    /// Something went wrong with the HTTP request to the API.
    Http(reqwest::Error),
 
    /// There was a problem parsing the API response into JSON.
    Json(json::ParserError),
 
    /// There was a problem parsing the API response into JSON.
    Io(std::io::Error),
 
    /// There was a problem parsing the API response into JSON.
    ParseUrl(url::ParseError),
 
    /// Unexpected JSON format from response
    UnexpectedFormat,
}
impl From<json::ParserError> for Error {
    fn from(error: json::ParserError) -> Self {
        Error::Json(error)
    }
}
 
impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Http(error)
    }
}
 
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}
 
impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Error::ParseUrl(error)
    }
}
 
fn query_api(url: url::Url) -> Result<Json, Error> {
    let mut response = (reqwest::get(url.as_str()))?;
    // Build JSON
    let mut body = String::new();
    response.read_to_string(&mut body)?;
 
    Ok((Json::from_str(&body))?)
}
 
fn construct_query_category(category: &str, cont: &str) -> Result<url::Url, Error> {
    let mut base_url = url::Url::parse("http://rosettacode.org/mw/api.php")?;
    let cat = format!("Category:{}", category);

    let query_pairs 
        = vec![ ("action", "query")
              , ("format", "json")
              , ("list", "categorymembers")
              , ("cmlimit", "10")
              , ("cmtitle", &cat)
              , ("continue", &cont)];

    base_url.query_pairs_mut().extend_pairs(query_pairs.into_iter());
    Ok(base_url)
}
 
fn construct_query_task_content(task_id: &str) -> Result<url::Url, Error> {
    let mut base_url = url::Url::parse("http://rosettacode.org/mw/api.php")?;
    let query_pairs 
        = vec![ ("action", "query")
              , ("format", "json")
              , ("prop", "revisions")
              , ("rvprop", "content")
              , ("pageids", task_id)
        ];

    base_url.query_pairs_mut().extend_pairs(query_pairs.into_iter());
    Ok(base_url)
}
 
fn parse_continue(reply: &Json) -> Vec<(String, String)> { 

    let json_to_continue = |sj: (&String,&Json)| {
        sj.1.as_string().map(|s| (sj.0.clone(), s.to_owned()))
    };

    reply.find_path(&["continue"])
               .and_then(|cont| cont.as_object())
               .unwrap_or(&BTreeMap::new())
               .iter()
               .filter_map(json_to_continue)
               .collect()
}

fn parse_all_tasks(reply: &Json) -> Result<Vec<Task>, Error> { 

    let json_to_task = |json: &Json| {

        let page_id = (json.find("pageid")
            .and_then(|id| id.as_u64())
            .ok_or(Error::UnexpectedFormat))?;

        let title = (json.find("title")
            .and_then(|title| title.as_string())
            .ok_or(Error::UnexpectedFormat))?;

        Ok(Task {
            page_id: page_id,
            title: title.to_owned(),
        })
    };

    reply.find_path(&["query", "categorymembers"])
               .and_then(|tasks| tasks.as_array())
               .ok_or(Error::UnexpectedFormat)?
               .iter()
               .map(json_to_task)
               .collect()

}

fn get_task(task: &Json, task_id: u64) -> Result<String, Error> {
    let revisions =
        task.find_path(&["query", "pages", task_id.to_string().as_str(), "revisions"])
            .and_then(|content| content.as_array())
            .ok_or(Error::UnexpectedFormat)?;

    let content = (revisions[0]
        .find("*")
        .and_then(|content| content.as_string())
        .ok_or(Error::UnexpectedFormat))?;

    Ok(String::from(content))
}
 
fn query_all_tasks() -> Result<Vec<Task>, Error> {
    let query = construct_query_category("Programming_Tasks", "")?;
    let json = query_api(query)?;
    parse_all_tasks(&json) 
}
 
fn query_a_task(task: &Task) -> Result<String, Error> {
    let query = construct_query_task_content(&task.page_id.to_string())?;
    let json = query_api(query)?;
    get_task(&json, task.page_id)
}


pub fn run(dir: &str) -> Result<(), Error> {
    let all_tasks = query_all_tasks()?;
    for task in &all_tasks {
        let content = query_a_task(task)?;

        let mut path = dir.to_owned(); 
        path.push_str("/");
        let task_dir = str::replace(&task.title, " ", "_");
        path.push_str(&task_dir);

        fs::DirBuilder::new().recursive(true).create(&path)?;

        let mut file = (fs::File::create(path + "/task"))?;
        file.write_all(content.as_bytes())?;
    }
    Ok(())
}
