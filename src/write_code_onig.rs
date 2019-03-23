use std::fs;
use std::error::Error;
use crate::RosettaError;
use onig::Regex;
use std::fs::File;
use std::io::{BufWriter, Write};
use unicode_normalization::*;
use unicode_categories::*;
use crate::extensions::*;
use crate::languages::*;
use maplit::hashmap;

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

fn trim_extra(s: String) -> Result<String, Box<dyn Error>> 
{
    let s0 = Regex::new(r"^\s+")?.replace_all(&s, ""); 
    Ok(Regex::new(r"(\||,?\s+)$")?.replace_all(&s0, ""))
}

fn lang_to_filename(lan: &Languages, name: &str) -> Result <String, Box<dyn Error>> 
{
    let stripped_name = strip_accents(name.to_owned());
    let expanded_ligatures_name = expand_ligatures(stripped_name)?;
    let trimmed_extra = trim_extra(expanded_ligatures_name)?;
    let looked_up = lan.lookup(trimmed_extra);
    let s:String = looked_up.chars()
        .map(|x| match x { 
                    ' ' => '-', 
                    '/' => '-', 
                    '_' => '-', 
                    '(' => '-', 
                    ')' => '-', 
                    '*' => '-', 
                    // '\u{00e9}' => 'e', 
                    _ => x}
            )
        .filter(|x| match x { 
                    // ',' => false,
                    '\'' => false,
                    '"' => false,
                    _ => true}
               )
        .collect();

    let special_cases = hashmap![
            "Basic|QuickBasic" => "Basic-or-QuickBasic",
            "C++|CPP" => "C++",
            "C#|C-sharp" => "c-sharp",
            "C#|CSharp" => "c-sharp",
            "clojure|Clojure" => "Clojure",
            "Clojure|Clojure" => "Clojure",
            "Clojure|ClojureScript" => "Clojure-or-ClojureScript",
            "c-sharp|C#" => "C-sharp",
            "C-sharp|C#" => "C-sharp",
            "Dylan.NET|Dylan.NET" => "Dylan.NET",
            "F#|F-sharp" => "F-sharp",
            "F-sharp|F#" => "F-sharp",
            "F-Sharp|F#" => "F-sharp",
            "Pascal|FreePascal" => "Pascal-or-FreePascal",
            "PostScript|Post-Script" => "PostScript",
            "Python|Python-3" => "Python-or-Python-3",
    ];

    let str_lang : &str = &s;

    let ext : Option<String> = special_cases.get(str_lang)
                             .map(|&st| st.to_owned());
    match ext {
        None => Ok(s),
        Some(found) => Ok(found),
    }
}



fn task_to_filename(name: &str) -> Result <String, Box<dyn Error>> 
{
    // let stripped_name = strip_accents(name.to_owned());
    let expanded_ligatures_name = expand_ligatures(name.to_owned())?;
    let s:String = expanded_ligatures_name.chars()
        .map(|x| match x { 
                    ' ' => '-', 
                    '/' => '-', 
                    '_' => '-', 
                    '(' => '-', 
                    ')' => '-', 
                    '*' => '-', 
                    '!' => '-', 
                    '–' => '-', 
                    '\u{00e9}' => '-', // e with acute accent
                    '\u{00e8}' => '-', // e with grave accent
                    _ => x}
            )
        .filter(|x| match x { 
                    // ',' => false,
                    '\'' => false,
                    '"' => false,
                    _ => true}
               )
        .collect();

    Ok(s)
}


pub fn write_code(lan: &Languages, dir: &str, task_name: &str, code: &str) -> Result<(), Box<dyn Error>>
{
    println!("TASK: {}", task_name);

    let header_re = Regex::new(r"(?m)^===*\{\{[Hh]eader\|(.*?)\}\}(.*?)(?:\z|(?=^===*\{\{[Hh]eader))")?;
    let program_re = Regex::new(r"(?mi)<lang *(?: [^>]+)?>(.*?)<\/lang *>")?;

    for header_match in header_re.captures_iter(&code) {
        let lang = header_match.at(1).ok_or(RosettaError::UnexpectedFormat)?;

        let task_file_name = task_to_filename(task_name)?;
        let lang_file_name = lang_to_filename(lan, lang)?;
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
                               + &task_file_name.to_lowercase()
                               + &qualifier
                               + "." 
                               + &extension;

            let f = File::create(&program_name)?;
            let mut f = BufWriter::new(f);

            // Decided not to remove trailing (or any other) spaces.
            // Who am I to say that trailing spaces are not 
            // relevant to the meaning of a program?
            
            // let trailing_spaces_re = Regex::new(r"(?m) +$")?; 
            // let no_trailing_program = trailing_spaces_re.replace_all(program, "");
            f.write_all(program.as_bytes())?;
        }
    }
    Ok(())
}

