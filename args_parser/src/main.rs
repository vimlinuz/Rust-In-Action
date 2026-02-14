fn main() {
    let args: Vec<String> = std::env::args().collect::<Vec<String>>();
    args.iter().for_each(|item| println!("The args is {item}"));
}
