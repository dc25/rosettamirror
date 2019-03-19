use std::fs;
use std::error::Error;
use crate::RosettaError;
use onig::Regex;
use crate::extensions::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use unicode_normalization::*;
use unicode_categories::*;

fn strip_accents(s: String) -> String {
    // based on suggestions at ...
    // https://www.reddit.com/r/rust/comments/73vr1u/is_there_a_way_to_remove_accentspunctuation_from/
    s.chars().nfd().filter(|s| !s.is_mark_nonspacing()).collect()
}

fn expand_ligatures(s: String) -> Result<String, Box<dyn Error>> 
{
    let re = Regex::new(r"\u00E6")?; // got unicode here: https://en.wiktionary.org/wiki/%C3%A6#Translingual
    let s_out = re.replace_all(&s, "ae"); 
    Ok(s_out)
}

fn to_filename(name: &str) -> Result <String, Box<dyn Error>> 
{
    let stripped_name = strip_accents(name.to_owned());
    let expanded_ligatures_name = expand_ligatures(stripped_name)?;
    let s:String = expanded_ligatures_name.chars()
        .map(|x| match x { 
                    ' ' => '-', 
                    '/' => '-', 
                    '_' => '-', 
                    '(' => '-', 
                    ')' => '-', 
                    '\u{00e9}' => 'e', 
                    _ => x}
            )
        .filter(|x| match x { 
                    ',' => false,
                    '\'' => false,
                    _ => true}
               )
        .collect();

    // some clumsy artifacts
    let sc = if s == "F-Sharp|F#" {
                         "f-sharp".to_owned()
                     } else
                     {
                         s
                     };
    Ok(sc)
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
        let extension = get_extension(&lang_file_name)?;

        let program_dir = dir.to_owned() 
                           + "/" 
                           + &task_file_name
                           + "/" 
                           + &lang_file_name;

        fs::DirBuilder::new().recursive(true).create(&program_dir)?;

        let program_matches = header_match.at(2).ok_or(RosettaError::UnexpectedFormat)?;

        let programs_opt : Option<Vec<_>> 
                = program_re.captures_iter(program_matches) 
                            .map(|pm| pm.at(1))
                            .collect();
                                                  
        let programs: Vec<_> 
                = programs_opt.ok_or(RosettaError::UnexpectedFormat)?;

        let mut index: u32 = 1;
        for program in programs.iter() {
            let qualifier = 
                if programs.len() == 1 {
                    "".to_owned()
                } else {
                    "-".to_owned() + &index.to_string()
                };

            index = index+1;

            let program_name =   program_dir.clone()
                               + "/" 
                               + &task_file_name 
                               + &qualifier
                               + "." 
                               + &extension;

            let f = File::create(&program_name)?;
            let mut f = BufWriter::new(f);
            f.write_all(program.as_bytes())?;
        }
    }
    Ok(())
}

