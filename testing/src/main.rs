use std::collections::HashMap;
fn main() {
    // let mut scores = HashMap::new();
    //
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    //
    // // let team_name = String::from("Blue");
    // // let score = scores.get(&team_name).copied().unwrap_or(0);
    // // println!("The value of score is {}", score);
    // for (key, value) in scores {
    //     println!("The key is {} and the value is {}", key, value);
    // }
    //
    //
    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");
    //
    // let mut map = HashMap::new();
    // map.insert(&field_name, &field_value);
    // // field_name and field_value are invalid at this point, try using them and
    // // see what compiler error you get!
    // println!("The key is: {},The value is: {}", field_name, field_value);
    //
    // for (key, value) in map {
    //     println!("The key in map is: {},The value in map is: {}", key, value);
    // }

    let mut map = HashMap::new();
    map.insert(String::from("key"), String::from(String::from("value")));
    map.insert(
        String::from("second key"),
        String::from(String::from("second value")),
    );
    map.insert(
        String::from("thrid key"),
        String::from(String::from("thrid value")),
    );
    let want = map
        .entry(String::from("key"))
        .or_insert(String::from("new value"));

    println!("The data is {}", want);
}
