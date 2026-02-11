use std::fmt;

struct Message {
    content: String,
    hasher: u32,
}

impl Message {
    fn new(content: String, hasher: u32) -> Self {
        Self { content, hasher }
    }

    fn crypt(self) -> Self {
        Self {
            content: self
                .content
                .chars()
                .map(|c| char::from_u32(c as u32 ^ self.hasher).expect("unicode conversion error"))
                .collect::<String>(),
            hasher: self.hasher,
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

fn main() {
    let message = Message::new(String::from("rohit"), 323);

    println!("The original message is: {}", message);

    let encrypted_message = message.crypt();
    println!("The encrypted message is: {}", encrypted_message);

    let plane_message = encrypted_message.crypt();
    println!("The plane message again is: {}", plane_message);
}
