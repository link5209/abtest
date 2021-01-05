use std::vec;

use scenario::Platform::{Android, Ios, H5};
use scenario::{Filter, Grade, Operator, PlatformGroup, Scenario, TagGroup, User, Vip};

// extern crate greetings;
mod parse;
mod scenario;
mod tag;

fn main() {
    // mock user
    let user = User {
        id: "vip".to_string(),
        platform: [H5],
        grade: 7,
    };

    case1(&user);
    case2(&user);
    case3(&user);
    case4(&user);

    let s = parse::parse();
    println!("parse is {}", s.meet(&user));
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
