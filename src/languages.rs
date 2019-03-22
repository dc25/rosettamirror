use std::error::*;
use std::collections::*;
use onig::Regex;
use crate::LanguageQuery;


pub struct Languages {
    names: HashMap<String, String>  // map from lowercase name to name of choice
}

impl Languages {
    pub fn new(langs: &LanguageQuery) -> Result<Languages, Box<dyn Error>> {
        let trim_cat_re = Regex::new(r"Category:")?;

        let names: Vec<String> 
            = langs.categorymembers.iter()
                                   .map(|n| trim_cat_re.replace(&n.title,""))
                                   .collect();

        let name_pairs: Vec<(String,String)> 
            = names.into_iter()
                   .map(|n| (n.to_lowercase(), n.clone()))
                   .collect();

        let name_map = name_pairs.into_iter().collect();

        Ok(Languages{names: name_map})
    }
}
