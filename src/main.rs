use std::fs::File;
use std::io::prelude::*;
use uuid::Uuid;

fn main() -> std::io::Result<()> {
    let id = Uuid::new_v4();

    let mut file = File::create("foo.txt")?;
    let content = "Hello, world!".to_owned() + "\n" + &id.to_string();
    file.write_all(content.as_bytes())?;
    Ok(())
}
