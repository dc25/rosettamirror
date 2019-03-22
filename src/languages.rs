use std::error::*;
use std::collections::*;
use onig::Regex;
use crate::LanguageQuery;


#[derive(Debug)]
pub struct Languages {
    names: HashMap<String, String>  // map from lowercase name to name of choice
}

impl Languages {
    pub fn new(langs: &LanguageQuery) -> Result<Languages, Box<dyn Error>> {
        let trim_cat_re = Regex::new(r"Category:")?;

        let name_map
            = langs.categorymembers
                       .iter()
                       .map(|n| trim_cat_re.replace(&n.title,""))
                       .map(|n| (n.to_lowercase(), n))
                       .collect();

        Ok(Languages{names: name_map})
    }

    pub fn lookup(self: &Self, name: String) -> String {
        if let Some(found_name) = self.names.get(&name) {
            found_name.to_string()
        } else {
            name
        }
    }
}

