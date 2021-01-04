use std::{fmt::Debug, vec};

// https://stackoverflow.com/questions/25959075/why-explicit-lifetime-bound-required-for-boxt-in-struct#25959806
pub struct Scenario<'a> {
    pub filter: Box<dyn Tag + 'a>,
}

// https://users.rust-lang.org/t/box-with-a-trait-object-requires-static-lifetime/35261/2
impl<'a> Scenario<'a> {
    pub fn meet(&self, user: &User) -> bool {
        self.filter.meet(user)
    }

    pub fn new<T: Tag + 'a>(filter: T) -> Scenario<'a> {
        Scenario {
            filter: Box::new(filter),
        }
    }
}

#[allow(dead_code)]
pub struct TestGroup;

#[allow(dead_code)]
pub struct Test;

// https://stackoverflow.com/questions/50040596/how-do-i-derive-a-trait-for-another-trait
// create_filter
// https://stackoverflow.com/questions/56100579/how-do-i-match-on-the-concrete-type-of-a-generic-parameter
pub trait Tag: Debug {
    fn meet(&self, user: &User) -> bool;
    fn create_filter(self) -> Filter;
}

#[derive(Debug)]
pub struct Vip(pub bool);

impl Tag for Vip {
    fn meet(&self, user: &User) -> bool {
        let is_vip = if user.id == "vip" { true } else { false }; // 模拟调用API
        self.0 == is_vip
    }

    fn create_filter(self) -> Filter {
        Filter::Tag(Box::new(self))
    }
}

#[derive(Debug)]
pub struct PlatformGroup(pub Vec<Platform>);

impl Tag for PlatformGroup {
    fn meet(&self, user: &User) -> bool {
        for p in user.platform.iter() {
            if !self.0.contains(p) {
                return false;
            }
        }
        true
    }

    fn create_filter(self) -> Filter {
        Filter::Tag(Box::new(self))
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Platform {
    Ios,
    Android,
    H5,
}

// 入参结构：场景匹配

pub struct User {
    pub id: String,
    pub platform: [Platform; 1],
}

#[derive(Debug)]
pub enum Operator {
    And,
    Or,
}

#[derive(Debug)]
/// 过滤器，简单标签|逻辑组合标签
pub enum Filter {
    Tag(Box<dyn Tag>),
    TagGroup(TagGroup),
}

impl Filter {
    pub fn new<T: Tag>(filter: T) -> Filter {
        filter.create_filter()
    }
}

#[derive(Debug)]
// https://stackoverflow.com/questions/54984969/rust-lifetime-in-vect-convoluted-syntax
pub struct TagGroup {
    pub op: Operator,
    pub operands: Vec<Filter>,
}

impl Tag for TagGroup {
    fn create_filter(self) -> Filter {
        Filter::TagGroup(self)
    }

    // 递归构造图
    // https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes
    fn meet(&self, user: &User) -> bool {
        match self.op {
            Operator::And => {
                for filter in self.operands.iter() {
                    match filter {
                        Filter::TagGroup(tg) if !tg.meet(user) => return false,
                        Filter::TagGroup(_) => true,
                        Filter::Tag(t) if !t.meet(user) => return false,
                        Filter::Tag(_) => true,
                    };
                }
                true
            }
            Operator::Or => {
                for filter in self.operands.iter() {
                    match filter {
                        Filter::Tag(t) if t.meet(user) => return true,
                        Filter::Tag(_) => false,
                        Filter::TagGroup(tg) if tg.meet(user) => return true,
                        Filter::TagGroup(_) => false,
                    };
                }
                false
            }
        }
    }
}

impl TagGroup {
    pub fn new(op: Operator) -> TagGroup {
        TagGroup {
            op,
            operands: vec![],
        }
    }

    /// 添加过滤器
    pub fn add(&mut self, filter: Filter) -> &mut Self {
        self.operands.push(filter);
        self
    }
}

// A or B or C
// A and B and C
// 操作数在计算之前隐式转换为类型bool，结果的类型为bool。两个操作数可以是变量、常量和表达式
// Or,当第一个操作数的计算结果为 false (0) 时计算第二个操作数。在逻辑“或”表达式为 true 时，这将消除对第二个操作数的不必要的计算
// 在决定一事物的若干条件中，只要有一个条件能满足时，结果就会出现；只有当所有条件都不满足时，结果才不出现

// 因此，Or运算的每个操作数其实就是在做自身结果是否为真的运算，即，And运算，
// 而Or运算只是将多个And运算单元串联起来，形成了一个lazy计算的链条而已
