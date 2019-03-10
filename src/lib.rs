extern crate reqwest;
extern crate url;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;


use std::fs;
use std::iter::*;
use std::io::prelude::*;
use serde_json::{Value};
use serde::Deserialize;


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
 
fn query_tasks(cont_args: &Vec<(String, String)>) -> Result<String, Error> {
    let mut query = url::Url::parse("http://rosettacode.org/mw/api.php")?;

    let query_pairs 
        = vec![ ("action", "query")
              , ("format", "json")
              , ("list", "categorymembers")
              , ("cmlimit", "200")
              , ("cmtitle", "Category:Programming_Tasks")
              ];

    query.query_pairs_mut().extend_pairs(query_pairs.into_iter());
    query.query_pairs_mut().extend_pairs(cont_args.into_iter());
    let json = query_api(query)?;
    Ok(json)
}
 
fn query_a_task(task: &TaskData) -> Result<String, Error> {
    let mut query = url::Url::parse("http://rosettacode.org/mw/api.php")?;

    let tp = &task.pageid.to_string();

    let query_pairs 
        = vec![ ("action", "query")
              , ("format", "json")
              , ("prop", "revisions")
              , ("rvprop", "content")
              , ("pageids", tp)
              ];

    query.query_pairs_mut().extend_pairs(query_pairs.into_iter());
    let json = query_api(query)?;
    Ok(json)
}



fn query_all_tasks() -> Result<Vec<TaskData>, Error> {

    let mut all_tasks: Vec<TaskData> = vec![];

    let mut cont_args : Vec<(String, String)> 
                = vec![("continue".to_owned(), "".to_owned())];

    loop {
        let tasks_string = query_tasks(&cont_args)?;
        let tasks_value: Value = serde_json::from_str(&tasks_string)?;
        let query_value = &tasks_value["query"];
        let query:Query = Query::deserialize(query_value)?;
        all_tasks.extend(query.categorymembers);

        let cont_value = &tasks_value["continue"];
        if cont_value.is_object() {

            let cont_object = cont_value
                                  .as_object()
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
            return Ok(all_tasks);
        }
    }
}

pub fn run(dir: &str) -> Result<(), Error> {
    let all_tasks = query_all_tasks()?;
    println!("{:?}", all_tasks.len());
    for task in all_tasks.iter() {
        let content = &query_a_task(task)?;
        let v: Value = serde_json::from_str(content)?;
        let code = &v["query"]["pages"][task.pageid.to_string()]["revisions"][0]["*"];

        let mut path = dir.to_owned(); 
        path.push_str("/");
        let task_dir = str::replace(&task.title, " ", "_");
        path.push_str(&task_dir);

        fs::DirBuilder::new().recursive(true).create(&path)?;

        let mut file = (fs::File::create(path + "/task"))?;
        let slc = (&code).as_str().ok_or(Error::UnexpectedFormat)?;
        file.write_all(slc.as_bytes())?;
    }
    Ok(())
}
