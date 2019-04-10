extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;
#[macro_use]
extern crate serde_derive;

use serde::Deserialize;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write, Read};
use std::str;

use crate::error::RosettaError;

mod error;
mod languages;
mod write_code_onig;

// This post was helpful for getting impl Iterator argument working.
// (Stopped using impl Iterator but leaving comment for now)
// https://stackoverflow.com/a/34745885/509928

pub trait ContinuedQuery {
    fn concat(self: &mut Self, other: Self);
    fn partial_query(cont_args: Vec<(String, String)>) -> Result<String, Box<dyn Error>>;
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Task {
    pageid: u64,
    title: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Tasks {
    categorymembers: Vec<Task>,
}

impl ContinuedQuery for Tasks {
    fn concat(self: &mut Tasks, other: Tasks) {
        self.categorymembers.extend(other.categorymembers)
    }

    fn partial_query(cont_args: Vec<(String, String)>) -> Result<String, Box<dyn Error>> {
        query_category(&"Programming_Tasks", cont_args)
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Language {
    title: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Languages {
    categorymembers: Vec<Language>,
}

impl ContinuedQuery for Languages {
    fn concat(self: &mut Languages, other: Languages) {
        self.categorymembers.extend(other.categorymembers)
    }

    fn partial_query(cont_args: Vec<(String, String)>) -> Result<String, Box<dyn Error>> {
        query_category(&"Programming_Languages", cont_args)
    }
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub struct Revision {
    pageid: u64,
    old_revid: u64,
    rcid: u64,
    revid: u64,
    timestamp: String,
    title: String,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub struct Revisions {
    recentchanges: Vec<Revision>,
}

impl ContinuedQuery for Revisions {
    fn concat(self: &mut Self, other: Self) {
        self.recentchanges.extend(other.recentchanges)
    }

    fn partial_query(cont_args: Vec<(String, String)>) -> Result<String, Box<dyn Error>> {
        query_recentchanges(cont_args)
    }
}

fn query_api(args: Vec<(String, String)>) -> Result<String, Box<dyn Error>> {
    let mut query = url::Url::parse("http://rosettacode.org/mw/api.php")?;

    query.query_pairs_mut().extend_pairs(args.into_iter());
    let mut response = (reqwest::get(query.as_str()))?;
    let mut body = String::new();
    response.read_to_string(&mut body)?;
    Ok(body)
}

fn query_category(cname: &str, cont_args: Vec<(String, String)>) -> Result<String, Box<dyn Error>> {
    let cat = "Category:".to_owned() + cname;
    let query_pairs: Vec<(&str, &str)> = vec![
        ("action", "query"),
        ("format", "json"),
        ("formatversion", "2"),
        ("list", "categorymembers"),
        ("cmlimit", "200"),
        ("cmtitle", &cat),
    ];

    let mut query_string_pairs: Vec<_> = query_pairs
        .iter()
        .map(|(s0, s1)| (s0.to_string(), s1.to_string()))
        .collect();

    query_string_pairs.extend(cont_args);

    query_api(query_string_pairs)
}

fn query_recentchanges(cont_args: Vec<(String, String)>) -> Result<String, Box<dyn Error>> {
    let query_pairs: Vec<(&str, &str)> = vec![
        ("action", "query"),
        ("format", "json"),
        ("formatversion", "2"),
        ("list", "recentchanges"),
        ("rcprop", "title|ids|timestamp"),
        ("rclimit", "200"),
    ];

    let mut query_string_pairs: Vec<_> = query_pairs
        .iter()
        .map(|(s0, s1)| (s0.to_string(), s1.to_string()))
        .collect();

    query_string_pairs.extend(cont_args);

    query_api(query_string_pairs)
}

fn query_a_task(task: &Task) -> Result<String, Box<dyn Error>> {
    let pid = task.pageid.to_string();

    let query_pairs = vec![
        ("action", "query"),
        ("format", "json"),
        ("formatversion", "2"),
        ("prop", "revisions"),
        ("rvprop", "content"),
        ("pageids", &pid),
    ];

    let query_string_pairs: Vec<_> = query_pairs
        .iter()
        .map(|(s0, s1)| (s0.to_string(), s1.to_string()))
        .collect();

    query_api(query_string_pairs)
}

fn query<'a, T: Deserialize<'a> + Default + ContinuedQuery>() -> Result<T, Box<dyn Error>> {
    let mut complete: T = Default::default();

    let mut cont_args = vec![("continue".to_owned(), "".to_owned())];

    loop {
        let s = T::partial_query(cont_args)?;
        let v: Value = serde_json::from_str(&s)?;
        let qv = &v["query"];
        let partial = T::deserialize(qv.clone())?; // why the clone?
        complete.concat(partial);

        let cv = &v["continue"];
        if cv.is_object() {
            let to_cont_pair = |ca: (&String, &Value)| -> Result<_, Box<dyn Error>> {
                let cp1 = ca.1.as_str().ok_or(RosettaError::UnexpectedFormat)?;
                Ok((ca.0.clone(), cp1.to_owned()))
            };

            cont_args = cv
                .as_object()
                .ok_or(RosettaError::UnexpectedFormat)?
                .iter()
                .map(to_cont_pair)
                .collect::<Result<Vec<_>, _>>()?;
        } else {
            return Ok(complete);
        }
    }
}

pub fn run(directory: &str, _all: bool) -> Result<(), Box<dyn Error>> {
    let tasks: Tasks = query()?;
    let languages: Languages = query()?;
    let revisions: Revisions = query()?;


    {
        let rfo = File::create("revisions")?;
        let mut rbo = BufWriter::new(rfo);
        let rso : String = serde_json::to_string(&revisions)?;
        rbo.write_all(rso.as_bytes())?;

        let rfi = File::open("revisions")?;
        let mut rbi = BufReader::new(rfi);
        let mut rsi = String::new();
        rbi.read_to_string(&mut rsi)?;
        let v: Value = serde_json::from_str(&rsi)?;
        let revi = Revisions::deserialize(v)?;

    }

    let lan = languages::Langs::new(&languages)?;

    for task in tasks.categorymembers.iter() {
        let content = &query_a_task(task)?;
        let v: Value = serde_json::from_str(content)?;
        let code = &v["query"]["pages"][0]["revisions"][0]["content"];
        let slc = code.as_str().ok_or(RosettaError::UnexpectedFormat)?;

        write_code_onig::write_code(&lan, directory, &task.title, slc)?;
    }
    Ok(())
}
