use std::char::from_u32;

fn main() {
    (1..1000).into_iter().for_each(|i| {
        println!(
            "Decoded message: {}, key: {i}",
            ("ıĬīĪķ".to_string())
                .chars()
                .map(|character| { from_u32(character as u32 ^ i).unwrap() })
                .collect::<String>()
        );
    })
}
