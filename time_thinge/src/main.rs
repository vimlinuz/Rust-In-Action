use std::time::Duration;
fn main() {
    let mut time = Duration::from_secs(12);
    time += Duration::from_micros(42);
    let only_secs = time.as_secs();
    println!("The total seconds is {}", only_secs);
}
