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
mod languages;

// This post was helpful for getting impl Iterator argument working.
// https://stackoverflow.com/a/34745885/509928

pub trait CategoryQuery {
    fn new() -> Self;
    fn extend(self: &mut Self, other: Self);
    fn partial_query(cont_args: impl Iterator<Item = (String,String)>) -> Result<String, Box<dyn Error>>;
}

#[derive(Deserialize, Debug)]
struct TaskData {
    pageid: u64,
    title: String,
}

#[derive(Deserialize, Debug)]
struct TaskQuery {
    categorymembers: Vec<TaskData>
}

impl CategoryQuery for TaskQuery {

        fn new() -> TaskQuery {
            let categorymembers = Vec::new();
            TaskQuery{categorymembers}
        }

        fn extend(self: & mut TaskQuery, other: TaskQuery) {
            self.categorymembers.extend(other.categorymembers)
        }

        fn partial_query(cont_args: impl Iterator<Item = (String,String)>) -> Result<String, Box<dyn Error>> {
          query_category(&"Programming_Tasks", cont_args)
        }

}

#[derive(Deserialize, Debug)]
pub struct LanguageData {
    title: String
}

#[derive(Deserialize, Debug)]
pub struct LanguageQuery {
    categorymembers: Vec<LanguageData>
}

impl CategoryQuery for LanguageQuery {

        fn new() -> LanguageQuery {
            let categorymembers = Vec::new();
            LanguageQuery{categorymembers}
        }

        fn extend(self: & mut LanguageQuery, other: LanguageQuery) {
            self.categorymembers.extend(other.categorymembers)
        }

        fn partial_query(cont_args: impl Iterator<Item = (String,String)>) -> Result<String, Box<dyn Error>> {
          query_category(&"Programming_Languages", cont_args)
        }
}

fn query_api(url: url::Url) -> Result<String, Box<dyn Error>> {
    let mut response = (reqwest::get(url.as_str()))?;
    let mut body = String::new();
    response.read_to_string(&mut body)?;
 
    Ok(body)
}

/*
http  rosettacode.org/mw/api.php               \
        action==query                          \
        format==json                           \
        list==recentchanges                    \
        'rcprop==title|ids'                    \
        rclimit==450                           \
        continue==
*/

fn query_category(cname: &str, cont_args: impl Iterator<Item = (String,String)>) -> Result<String, Box<dyn Error>> {
    let mut query = url::Url::parse("http://rosettacode.org/mw/api.php")?;

    let query_pairs 
        =     [ ("action", "query")
              , ("format", "json")
              , ("formatversion", "2")
              , ("list", "categorymembers")
              , ("cmlimit", "200")
              , ("cmtitle", &("Category:".to_owned() + cname))
              ];

    query.query_pairs_mut().extend_pairs(query_pairs.into_iter());
    query.query_pairs_mut().extend_pairs(cont_args);
    let json = query_api(query)?;
    Ok(json)
}
 
fn query_a_task(task: &TaskData) -> Result<String, Box<dyn Error>> {
    let mut query = url::Url::parse("http://rosettacode.org/mw/api.php")?;

    let query_pairs 
        =     [ ("action", "query")
              , ("format", "json")
              , ("formatversion", "2")
              , ("prop", "revisions")
              , ("rvprop", "content")
              , ("pageids", &task.pageid.to_string())
              ];

    query.query_pairs_mut().extend_pairs(query_pairs.into_iter());
    let json = query_api(query)?;
    Ok(json)
}

fn query<'a, T: Deserialize<'a> + CategoryQuery>() -> Result<T, Box<dyn Error>> {

    let mut all_tasks = T::new();

    let mut cont_args = 
                vec![("continue".to_owned(), "".to_owned())];

    loop {
        let tasks_string = T::partial_query(cont_args.into_iter())?;
        let tasks_value: Value = serde_json::from_str(&tasks_string)?;
        let query_value = tasks_value["query"].clone(); // why is this clone() necessary ?
        let query:T = T::deserialize(query_value)?;
        all_tasks.extend(query);

        let cont_value = &tasks_value["continue"];
        if cont_value.is_object() {

            let to_cont_pair = |ca: (&String, &Value)| -> Result<_, Box<dyn Error>> { 
                let cp1 = ca.1.as_str().ok_or(RosettaError::UnexpectedFormat)?;
                Ok((ca.0.clone(), cp1.to_owned()))
            };

            cont_args = cont_value 
                            .as_object()
                            .ok_or(RosettaError::UnexpectedFormat)?
                            .iter()
                            .map(to_cont_pair)
                            .collect::<Result<Vec<_>, _>>()?;

        } else {
            return Ok(all_tasks);
        }
    }
}

pub fn run(dir: &str) -> Result<(), Box<dyn Error>> {
    let all_tasks : TaskQuery= query()?;
    let all_languages : LanguageQuery = query()?;

    let lan = languages::Languages::new(&all_languages)?;

    for task in all_tasks.categorymembers.iter() {
        let content = &query_a_task(task)?;
        let v: Value = serde_json::from_str(content)?;
        let code = &v["query"]["pages"][0]["revisions"][0]["content"];
        let slc = code.as_str().ok_or(RosettaError::UnexpectedFormat)?;

        write_code_onig::write_code(&lan, dir, &task.title, slc)?;
    }
    Ok(())
}
