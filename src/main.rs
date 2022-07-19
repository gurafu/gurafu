mod gurafu;

use std::io;

use env_logger::Target;
use log::{info, LevelFilter};

use crate::gurafu::client::Client;
use crate::gurafu::datatype::DataType;
use crate::gurafu::mutation::MutationBuilder;
use crate::gurafu::query::{QueryBuilder, QueryResultProperty};
use crate::gurafu::schema::SchemaBuilder;

fn main() -> io::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Debug)
        .target(Target::Stdout)
        .init();

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

    session.execute_schema(&statement).unwrap();

    // Use the newly created graph
    session.use_graph("my_test_db").unwrap();

    // Create a new vertex named "user"
    let statement = schema_builder
        .create_vertex("user")
        .allow_redefine()
        .property("username", DataType::Text)
        .property("password", DataType::Text)
        .property("last_logged_in", DataType::Timestamp)
        .property("created_at", DataType::Timestamp)
        .build();

    session.execute_schema(&statement).unwrap();

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

    let result = session.execute_mutation(&mutation).unwrap();

    info!("Generated id of vertex was {}", result.vertex_id);

    // Query the user vertex
    let mut query_builder = QueryBuilder::new();

    let query = query_builder
        .find_vertex("user".to_string())
        .with_id(result.vertex_id)
        .build();

    let result2 = session.execute_query(&query).unwrap();

    assert_eq!(result2.vertex_id, result.vertex_id);

    for property in result2.properties {
        match property {
            QueryResultProperty {
                name,
                value,
                datatype,
            } if datatype == DataType::Text => {
                info!(
                    "Queried result {} was {} and has datatype {}",
                    name, value, datatype
                );
            }
            _ => {}
        }
    }

    Ok(())
}
