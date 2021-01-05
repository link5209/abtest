use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct Scenario {
    op: String,
    filters: Vec<Filter>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Filter {
    key: String,
    op: String,
    val: Value,
}

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(untagged)]
// enum TagVal {
//     Bool(bool),
//     U8(u8),
// }

pub fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "op": "and",
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
    let v: Scenario = serde_json::from_str(data)?;
    // let res: bool = serde_json::from_value(serde_json::Value(false));

    // u8::from_str("8");

    // use serde_json::json;
    // let u = json!(v.filters[0].val);
    // let u: u8 = serde_json::from_value(u).unwrap();
    // println!("test .... {:#?}", u);

    // bool::from_str("false");

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {:?}", v.op, v.filters);
    println!("extract val {}, {}", v.filters[0].val, v.filters[1].val);
    // println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}
