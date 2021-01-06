use regex::Regex;
// use fancy_regex::Regex;

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
    for caps in extractArgs.captures_iter(r#"vip("eq", false), or(platform("in", [5, 6]), grade("gt", 7))"#) {
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
