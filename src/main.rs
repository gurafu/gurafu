use std::fs;
use std::fs::File;
use std::io::prelude::*;
use uuid::Uuid;

pub mod gurafu {
    pub mod types {
        pub const TEXT: &'static str = "Text";
        pub const TIMESTAMP: &'static str = "Timestamp";
    }

    pub struct SchemaBuilder {}

    impl SchemaBuilder {
        pub fn new() -> SchemaBuilder {
            println!("{}", "new");
            SchemaBuilder {}
        }

        pub fn create_vertex(&mut self, name: &str) -> &mut SchemaBuilder {
            println!("{}", name);
            self
        }

        pub fn property(&mut self, name: &str, typ: &'static str) -> &mut SchemaBuilder {
            println!("{} {}", name, typ);
            self
        }

        pub fn create(&mut self) -> () {
            println!("{}!", "create");
            ()
        }
    }
}

fn main() -> std::io::Result<()> {
    let id = Uuid::new_v4();

    let first_two = &id.to_string()[..2];
    let path = format!("{}{}{}{}", "collections/", first_two, "/", id);

    print!("{}", path);

    fs::create_dir_all("collections/".to_owned() + first_two)?;

    let mut file = File::create(path)?;
    let content = "Hello, world!".to_owned() + "\n" + &id.to_string();
    file.write_all(content.as_bytes())?;

    gurafu::SchemaBuilder::new()
        .create_vertex("user")
        .property("username", gurafu::types::TEXT)
        .property("password", gurafu::types::TEXT)
        .property("lastLoggedIn", gurafu::types::TIMESTAMP)
        .property("createdAt", gurafu::types::TIMESTAMP)
        .create();

    Ok(())
}
