#![allow(unused)]
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::json;

#[derive(Serialize, Debug, Deserialize)]
struct Data {
    x: i32,
    y: i32,
}
impl Data {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn main() {
    let origin = Data::new(0, 0);
    let serialized = serde_json::to_string(&origin).unwrap();
    println!("origin: {:?}", origin);
    println!("origin: {}", serialized);

    let json_data = json!({
        "name": "santosh",
        "age": 21,
        "sanity cleck": 100
    });
    println!("the json data is: {json_data:#?} ");

    let rando_point = json!({
        "x": 12,
        "y": 12,
    });

    // we have to conver the data into string as teh rando_point is a type of Value which this
    // method doesn't takes
    let rando_with_type: Data = serde_json::from_str(&rando_point.to_string()).unwrap();
    println!("The rando data is: {:#?}", rando_with_type);

    let mero_json_wala_string = r#"{
        "x": 34,
        "y": 45
    }"#;
    let mero_arko_string = String::from(
        "{
            \"x\": 56,
            \"y\": 67
        }",
    );

    let with_types = serde_json::from_str::<Data>(mero_json_wala_string).unwrap();
    let with_types_arko = serde_json::from_str::<Data>(&mero_arko_string).unwrap();
    println!("with types: {:#?}", with_types);
    println!("with types arko: {:#?}", with_types_arko);
}
