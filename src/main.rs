mod gurafu;

use gurafu::client::Client;
use gurafu::datatype::DataType;
use gurafu::mutation::MutationBuilder;
use gurafu::schema::SchemaBuilder;

fn main() -> std::io::Result<()> {
    // Create a new client
    let client = Client {
        host: String::from("localhost"),
        port: 41765,
        username: String::from("gurafu"),
        password: String::from("gurafu"),
    };

    // Create a new session
    let mut session = client.session();

    // Open a new connection
    session.connect();

    // Create a new graph
    let mut schema_builder = SchemaBuilder::new();

    let statement = schema_builder.create_graph("my_test_db").build();

    session.execute_schema(&statement)?;

    // Use the newly created graph
    session.use_graph("my_test_db");

    // Create a new vertex named "user"
    let statement = schema_builder
        .create_vertex("user")
        .property("username", DataType::Text)
        .property("password", DataType::Text)
        .property("last_logged_in", DataType::Timestamp)
        .property("created_at", DataType::Timestamp)
        .build();

    session.execute_schema(&statement)?;

    // Create a new "user" vertex with some properties
    let mut mutation_builder = MutationBuilder::new();

    let mutation = mutation_builder
        .insert_vertex("user")
        .property("username", "Shinigami")
        .property(
            "password",
            "$2a$13$mhQRlZqBqPUbkThjgBp1r.ftgpzG54ra4mTCS0acigwwk1xwUMH1q",
        )
        // Intentionally omit "last_logged_in"
        .property("created_at", "2022-07-09T08:47:45.409Z")
        .build();

    let result = session.execute_mutation(&mutation)?;

    println!("Generated id of vertex was {}", result.vertex_id);

    Ok(())
}
