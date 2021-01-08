use crate::bracket;
use regex::Regex;
// use std::error::Error;

#[derive(Debug)]
pub struct Scaner<'a> {
    pub text: &'a str,
}

// fn(x, y, z, ...)
impl<'a> Scaner<'a> {
    fn balance_brackets(arg_str: &str) -> Vec<String> {
        let extract_args: Regex = Regex::new(r"([^,]+\(.+?\))|([^,]+)").unwrap();
        // let caps1 = extract_args.captures(arg_str).unwrap();
        let mut arg1: &str = "";
        let mut end = 0;
        // let mut start = 0;
        let mut text = arg_str;
        let mut res = vec![];

        while text.len() > 0 {
            let caps1 = extract_args.captures(text).unwrap();
            for (i, cap) in caps1.iter().enumerate() {
                match cap {
                    Some(c) if i != 0 => {
                        // println!("caps1: {}", c.as_str());
                        // arg1 = c.as_str();
                        end = c.end();
                        // start = c.start();
                        arg1 = c.as_str();
                        let xx = c.as_str();
                        // println!("xx: {}", xx);
                        break;
                    }
                    Some(_) => (),
                    None => (),
                }
            };
            let bra = bracket::Brackets::from(arg1);
            let is_b = bra.are_balanced();
            if is_b {
                // println!("arg1: {}, end: {}", arg1, end);
                text = &text[end..];
                // println!("text[end..]: {}", text);

                res.push(arg1.to_string())

            // let caps1 = extract_args.captures(arg_str).unwrap();
            } else {
                let mut idx = 0;
                let mut str1 = String::from(arg1);
                for n in end..text.len() {
                    str1.push(text.chars().nth(n).unwrap());
                    let bra = bracket::Brackets::from(&str1[..]);
                    let is_b = bra.are_balanced();
                    if is_b {
                        // println!("xxx: {}", &text[0..n]);
                        idx = n;
                        res.push(str1);

                        break;
                    }
                }
                text = &text[idx+1..];
                // println!("yyy text[end..]: {}", text);

            }
        }

        res
    }

    pub fn scan(&self, text: &'a str) -> Option<Filter> {
        let extract_func: Regex = Regex::new(r"(\b[^()]+)\((.*)\)$").unwrap();
        if !extract_func.is_match(text) {
            // return Err("extract_func failed");
            return None;
        }

        let caps = extract_func.captures(text).unwrap();
        let op = &caps[1];
        let arg_str = &caps[2];

        // println!("all-> {}; op-> {}; arg_str-> {}", &caps[0], op, arg_str);

        // let extract_args: Regex = Regex::new(r"([^,]+\(.+?\))|([^,]+)").unwrap();

        let mut args = vec![];
        // for caps in extract_args.captures_iter(arg_str) {
        //     // println!("arg match 2-> {}",&caps.get(2));

        //     for (i, cap) in caps.iter().enumerate() {
        //         match cap {
        //             Some(c) if i != 0 => {
        //                 // println!("arg-> {}", c.as_str().clone());
        //                 args.push(c.as_str());
        //             }
        //             Some(_) => (),
        //             None => println!("none..."),
        //         }
        //         // println!("extract_args match {:?}", cap.unwrap().as_str());
        //     }
        // }

        // println!("args: {:?}", args);

        // Balanced Brackets
        let args1 = Scaner::balance_brackets(arg_str);
        let v2: Vec<&str> = args1.iter().map(|s| &**s).collect();
        args = v2;

        // let caps1 = extract_args.captures(arg_str)?;
        // for (i, cap) in caps1.iter().enumerate() {
        //     match cap {
        //         Some(c) if i != 0 => {
        //             println!("caps1: {}", c.as_str());
        //             // args.push(c.as_str());
        //         }
        //         Some(_) => (),
        //         None => println!("none..."),
        //     }
        //     // println!("extract_args match {:?}", cap.unwrap().as_str());
        // }

        // let bra = bracket::Brackets::from(arg_str);
        // let is_b = bra.are_balanced();
        // if is_b {
        // } else {
        // }

        match op {
            "gt" => return Some(Filter::Gt(args[0].to_string(), args[1].to_string())),
            "ge" => return Some(Filter::Ge(args[0].to_string(), args[1].to_string())),
            "lt" => return Some(Filter::Lt(args[0].to_string(), args[1].to_string())),
            "le" => return Some(Filter::Le(args[0].to_string(), args[1].to_string())),
            "eq" => return Some(Filter::Eq(args[0].to_string(), args[1].to_string())),
            "ne" => return Some(Filter::Ne(args[0].to_string(), args[1].to_string())),
            "range" => return Some(Filter::Range(args[0].to_string(), args[1].to_string())),
            "in" => return Some(Filter::In(args[0].to_string(), args[1].to_string())),
            "nin" => return Some(Filter::Nin(args[0].to_string(), args[1].to_string())),
            "and" => {
                let mut ll = vec![];
                for arg in args.iter() {
                    let f = self.scan(arg);
                    ll.push(f);
                }

                Some(Filter::And(ll))
            }
            "or" => {
                let mut ll = vec![];
                for arg in args.iter() {
                    let f = self.scan(arg);
                    ll.push(f);
                }

                Some(Filter::Or(ll))
            }

            &_ => None,
        }
    }
}

