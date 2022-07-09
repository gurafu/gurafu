mod gurafu;

use gurafu::datatype;
use gurafu::schema::SchemaBuilder;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use uuid::Uuid;

fn main() -> std::io::Result<()> {
    let mut schema_builder = SchemaBuilder::new();

    schema_builder.create_graph("my_test_db").create();

    schema_builder.use_graph("my_test_db");

    schema_builder
        .create_vertex("user")
        .property("username", datatype::TEXT)
        .property("password", datatype::TEXT)
        .property("lastLoggedIn", datatype::TIMESTAMP)
        .property("createdAt", datatype::TIMESTAMP)
        .create();

    // Insert a user
    let id = Uuid::new_v4().simple().to_string();

    let first_two_chars = &id[..2];
    let path_to_user = format!("gurafu/my_test_db/vertices/user/{}", first_two_chars);

    fs::create_dir_all(&path_to_user)?;

    let rest_of_id = &id[2..];

    let mut user_file = File::create(format!("{}/{}", path_to_user, rest_of_id))?;
    user_file.write_all(b"Shinigami\n$2a$13$mhQRlZqBqPUbkThjgBp1r.ftgpzG54ra4mTCS0acigwwk1xwUMH1q\n\n2022-07-09T08:47:45.409Z")?;

    Ok(())
}
