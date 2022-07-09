mod gurafu;

use gurafu::schema::SchemaBuilder;
use gurafu::types;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use uuid::Uuid;

fn main() -> std::io::Result<()> {
    let database_name = "my_test_db";

    let path_to_db = format!("{}/{}", "gurafu", database_name);

    // Create database
    fs::create_dir_all(&path_to_db)?;

    // Create "user" vertex definition
    let path_to_user_vertex = format!("{}/{}/{}", path_to_db, "vertices", "user");
    fs::create_dir_all(&path_to_user_vertex)?;

    let mut user_definition_file =
        File::create(format!("{}/{}", path_to_user_vertex, "definition"))?;
    user_definition_file
        .write_all(b"username,Text\npassword,Text\nlastLoggedIn,Timestamp\ncreatedAt,Timestamp")?;

    // Insert a user
    let id = Uuid::new_v4().simple().to_string();

    let first_two_chars = &id[..2];
    let path_to_user = format!("{}/{}", path_to_user_vertex, first_two_chars);

    fs::create_dir_all(&path_to_user)?;

    let rest_of_id = &id[2..];

    let mut user_file = File::create(format!("{}/{}", path_to_user, rest_of_id))?;
    user_file.write_all(b"Shinigami\n$2a$13$mhQRlZqBqPUbkThjgBp1r.ftgpzG54ra4mTCS0acigwwk1xwUMH1q\n\n2022-07-09T08:47:45.409Z")?;

    // For later
    SchemaBuilder::new()
        .create_vertex("user")
        .property("username", types::TEXT)
        .property("password", types::TEXT)
        .property("lastLoggedIn", types::TIMESTAMP)
        .property("createdAt", types::TIMESTAMP)
        .create();

    Ok(())
}
