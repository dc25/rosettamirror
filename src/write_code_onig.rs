use std::fs;
use std::error::Error;
use onig::Regex;
use crate::RosettaError;
use crate::extensions::*;
use std::fs::File;
use std::io::{BufWriter, Write};

fn to_filename(name: &str) -> Result <String, Box<dyn Error>> 
{
    let s:String = name.chars()
        .map(|x| match x { 
                    ' ' => '-', 
                    '/' => '-', 
                    '_' => '-', 
                    '(' => '-', 
                    ')' => '-', 
                    _ => x}
            )
        .filter(|x| match x { 
                    ',' => false,
                    '\'' => false,
                    _ => true}
               )
        .collect();
    Ok(s)
}


pub fn write_code(dir: &str, task_name: &str, code: &str) -> Result<(), Box<dyn Error>>
{
    println!("TASK: {}", task_name);

    let re = Regex::new(r"(?m)\<nowiki\>.*?\</nowiki\>")?;
    let code = re.replace_all(code, "REMOVEDNOWIKI");

    let header_re = Regex::new(r"(?m)^===*\{\{[Hh]eader\|(.*?)\}\}(.*?)(?:\z|(?=^===*\{\{[Hh]eader))")?;
    let program_re = Regex::new(r"(?mi)<lang *(?: [^>]+)?>(.*?)<\/lang *>")?;

    for header_match in header_re.captures_iter(&code) {
        let lang = header_match.at(1).ok_or(RosettaError::UnexpectedFormat)?;

        let task_file_name = to_filename(task_name)?;
        let lang_file_name = to_filename(lang)?;
        let extension = get_extension(lang)?;

        let program_dir = dir.to_owned() 
                           + "/" 
                           + &task_file_name
                           + "/" 
                           + &lang_file_name;

        fs::DirBuilder::new().recursive(true).create(&program_dir)?;


        let program_code = header_match.at(2).ok_or(RosettaError::UnexpectedFormat)?;

        let programs_it : onig::FindCaptures = program_re.captures_iter(program_code) ;
        // let programs_it2 : std::iter::Map<onig::FindCaptures, Option<&str>> = programs_it
        //                                           .map(|pm:std::ops::FnMut<(onig::find::Captures<'_>,)>| pm.at(1));
        let programs_it2:Option<Vec<_>> = programs_it
                                                  .map(|pm| pm.at(1)).collect();
                                                  

        let programs_it3: Result<Vec<_>,_> = programs_it2.ok_or(RosettaError::UnexpectedFormat);
        let programs_it4 = programs_it3?;
        // let programs:Result<Vec<&str>, Box<dyn Error>> = programs_it.collect()?;
        for program in programs_it4.iter() {
            let program_name =   program_dir.clone()
                               + "/" 
                               + &task_file_name 
                               + "." 
                               + &extension;

			let f = File::create(&program_name)?;
			let mut f = BufWriter::new(f);
			f.write_all(program.as_bytes())?;
        }
    }
    Ok(())
}

