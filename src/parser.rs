use crate::scenario;
use regex::Regex;
use std::error::Error;
use std::result::Result;
// use serde::{Deserialize, Serialize};

// 语法树
#[derive(Debug)]
pub enum Filter<'a> {
    Tag(&'a str, &'a str, &'a str),    // (vip, eq, false)
    Tagroup(&'a str, Vec<Filter<'a>>), // (and, [vip(eq, false), grade(gt, 7)])
}

#[derive(Debug)]
pub struct Parser<'a>(pub &'a str);

impl<'a> Parser<'a> {
    fn is_tag(&self) -> bool {
        true
    }

    fn captures(&self) -> Filter {
        Filter::Tag("vip", "eq", "false")
        // Filter::Tagroup("and", "vip(eq, false)", "grade(gt, 7)")
    }

    fn is_tagroup(&self) -> bool {
        true
    }

    fn extractArgs(&self) -> Vec<&'a str> {
        vec!["vip(eq, false)", "grade(gt, 7)"]
    }

    pub fn parse(&self) -> Filter<'a> {
        if self.is_tag() {
            Filter::Tag("vip", "eq", "false")
        } else if self.is_tagroup() {
            let mut filters = vec![];
            for arg in self.extractArgs() {
                let p = Parser(arg);
                filters.push(p.parse());
            }

            Filter::Tagroup("and", filters)
        } else {
            panic!()
        }
    }
}

pub fn parse(s: &str) -> Option<scenario::Filter> {
    let extractFunc: Regex = Regex::new(r"(\b[^()]+)\((.*)\)$").unwrap();
    // Note the +? so that the match is lazy and stops at the first ) it meets.
    // so it doesn't handle nested functions very well
    let extractArgs: Regex = Regex::new(r"([^,]+\(.+?\))|([^,]+)").unwrap();
    // eg: vip(eq, false)
    let extract_tag: Regex =
        Regex::new(r"^(\b[^()|and|or]+)\((gt|ge|lt|le|eq|ne|range|in|nin),\s*(.+)\)$").unwrap();
    let caps = extract_tag.captures(s)?;
    for cap in caps.iter() {
        println!("cap match {:?}", cap?.as_str());
    }

    // and(vip("eq", false), grade("gt", 7))
    // let tag_group: Regex = Regex::new(r"([^,]+\(.+?\))|([^,]+)").unwrap();

    // println!("...caps len {:?}", &caps.len());
    // println!("...caps match {:?}", caps.get(0));
    // println!("...caps match {:?}", &caps[1]);
    // println!("...caps match {:?}", &caps[2]);
    // println!("...caps match {:?}", &caps[3]);

    // and(vip("eq", false), or(platform("in", [5, 6]), grade("gt", 7)))

    Some(scenario::Filter::new(scenario::Vip(true)))
}

fn tag_parse(tag: (&str, String, String)) -> Result<scenario::Filter, Box<dyn Error>> {
    match tag {
        ("vip", op, val) => {
            let t: bool = val.parse()?;
            return Ok(scenario::Filter::new(scenario::Vip(t)));
        }
        ("grade", op, val) => {
            let t: i32 = val.parse()?;
            return Ok(scenario::Filter::new(scenario::Grade::new(t)));
        }
        (&_, _, _) => panic!("tag_parse on match"),
    }

    // Ok(())
}
