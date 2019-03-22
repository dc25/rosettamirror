use crate::error::RosettaError;
use std::error::*;
use std::collections::*;
use crate::LanguageQuery;

pub struct Languages {
    names: HashMap<String, String>  // map from lowercase name to name of choice
}

impl Languages {
    pub fn new(langs: &LanguageQuery) -> Result<Languages, Box<dyn Error>> {
        Ok(Languages{names: std::collections::HashMap::new()})
    }
}
