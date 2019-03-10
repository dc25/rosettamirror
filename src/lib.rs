extern crate reqwest;
extern crate url;

use std::fs;
use std::iter::*;
use std::io::prelude::*;
use serde_json::{Value, Map};
use serde::Deserialize;

#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct TaskData {
    pageid: u64,
    title: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Query {
    categorymembers: Vec<TaskData>
}

pub enum Error {
    /// Something went wrong with the HTTP request to the API.
    Http(reqwest::Error),
 
    /// There was a problem parsing the API response into JSON.
    Io(std::io::Error),
 
    /// There was a problem parsing the API response into JSON.
    ParseUrl(url::ParseError),
 
    /// There was a problem parsing the API response into JSON.
    SerdeJson(serde_json::Error),
 
    /// Unexpected JSON format from response
    UnexpectedFormat,
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
 
impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::SerdeJson(error)
    }
}
 
fn query_api(url: url::Url) -> Result<String, Error> {
    let mut response = (reqwest::get(url.as_str()))?;
    let mut body = String::new();
    response.read_to_string(&mut body)?;
 
    Ok(body)
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
              , ("continue", &cont)
              ];

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
 
fn query_all_tasks() -> Result<String, Error> {
    let query = construct_query_category("Programming_Tasks", "")?;
    let json = query_api(query)?;
    Ok(json)
}
 
fn query_a_task(task: &TaskData) -> Result<String, Error> {
    let query = construct_query_task_content(&task.pageid.to_string())?;
    let json = query_api(query)?;
    Ok(json)
}


pub fn run(dir: &str) -> Result<(), Error> {

    let mut cont_args : Vec<(String, String)> = vec![("continue".to_owned(), "".to_owned())];
    let all_tasks = query_all_tasks()?;
    let all_tasks_value: Value = serde_json::from_str(&all_tasks)?;
    let query_value = &all_tasks_value["query"];
    let cont_value = &all_tasks_value["continue"];
    if cont_value.is_object() {

        let cont_object = cont_value.as_object()
                                .ok_or(Error::UnexpectedFormat)?;

        let to_cont_pair = |ca: (&String, &Value)| { 
            let cp1 = ca.1.as_str().ok_or(Error::UnexpectedFormat)?;
            Ok((ca.0.clone(), cp1.to_owned()))
        };

        let cont_args_result : Result<Vec<(String,String)>, Error> 
                = cont_object 
                    .iter().map(to_cont_pair).collect();

        cont_args = cont_args_result?;
    } else {
        return Ok(());
    }
    let query:Query = Query::deserialize(query_value)?;
    for task in &query.categorymembers {
        let content = &query_a_task(task)?;
        let v: Value = serde_json::from_str(content)?;

    }
    Ok(())
}

/*

pub fn run(dir: &str) -> Result<(), Error> {
    let all_tasks = query_all_tasks()?;
    let all_tasks_value: Value = serde_json::from_str(&all_tasks)?;
    let query_value = &all_tasks_value["query"];
    let cont_value = &all_tasks_value["continue"].as_object().unwrap_or(&serde_json::Map::new());
    let cont_pairs = cont_value.iter().
    let query = Query::deserialize(query_value)?;
    for task in &query.categorymembers {
        let content = &query_a_task(task)?;
        let v: Value = serde_json::from_str(content)?;
        let code = &v["query"]["pages"][task.pageid.to_string()]["revisions"][0]["*"];

        let mut path = dir.to_owned(); 
        path.push_str("/");
        let task_dir = str::replace(&task.title, " ", "_");
        path.push_str(&task_dir);

        fs::DirBuilder::new().recursive(true).create(&path)?;

        let mut file = (fs::File::create(path + "/task"))?;
        file.write_all(serde_json::to_string(&code).unwrap().as_bytes())?;
    }
    Ok(())
}
*/
