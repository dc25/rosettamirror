extern crate reqwest;
extern crate url;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::str;
use std::io::prelude::*;
use std::error::Error;
use serde_json::{Value};
use serde::Deserialize;

use crate::error::RosettaError;

mod write_code_onig;
mod error;

#[derive(Serialize, Deserialize, Debug)]
struct TaskData {
    pageid: u64,
    title: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Query {
    categorymembers: Vec<TaskData>
}

fn query_api(url: url::Url) -> Result<String, Box<dyn Error>> {
    let mut response = (reqwest::get(url.as_str()))?;
    let mut body = String::new();
    response.read_to_string(&mut body)?;
 
    Ok(body)
}
 
fn query_tasks(cont_args: &Vec<(String, String)>) -> Result<String, Box<dyn Error>> {
    let mut query = url::Url::parse("http://rosettacode.org/mw/api.php")?;

    let query_pairs 
        = vec![ ("action", "query")
              , ("format", "json")
              , ("formatversion", "2")
              , ("list", "categorymembers")
              , ("cmlimit", "200")
              , ("cmtitle", "Category:Programming_Tasks")
              ];

    query.query_pairs_mut().extend_pairs(query_pairs.into_iter());
    query.query_pairs_mut().extend_pairs(cont_args.into_iter());
    let json = query_api(query)?;
    Ok(json)
}
 
fn query_a_task(task: &TaskData) -> Result<String, Box<dyn Error>> {
    let mut query = url::Url::parse("http://rosettacode.org/mw/api.php")?;

    let tp = &task.pageid.to_string();

    let query_pairs 
        = vec![ ("action", "query")
              , ("format", "json")
              , ("formatversion", "2")
              , ("prop", "revisions")
              , ("rvprop", "content")
              , ("pageids", tp)
              ];

    query.query_pairs_mut().extend_pairs(query_pairs.into_iter());
    let json = query_api(query)?;
    Ok(json)
}

fn query_all_tasks() -> Result<Vec<TaskData>, Box<dyn Error>> {

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
                                  .ok_or(RosettaError::UnexpectedFormat)?;

            let to_cont_pair = |ca: (&String, &Value)| { 
                let cp1 = ca.1.as_str().ok_or(RosettaError::UnexpectedFormat)?;
                Ok((ca.0.clone(), cp1.to_owned()))
            };

            let cont_args_result : Result<Vec<(String,String)>, Box<dyn Error>>
                    = cont_object 
                        .iter().map(to_cont_pair).collect();

            cont_args = cont_args_result?;
        } else {
            return Ok(all_tasks);
        }
    }
}

pub fn run(dir: &str) -> Result<(), Box<dyn Error>> {
    let all_tasks = query_all_tasks()?;

    for task in all_tasks.iter() {
        let content = &query_a_task(task)?;
        let v: Value = serde_json::from_str(content)?;
        let code = &v["query"]["pages"][0]["revisions"][0]["content"];
        let slc = code.as_str().ok_or(RosettaError::UnexpectedFormat)?;

        let path = dir.to_owned() 
                       + "/" 
                       + &str::replace(&task.title, " ", "-");

        write_code_onig::write_code(&path, slc)?;
    }
    Ok(())
}
