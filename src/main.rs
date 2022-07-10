mod gurafu;

use gurafu::client::Client;
use gurafu::datatype::DataType;
use gurafu::mutation::MutationBuilder;
use gurafu::schema::SchemaBuilder;

fn main() -> std::io::Result<()> {
    let client = Client {
        host: String::from("localhost"),
        port: 41765,
        username: String::from("gurafu"),
        password: String::from("gurafu"),
    };

    let mut schema_builder = SchemaBuilder::new();

    schema_builder.create_graph("my_test_db").create()?;

    schema_builder.use_graph("my_test_db");

    schema_builder
        .create_vertex("user")
        .property("username", DataType::Text)
        .property("password", DataType::Text)
        .property("last_logged_in", DataType::Timestamp)
        .property("created_at", DataType::Timestamp)
        .create()?;

    let mut session = client.session();

    session.connect();

    session.use_graph("my_test_db");

    let mut mutation_builder = MutationBuilder::new();

    let mutation = mutation_builder
        .insert_vertex("user")
        .property("username", "Shinigami")
        .property(
            "password",
            "$2a$13$mhQRlZqBqPUbkThjgBp1r.ftgpzG54ra4mTCS0acigwwk1xwUMH1q",
        )
        .property("created_at", "2022-07-09T08:47:45.409Z")
        .build();

    let result = session.execute_mutation(&mutation)?;

    println!("Generated id of vertex was {}", result.vertex_id);

    Ok(())
}
