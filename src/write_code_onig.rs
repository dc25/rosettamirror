use std::fs;
use std::error::Error;
use onig::Regex;
use crate::RosettaError;

pub fn write_code(dir: &String, code: &str) -> Result<(), Box<dyn Error>>
{

    fs::DirBuilder::new().recursive(true).create(&dir)?;

    let re = Regex::new(r"(?m)\<nowiki\>.*?\</nowiki\>")?;
    let code = re.replace_all(code, "REMOVEDNOWIKI");

    let header_re = Regex::new(r"(?m)^===*\{\{[Hh]eader\|(.*?)\}\}(.*?)(?:\z|(?=^===*\{\{[Hh]eader))")?;
    let program_re = Regex::new(r"(?mi)<lang *(?: [^>]+)?>(.*?)<\/lang *>")?;

    for header_match in header_re.captures_iter(&code) {
        let lang = header_match.at(1).ok_or(RosettaError::UnexpectedFormat)?;
        println!("LANGUAGE: {}", lang);
        let program_code = header_match.at(2).ok_or(RosettaError::UnexpectedFormat)?;
        for program_match in program_re.captures_iter(program_code) {
            let program  = program_match.at(1).ok_or(RosettaError::UnexpectedFormat)?;
            println!("PROGRAM: {}", program);
        }
    }
    Ok(())
}
