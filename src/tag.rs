use std::fmt::Debug;

// 标签系统API
// http://yapi.yc345.tv/project/512/interface/api/cat_1209

enum Role {
    Student,
    Teacher,
}

enum Registration {
    Signup,
    Batch,
    Qq,
    Weixin,
    Huawei,
    Oppo,
    ParentApplet,
    Bubugao,
    Dushulang,
}

enum Gender {
    Male,
    Female,
}

enum Grade {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
}

#[derive(Debug)]
pub enum Cmp<T: PartialOrd> {
    Gt(T), // Greater than comparison, expr > expr
    Ge(T), // Greater than or equal to comparison, expr >= expr
    Lt(T), // Less than comparison, expr < expr
    Le(T), // Less than or equal to comparison, expr <= expr
    Eq(T), // Equality comparison, expr == expr
    Ne(T), // Nonequality comparison, expr != expr
}

impl<T: PartialOrd> Cmp<T> {
    fn cmp(&self, other: &T) -> bool {
        match self {
            Cmp::Gt(v) => v > other,
            Cmp::Ge(v) => v >= other,
            Cmp::Lt(v) => v < other,
            Cmp::Le(v) => v <= other,
            Cmp::Eq(v) => v == other,
            Cmp::Ne(v) => v != other,
        }
    }
}

#[derive(Debug)]
pub enum Operation<T: PartialOrd> {
    Compare(Cmp<T>),
    Range(Cmp<T>, Cmp<T>),
    // compares a value to a set of values, returns true if any value of the elements meets the condition,
    // otherwise, it returns false
    In(Vec<T>),
    Nin(Vec<T>),
}

impl<T: PartialOrd+Debug> Operation<T> {
    pub fn cmp(&self, other: &T) -> bool {
        match self {
            Operation::Compare(cmp) => cmp.cmp(other),
            Operation::Range(left, right) => left.cmp(other) && right.cmp(other),
            Operation::In(tags) => tags.iter().any(|v| v == other),
            Operation::Nin(tags) => !tags.iter().any(|v| v == other),
        }
    }
}
