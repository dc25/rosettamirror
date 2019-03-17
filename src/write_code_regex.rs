
use std::fs;
use std::io::Write;
use std::io::BufWriter;
use regex::Regex;
use crate::Error;


pub fn write_code<T: Write>(log: &mut BufWriter<T>, dir: &String, code: &str) -> Result<(), Box<dyn Error>>
{

    fs::DirBuilder::new().recursive(true).create(&dir)?;

    /* 
     * awkward use of regex splits due to limited regex 
     * functionality - no lookaheads.  
     */

    let re = Regex::new(r"(?m)<nowiki>.*?</nowiki>")?;
    let code = re.replace_all(code, "REMOVEDNOWIKI");

    // split task up at "=={{header|" intervals.
    let header_re = Regex::new(r"(?m)^===*\{\{[Hh]eader\|")?;
    let mut head_it = header_re.split(&code);
    head_it.next();
    for head in head_it {
        // language should follow immediately
        let language_re = Regex::new(r"([^\}]*)\}\}")?;
        match language_re.captures(head) {
            None => (),
            Some(lang) => 
                match lang.get(1) {
                    None => (),
                    Some(l) => {writeln!(log, "{}", l.as_str())?; ()},
                },
        }
        // split again at "<lang .... >" intervals.
        let prog_re = Regex::new(r"(?mi)<lang[^>\n]*>")?;
        let mut prog_it = prog_re.split(head);
        prog_it.next();
        for prog in prog_it {
            // split again at "</lang>" to isolate program.
            let prog_end_re = Regex::new(r"(?msi)(.*?)</lang *>")?;
            match prog_end_re.captures(prog) {
                None => (),
                Some(ended_prog) => 
                    match ended_prog.get(1) {
                        None => (),
                        Some(ep) => {writeln!(log, "{}", ep.as_str())?;()},
                    },
            }
        }
    }
    Ok(())
}

