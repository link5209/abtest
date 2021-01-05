use crate::scenario;
use crate::scenario::Scenario;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScenarioReq {
    op: Option<String>,
    filters: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filter {
    key: String,
    op: String,
    val: Value,
}

pub fn parse<'a>(tree: ScenarioReq) -> Scenario<'a> {
    match tree.op {
        Some(op) => match op.as_str() {
            "and" => {
                let mut tg = scenario::TagGroup::new(scenario::Operator::And);
                for filter in tree.filters.iter() {
                    // 根据变量声明类型自动反序列化
                    let res: Result<Filter> = serde_json::from_value(filter.clone());
                    match res {
                        Ok(f) => println!("ok is {:?}", f),
                        Err(e) => {}
                    }

                    // if let Ok(f) = serde_json::from_value(filter.clone()) {

                    // }

                    // tg.add(scenario::Filter::new(Vip(true)));
                }

                return Scenario::new(scenario::Vip(false));
            }
            "or" => {
                return Scenario::new(scenario::Vip(false));
            }
            &_ => panic!(),
        },
        // 单个filter
        None => {
            if let Some(filter) = tree.filters.first() {
                let f: Filter = serde_json::from_value(filter.clone()).unwrap();

                match f.key.as_str() {
                    "grade" => {
                        let val: i32 = serde_json::from_value(f.val.clone()).unwrap();
                        return Scenario::new(scenario::Grade::new(val));
                    }
                    "vip" => {
                        let val: bool = serde_json::from_value(f.val.clone()).unwrap();
                        return Scenario::new(scenario::Vip(val));
                    }
                    &_ => panic!(),
                }
            } else {
                panic!("no item in filters.")
            }
        }
    }
}
