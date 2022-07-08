use std::fs;
use std::fs::File;
use std::io::prelude::*;
use uuid::Uuid;

fn main() -> std::io::Result<()> {
    let id = Uuid::new_v4();

    let first_two = &id.to_string()[..2];
    let path = format!("{}{}{}{}", "collections/", first_two, "/", id);

    print!("{}", path);

    fs::create_dir_all("collections/".to_owned() + first_two)?;

    let mut file = File::create(path)?;
    let content = "Hello, world!".to_owned() + "\n" + &id.to_string();
    file.write_all(content.as_bytes())?;
    Ok(())
}
