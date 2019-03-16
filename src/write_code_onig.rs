use std::fs;
use std::io::Write;
use std::io::BufWriter;
use onig::Regex;
use crate::Error;

pub fn write_code<T: Write>(log: &mut BufWriter<T>, dir: &String, code: &str) -> Result<(), Error>
{

    fs::DirBuilder::new().recursive(true).create(&dir)?;

    let header_re = Regex::new(r"(?m)^===*\{\{[Hh]eader\|(.*?)\}\}(.*?)(?:\z|(?=^===*\{\{[Hh]eader))")?;
    let program_re = Regex::new(r"(?mi)<lang *(?: [^>]+)?>(.*?)<\/lang *>")?;

    for header_match in header_re.captures_iter(code) {
        let lang = header_match.at(1).ok_or(Error::UnexpectedFormat)?;
        writeln!(log, "{}", lang)?;
        let program_code = header_match.at(2).ok_or(Error::UnexpectedFormat)?;
        for program_match in program_re.captures_iter(program_code) {
            let program  = program_match.at(1).ok_or(Error::UnexpectedFormat)?;
            writeln!(log, "{}", program)?;
        }
    }
    Ok(())
}
