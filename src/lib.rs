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
use std::io::{BufWriter, Read, Write};
// use std::io::{BufReader, BufWriter, Read, Write};
// use std::io::Read;
use std::str;

use crate::error::RosettaError;

mod error;
mod languages;
mod write_code_onig;

pub trait ContinuedQuery {
    fn concat(self: &mut Self, other: Self);
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

impl Revisions {
    fn latest(self: &Self) -> Result<String, Box<dyn Error>> {
        self.recentchanges
            .iter()
            .max_by(|x, y| x.timestamp.cmp(&y.timestamp))
            .map(|r| r.timestamp.clone())
            .ok_or(Box::new(RosettaError::UnexpectedFormat))
    }
}

impl ContinuedQuery for Revisions {
    fn concat(self: &mut Self, other: Self) {
        self.recentchanges.extend(other.recentchanges)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct WrittenTask {
    pageid: u64,
    revid: u64,
}

fn query_api(args: Vec<(String, String)>) -> Result<String, Box<dyn Error>> {
    let mut query = url::Url::parse("http://rosettacode.org/mw/api.php")?;

    query.query_pairs_mut().extend_pairs(args.into_iter());
    let mut response = (reqwest::get(query.as_str()))?;
    let mut body = String::new();
    response.read_to_string(&mut body)?;
    Ok(body)
}

fn to_string_pair(s: &(&str, &str)) -> (String, String) {
    (s.0.to_string(), s.1.to_string())
}

fn make_category_query_args(cname: &str, latest: &str) -> Vec<(String, String)> {
    [
        ("action", "query"),
        ("format", "json"),
        ("formatversion", "2"),
        ("list", "categorymembers"),
        ("cmlimit", "200"),
        ("cmtitle", &("Category:".to_owned() + cname)),
        ("cmsort", "timestamp"),
        ("cmend", latest),
    ]
    .iter()
    .map(to_string_pair)
    .collect()
}

fn make_recentchanges_query_args() -> Vec<(String, String)> {
    [
        ("action", "query"),
        ("format", "json"),
        ("formatversion", "2"),
        ("list", "recentchanges"),
        ("rcprop", "title|ids|timestamp"),
        ("rclimit", "200"),
    ]
    .iter()
    .map(to_string_pair)
    .collect()
}

fn make_task_query_args(task: &Task) -> Vec<(String, String)> {
    [
        ("action", "query"),
        ("format", "json"),
        ("formatversion", "2"),
        ("prop", "revisions"),
        ("rvprop", "content|ids"),
        ("pageids", &task.pageid.to_string()),
        ("continue", ""),
    ]
    .iter()
    .map(to_string_pair)
    .collect()
}

fn to_continue_pair(ca: (&String, &Value)) -> Result<(String, String), Box<dyn Error>> {
    let cp1 = ca.1.as_str().ok_or(RosettaError::UnexpectedFormat)?;
    Ok((ca.0.clone(), cp1.to_owned()))
}

fn query<'a, T: Deserialize<'a> + Default + ContinuedQuery>(
    query_args: Vec<(String, String)>,
) -> Result<T, Box<dyn Error>> {
    let mut complete: T = Default::default();

    let mut cont_args = vec![("continue".to_owned(), "".to_owned())];

    loop {
        let mut ac = query_args.clone();
        ac.extend(cont_args);
        let s = query_api(ac)?;
        let v: Value = serde_json::from_str(&s)?;
        let qv = &v["query"];
        let partial = T::deserialize(qv.clone())?; // why the clone?
        complete.concat(partial);

        let cv = &v["continue"];
        if cv.is_object() {
            cont_args = cv
                .as_object()
                .ok_or(RosettaError::UnexpectedFormat)?
                .iter()
                .map(to_continue_pair)
                .collect::<Result<Vec<_>, _>>()?;
        } else {
            return Ok(complete);
        }
    }
}

fn write_task(lan: &languages::Langs, directory: &str, task: &Task) -> Result<WrittenTask, Box<dyn Error>> {
    let content = &query_api(make_task_query_args(task))?;
    let v: Value = serde_json::from_str(content)?;

    let code = &v["query"]["pages"][0]["revisions"][0]["content"];
    let slc = code.as_str().ok_or(RosettaError::UnexpectedFormat)?;

    let revid = &v["query"]["pages"][0]["revisions"][0]["revid"];
    let rid = revid.as_u64().ok_or(RosettaError::UnexpectedFormat)?;

    write_code_onig::write_code(&lan, directory, &task.title, slc)?;
    Ok(WrittenTask{pageid:task.pageid, revid:rid})
}

pub fn run(directory: &str, _all: bool) -> Result<(), Box<dyn Error>> {
    let revisions: Revisions = query(make_recentchanges_query_args())?;

    let latest_timestamp = revisions.latest()?;

    let tasks: Tasks = query(make_category_query_args(
        "Programming_Tasks",
        &latest_timestamp,
    ))?;

    let languages: Languages = query(make_category_query_args(
        "Programming_Languages",
        &latest_timestamp,
    ))?;

    let lan = languages::Langs::new(&languages)?;

    // flat_map trick ref : https://stackoverflow.com/a/28572170/509928
    let written_tasks: Vec<_> = tasks
        .categorymembers
        .iter()
        .flat_map(|task| write_task(&lan, directory, task))
        .collect();

    let tfo = File::create("tasks")?;
    let mut tbo = BufWriter::new(tfo);
    let tso : String = serde_json::to_string(&written_tasks)?;
    tbo.write_all(tso.as_bytes())?;

    /*
    let rfi = File::open("revisions")?;
    let mut rbi = BufReader::new(rfi);
    let mut rsi = String::new();
    rbi.read_to_string(&mut rsi)?;
    let v: Value = serde_json::from_str(&rsi)?;
    let _revi = Revisions::deserialize(v)?;
    */


    Ok(())
}
