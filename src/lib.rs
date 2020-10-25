#[macro_use]
extern crate serde_derive;

use crate::error::RosettaError;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::process::Command;
use std::str;

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

#[derive(Deserialize, Debug)]
struct PageDetail {
    pageid: u64,
    title: String,
}

#[derive(Deserialize, Debug)]
struct RevisionDetail<'a> {
    content: &'a str,
    revid: u64,
    timestamp: String,
    user: String,
    comment: String,
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

impl ContinuedQuery for Revisions {
    fn concat(self: &mut Self, other: Self) {
        self.recentchanges.extend(other.recentchanges)
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize, Debug)]
struct WrittenTask {
    pageid: u64,
    revid: u64,
}

impl WrittenTask {
    fn new(pageid: u64, revid: u64) -> Self {
        Self { pageid, revid }
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

fn to_string_pair(s: &(&str, &str)) -> (String, String) {
    (s.0.to_string(), s.1.to_string())
}

fn make_category_query_args(cname: &str) -> Vec<(String, String)> {
    [
        ("action", "query"),
        ("format", "json"),
        ("formatversion", "2"),
        ("list", "categorymembers"),
        ("cmlimit", "200"),
        ("cmtitle", &("Category:".to_owned() + cname)),
    ]
    .iter()
    .map(to_string_pair)
    .collect()
}

fn make_recentchanges_query_args(rcstart: Option<String>) -> Vec<(String, String)> {
    let mut st_args = rcstart.map_or(vec![], |st| {
        vec![
            ("rcstart".to_string(), st),
            ("rcdir".to_string(), "newer".to_string()),
        ]
    });

    let mut args = vec![
        ("action", "query"),
        ("format", "json"),
        ("formatversion", "2"),
        ("list", "recentchanges"),
        ("rcprop", "title|ids|timestamp"),
        ("rclimit", "500"),
    ]
    .iter()
    .map(to_string_pair)
    .collect::<Vec<(String, String)>>();

    args.append(&mut st_args);
    args
}

fn make_task_query_args(task: &Task) -> Vec<(String, String)> {
    [
        ("action", "query"),
        ("format", "json"),
        ("formatversion", "2"),
        ("prop", "revisions"),
        ("rvprop", "content|ids|timestamp|user|comment"),
        ("pageids", &task.pageid.to_string()),
        ("continue", ""),
    ]
    .iter()
    .map(to_string_pair)
    .collect()
}

fn make_revision_query_args(revision: &Revision) -> Vec<(String, String)> {
    [
        ("action", "query"),
        ("format", "json"),
        ("formatversion", "2"),
        ("prop", "revisions"),
        ("rvprop", "content|ids|timestamp|user|comment"),
        ("pageids", &revision.pageid.to_string()),
        ("rvstartid", &revision.revid.to_string()),
        ("rvlimit", "1"),
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

        let v: Value = serde_json::from_str(&"xxx")?;
        let query_value: &Value = &v["query"];    
        let partial: T = T::deserialize(query_value.clone())?; // why the clone? // Answer(?):  because deserialize takes ownership of its argument.
        return Ok(partial);

}

fn tq<'a, T: Deserialize<'a> >() -> () {

        let v: Value = serde_json::from_str("xxx").unwrap();
        let query_value: &Value = &v["query"];   
        println!("{}", query_value); 
        // ()
        // T::deserialize(query_value).unwrap()

}

fn write_task_response(
    lan: &languages::Langs,
    directory: &str,
    response: &str,
) -> Result<(WrittenTask, String, String, String, String), Box<dyn Error>> {
    let v: &Value = &serde_json::from_str(response)?;
    let p0 = &v["query"]["pages"][0];

    let pd = PageDetail::deserialize(p0)?;
    let rd = RevisionDetail::deserialize(&p0["revisions"][0])?;

    write_code_onig::write_code(lan, directory, &pd.title, rd.content)?;
    Ok((
        WrittenTask::new(pd.pageid, rd.revid),
        rd.timestamp,
        rd.user,
        rd.comment,
        pd.title,
    ))
}

fn write_revision(
    lan: &languages::Langs,
    directory: &str,
    revision: &Revision,
) -> Result<(WrittenTask, String, String, String, String), Box<dyn Error>> {
    let response = &query_api(make_revision_query_args(revision))?;
    write_task_response(lan, directory, response)
}

fn write_task(
    lan: &languages::Langs,
    directory: &str,
    task: &Task,
) -> Result<WrittenTask, Box<dyn Error>> {
    let response = &query_api(make_task_query_args(task))?;
    let (wt, _, _, _, _) = write_task_response(lan, directory, response)?;
    Ok(wt)
}

fn write_tasks(tasks: &Tasks, lan: &languages::Langs, directory: &str) -> HashSet<WrittenTask> {
    // flat_map trick ref : https://stackoverflow.com/a/28572170/509928
    tasks
        .categorymembers
        .iter()
        .flat_map(|task| write_task(lan, directory, task))
        .collect()
}

fn write_task_tally(
    written_tasks: &HashSet<WrittenTask>,
    directory: &str,
) -> Result<(), Box<dyn Error>> {
    let f = File::create(directory.to_string() + "/tasks")?;
    let mut b = BufWriter::new(f);
    let s = serde_json::to_string(&written_tasks)?;
    b.write_all(s.as_bytes())?;
    Ok(())
}

fn read_task_tally(directory: &str) -> Result<HashSet<WrittenTask>, Box<dyn Error>> {
    let f = File::open(directory.to_string() + "/tasks")?;
    let mut b = BufReader::new(f);
    let mut s = String::new();
    b.read_to_string(&mut s)?;
    let v: Value = serde_json::from_str(&s)?;
    let revi = <HashSet<WrittenTask>>::deserialize(v)?;

    Ok(revi)
}

fn initialize_tasks(
    lan: &languages::Langs,
    category_name: &str,
) -> Result<(), Box<dyn Error>> {
    let tasks: Tasks = query(make_category_query_args(category_name))?;
    let written_tasks = write_tasks(&tasks, &lan, category_name);
    write_task_tally(&written_tasks, category_name)?;
    init_repo(category_name)?;
    Ok(())
}

fn diff_names(directory: &str) -> Result<String, Box<dyn Error>> {
    Command::new("git").arg("add").arg(directory).output()?;

    let output = Command::new("git")
        .arg("diff")
        .arg("--name-only")
        .arg("--cached")
        .arg(directory)
        .output()?;

    let ostr = str::from_utf8(&output.stdout)?;
    Ok(ostr.to_string())
}

fn commit_changes(comment: &str) -> Result<(), Box<dyn Error>> {
    Command::new("git").arg("add").arg(".").output()?;
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(comment)
        .output()?;
    Ok(())
}

fn init_repo(directory: &str) -> Result<(), Box<dyn Error>> {
    Command::new("git").arg("init").output()?;
    commit_changes(&(directory.to_string() + ": initial commit"))
}

fn read_revision_timestamp() -> Result<String, Box<dyn Error>> {
    let f = File::open("revision_timestamp")?;
    let mut b = BufReader::new(f);
    let mut s = String::new();
    b.read_to_string(&mut s)?;
    let v: Value = serde_json::from_str(&s)?;
    let ts = <String>::deserialize(v)?;

    Ok(ts)
}

fn save_revision_timestamp(ts: &str) -> Result<(), Box<dyn Error>> {
    let f = File::create("revision_timestamp")?;
    let mut b = BufWriter::new(f);
    let s = serde_json::to_string(ts)?;
    b.write_all(s.as_bytes())?;
    Ok(())
}

fn process_revision(
    lan: &languages::Langs,
    directory: &str,
    revision: &Revision,
    task_set: &mut HashSet<WrittenTask>,
) -> Result<(), Box<dyn Error>> {
    let current_task = WrittenTask::new(revision.pageid, revision.revid);
    let old_task = WrittenTask::new(revision.pageid, revision.old_revid);
    if task_set.contains(&old_task) && !task_set.contains(&current_task) {
        let (written_task, timestamp, user, comment, title) =
            write_revision(&lan, directory, revision)?;
        task_set.remove(&old_task);
        task_set.insert(written_task);
        let modified = diff_names(directory)?;
        if modified != "" {
            let comment_arg = format!(
                "task: {}\nuser: {}\ncomment: {}\ntimestamp: {}\nmodified: {}\n",
                title, user, comment, timestamp, modified
            );
            write_task_tally(task_set, directory)?;
            save_revision_timestamp(&revision.timestamp)?;
            commit_changes(&comment_arg)?;
        }
    }
    Ok(())
}

fn update_new_tasks(
    lan: &languages::Langs,
    directory: &str,
    tasks: &HashSet<WrittenTask>,
    rc: &[Revision],
) -> Result<(), Box<dyn Error>> {
    let mut task_set: HashSet<WrittenTask> = tasks.clone();

    let _u = rc
        .iter()
        .flat_map(|revision| process_revision(lan, directory, revision, &mut task_set))
        .collect::<Vec<_>>();
    Ok(())
}

fn update_tasks<'a>(
    lan: &languages::Langs,
    category_name: &'a str,
    rc: &[Revision],
) -> Result<Option<&'a str>, Box<dyn Error>> {
    match read_task_tally(category_name) {
        Ok(tasks) => { 
            update_new_tasks(lan, category_name, &tasks, &rc)?; 
            Ok(None)
        },
        _ => Ok(Some(category_name))
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    // first gather up all languages by name
    //let category_name = "Programming_Languages";
    let languages_query_args = make_category_query_args("Programming_Languages");
    let languages = query(languages_query_args)?;
    let lan = &languages::Langs::new(&languages)?;

    // get previous timestamp (if it exists).
    let timestamp = read_revision_timestamp().ok();
    let revisions: Revisions = query(make_recentchanges_query_args(timestamp))?;
    let mut rc = revisions.recentchanges;
    rc.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    let categories = [
        //"Programming_Tasks",
        "Draft_Programming_Tasks",
        "Simple",
    ];

    // do updates first so that timestamp gets set before reading tasks for new category.
    
    let new_categories = categories
                             .iter()
                             .map(|category|
                                  update_tasks(lan, category, &rc))
                             .collect::<Result<Vec<_>,_>>()?;

    let _unused = new_categories
                             .iter()
                             .map(|category| {
                                  match category {
                                      Some(cat) => initialize_tasks(lan, cat),
                                      _ => Ok(()),
                                  }
                             })
                             .collect::<Result<Vec<_>,_>>()?;

    Ok(())
}