// pub struct Filter<'a> {
//     name: &'a str,
//     args: Vec<Filter<'a>>,
// }

#[derive(Debug)]
pub enum Filter {
    Gt(String, String),
    Ge(String, String),
    Lt(String, String),
    Le(String, String),
    Eq(String, String),
    Ne(String, String),
    Range(String, String),
    In(String, String),
    Nin(String, String),
    And(Vec<Option<Filter>>),
    Or(Vec<Option<Filter>>),
}

// #[derive(Debug)]
// pub enum Filter<'a> {
//     Tag{name: &'a str, op: &'a str, val: &'a str},    // eq(vip, false)
//     Tagroup{name: &'a str, filters: Vec<Filter<'a>>}, // and(eq("vip", false), gt("grade", 7))
// }

// https://stackoverflow.com/questions/18906514/regex-for-matching-functions-and-capturing-their-arguments/18908330
pub fn some_helper_function() {
    // and(vip#eq#false, grade#in#(4,5))
    // or(and(vip#eq#false, grade#in#(4,5)), vip#eq#false)
    let re1: Regex = Regex::new(r"(?P<key>.+)#(?P<op>.+)#(?P<val>.+)").unwrap();
    let caps = re1.captures("vip#eq#false").unwrap();
    // println!("is match {:?}", caps);
    // println!(".... match {:?}, {:?}, {:?}", &caps["key"], &caps["op"], &caps["val"]);

    // re.is_match(text)

    let extractFunc: Regex = Regex::new(r"(\b[^()]+)\((.*)\)$").unwrap();
    // let extractArgs: Regex = Regex::new(r"(?:[^,()]+((?:\(([^()]+|\((?<open>)|\)(?<-open>))*\)))*)+").unwrap();
    // let extractArgs: Regex = Regex::new(r"(?:[^,()]+((?:\(([^()]+|([^,]+))").unwrap();
    // let extractArgs: Regex = Regex::new(r#"(?:[^,()]+((?:\((?>[^()]+|\((?<open>)|\)(?<-open>))*\)))*)+"#).unwrap();
    let extractArgs: Regex = Regex::new(r"([^,]+\(.+?\))|([^,]+)").unwrap();

    let caps = extractFunc
        .captures(r#"and(vip("eq", false), or(platform("in", [5, 6]), grade("gt", 7)))"#)
        .unwrap();
    println!("is match {:?}", &caps[0]);
    println!("is match {:?}", &caps[1]);
    println!("is match {:?}", &caps[2]);

    // let caps1 = extractArgs
    //     .captures("vip#eq#false, grade#in#(4,5), vip#eq#false")
    //     .unwrap();
    // println!("b match {:?}", &caps1[0]);
    // println!("b match {:?}", &caps1[1]);
    // println!("b match {:?}", &caps1[2]);

    // let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();

    println!("....");
    for caps in
        extractArgs.captures_iter(r#"vip("eq", false), or(platform("in", [5, 6]), grade("gt", 7))"#)
    {
        println!("c1 is {:?}", caps.get(1).map_or("noop", |m| m.as_str()));
        println!("c2 is {:?}", caps.get(2).map_or("noop", |m| m.as_str()));
    }

    // let caps1 = extractArgs
    //     .captures("and(vip#eq#false, grade#in#(4,5)), vip#eq#false")
    //     .unwrap();
    // println!("caps1 match {:?}", &caps1[0]);
    // println!("caps1 match {:?}", &caps1[1]);
    // println!("caps1 match {:?}", &caps1[2]);

    // for cap in caps.iter() {
    //     println!("xxx {:?}", cap);
    // }
}
