use crate::scenario;
use crate::scenario::Scenario;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScenarioReq {
    op: Option<String>,
    filters: Vec<Filter>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Filter {
    key: String,
    op: String,
    val: Value,
}

pub fn parse<'a>() -> Scenario<'a> {
    let data = r#"
        {
            "op": null,
            "filters": [
                {
                    "key": "grade",
                    "op": "gt",
                    "val": 7
                },
                {
                    "key": "vip",
                    "op": "eq",
                    "val": true
                }
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let tree: ScenarioReq = serde_json::from_str(data).unwrap();
    // let tree: ScenarioReq = serde_json::from_str(data)?;
    println!("tree is {:?}", tree);

    match tree.op {
        Some(op) => Scenario::new(scenario::Vip(false)),
        // 单个filter
        None => {
            if let Some(filter) = tree.filters.first() {
                match filter.key.as_str() {
                    "grade" => {
                        let val: i32 = serde_json::from_value(filter.val.clone()).unwrap();
                        return Scenario::new(scenario::Grade::new(val))
                    }
                    "vip" => println!("vip..."),
                    &_ => println!("&_..."),
                }
            } else {
                panic!("no item in filters.")
            }

            Scenario::new(scenario::Vip(false))
        }
    }
}
