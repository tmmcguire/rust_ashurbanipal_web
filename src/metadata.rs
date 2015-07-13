//! Project Gutenberg text metadata routines.

use std::collections::HashMap;
use std::collections::hash_map;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::path::Path;

use recommendation::Etext;

#[derive(RustcEncodable)]
pub struct Text {
    pub etext_no:          Etext,
    pub link:              String,
    pub title:             String,
    pub author:            String,
    pub subject:           String,
    pub language:          String,
    pub release_date:      String,
    pub loc_class:         String,
    pub notes:             String,
    pub copyright_status:  String,
    pub score:             Option<f64>,
}

// TODO: find out if this is helpful over plain Text.
#[derive(RustcEncodable)]
pub struct TextRef<'a> {
    pub etext_no:          Etext,
    pub link:              &'a str,
    pub title:             &'a str,
    pub author:            &'a str,
    pub subject:           &'a str,
    pub language:          &'a str,
    pub release_date:      &'a str,
    pub loc_class:         &'a str,
    pub notes:             &'a str,
    pub copyright_status:  &'a str,
    pub score:             Option<f64>,
}

impl Text {
    pub fn score(&self, score : f64) -> TextRef {
        TextRef {
            etext_no:          self.etext_no,
            link:              &self.link,
            title:             &self.title,
            author:            &self.author,
            subject:           &self.subject,
            language:          &self.language,
            release_date:      &self.release_date,
            loc_class:         &self.loc_class,
            notes:             &self.notes,
            copyright_status:  &self.copyright_status,
            score:             Some(score),
        }
    }
}

impl Default for Text {
    fn default() -> Text {
        Text {
            etext_no:          0,
            link:              "".to_string(),
            title:             "".to_string(),
            author:            "".to_string(),
            subject:           "".to_string(),
            language:          "".to_string(),
            release_date:      "".to_string(),
            loc_class:         "".to_string(),
            notes:             "".to_string(),
            copyright_status:  "".to_string(),
            score:             None
        }
    }
}

pub struct Metadata {
    metadata:     HashMap<Etext,Text>,
}

impl Metadata {
    pub fn read<P : AsRef<Path>>(path:P) -> Metadata {
        let texts: HashMap<Etext,Text> = 
            BufReader::new( File::open(path).unwrap() ).lines()
            .skip(1)
            .map( |line| {
                let line  = line.unwrap();
                let elements: Vec<&str> = line.split('\t').collect();
                let etext_no: Etext = elements[0].parse().unwrap();
                let t = Text {
                      etext_no:          etext_no,
                      link:              elements[1].to_string(),
                      title:             elements[2].to_string(),
                      author:            elements[3].to_string(),
                      subject:           elements[4].to_string(),
                      language:          elements[5].to_string(),
                      release_date:      elements[6].to_string(),
                      loc_class:         elements[7].to_string(),
                      notes:             elements[8].to_string(),
                      copyright_status:  elements[9].to_string(),
                      score:             None,
                };
                ( etext_no, t )
            } ).collect();

        Metadata { metadata: texts, }
    }

    pub fn get(&self, etext_no: Etext) -> Option<&Text> {
        self.metadata.get(&etext_no)
    }

    pub fn iter(&self) -> hash_map::Iter<Etext,Text> {
        self.metadata.iter()
    }


    pub fn add_metadata<'a>(&'a self, rows: &Vec<(Etext,f64)>, start: usize, limit: usize) -> Vec<TextRef<'a>> {
        rows.iter()
            // limit rows to given window
            .skip(start).take(limit)
            // collect metadata for chosen texts
            .map( |&(e,s)| (self.get(e),s) )
            // filter out texts with no metadata
            .filter( |&(ref o,_)| o.is_some() )
            // combine the metadata and scored result
            .map( |(ref o,s)| o.unwrap().score(s) )
            // produce a vector
            .collect()
    }
}
