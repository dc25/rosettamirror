extern crate reqwest;
extern crate url;
extern crate rustc_serialize;

use std::fs::*;
use std::io::prelude::*;

use rosettamirror;
 
fn main() {
    let all_tasks = rosettamirror::query_all_tasks();
    for task in &all_tasks {
        let content = rosettamirror::query_a_task(task);

        let path = "mirror/".to_owned() + &task.title;

        DirBuilder::new()
            .recursive(true)
            .create(&path).unwrap();

        let mut file = (File::create(path + "/task")).unwrap();
        file.write_all(content.as_bytes());
    }
}
