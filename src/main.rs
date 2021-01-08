use std::vec;

use scenario::Platform::{Android, Ios, H5};
use scenario::{Filter, Grade, Operator, PlatformGroup, Scenario, TagGroup, User, Vip};

// extern crate greetings;
mod bracket;
mod lex;
mod scenario;
mod tag;
mod parser;

fn main() {

    // println!("test: {}", &"Golden Eagle"[5..]);
    // match parser::parse(r"vip(eq, false)") {
    //     Some(f) => println!("f is {:?}", f),
    //     None => println!("none..."),
    // }

    // let p = parser::Parser(r"   or(vip(eq, false), grade(gt, 7))");
    // // let f = p.parse();
    // let b = p.is_tagroup();
    // println!("is_tagroup {:?}", b);

    let ss = r"and(eq(vip, false), or(eq(vip, false), gt(grade, 7)), and(eq(vip, false), or(in(platform, [3,4]), gt(grade, 7))))";
    let s = lex::Scaner{text: r"eq(vip, false)"};
    // let s = lex::Scaner{text: r"and(eq(vip, false), gt(grade, 7))"};
    let f = s.scan(ss);
    // let f = s.scan(r"and(eq(vip, false), or(eq(vip, false), gt(grade, 7)))");
    // let f = s.scan(r"and(eq(vip, false), gt(grade, 7))");
    // let f = s.scan(r"eq(vip, false)");

    println!("输入 str: {}", ss);
    println!();
    match f {
        Some(f) => println!("...分词: {:?}", f),
        None => println!("scan none..."),
    }


    // mock user
    let user = User {
        id: "vip".to_string(),
        platform: [H5],
        grade: 7,
    };

    println!();
    case1(&user);
    case2(&user);
    case3(&user);
    case4(&user);

    // lex::some_helper_function();
    // println!("is match {}", re);
    let b = bracket::Brackets::from("and(3, or(7,3)), or(3, 6)");
    println!("are_balanced {}", b.are_balanced());

}

// case 1 -> not vip
fn case1(user: &User) {
    let scenario = Scenario::new(Vip(false));
    println!("case 1 is {}", scenario.meet(&user));
}

// case 2 -> vip && PlatformGroup: [Ios, Android, H5]
fn case2(user: &User) {
    let mut tg = TagGroup::new(Operator::And);
    tg.add(Filter::new(Vip(true)))
        .add(Filter::new(PlatformGroup(vec![Ios, Android, H5])));

    let scenario = Scenario::new(tg);
    println!("case 2 is {}", scenario.meet(&user));
}

// case 3 -> Ios || (vip && Android)
fn case3(user: &User) {
    let mut tg = TagGroup::new(Operator::And);
    tg.add(Filter::new(Vip(true)))
        .add(Filter::new(PlatformGroup(vec![Android])));

    let mut tg1 = TagGroup::new(Operator::Or);
    tg1.add(Filter::new(PlatformGroup(vec![Ios])))
        .add(Filter::new(tg));

    let scenario = Scenario::new(tg1);
    println!("case 3 is {}", scenario.meet(&user));
}

fn case4(user: &User) {
    let scenario = Scenario::new(Grade::new(7));
    println!("case 4 is {}", scenario.meet(&user));
}
