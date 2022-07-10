use crate::gurafu::schema::{load_vertex_definition, VertexDefinition};

use super::mutation::{Mutation, MutationAction, MutationResult};

use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::{fs, io};
use uuid::Uuid;

pub struct Session {
    pub graph_name: String,
}

impl Session {
    pub fn new() -> Session {
        Session {
            graph_name: String::new(),
        }
    }

    pub fn connect(&self) {
        println!("Connecting to database...");
    }

    pub fn use_graph(&mut self, name: &str) {
        println!("Using graph {}", name);
        self.graph_name = name.to_string();
    }

    pub fn execute_mutation(&self, mutation: &Mutation) -> io::Result<MutationResult> {
        println!("Executing mutation...");
        let initial_mutation_step = &mutation.steps[0];

        let mut vertex_file: File;
        let result: io::Result<MutationResult> = match initial_mutation_step.action {
            MutationAction::InsertVertex => {
                let vertex_name = initial_mutation_step.args.get("vertex_name").unwrap();

                let vertex_definition: VertexDefinition =
                    load_vertex_definition(&self.graph_name, vertex_name)?;

                println!("Inserting vertex {}", vertex_name);

                let result: MutationResult;

                {
                    let id = Uuid::new_v4();
                    let id_simple = id.simple().to_string();

                    let first_two_chars = id_simple[..2].to_string();
                    let path_to_user = format!(
                        "gurafu/{}/vertices/{}/{}",
                        self.graph_name, vertex_name, first_two_chars
                    );

                    match fs::create_dir_all(&path_to_user) {
                        Ok(it) => it,
                        Err(err) => return Err(err),
                    };

                    let rest_of_id = id_simple[2..].to_string();

                    // TODO @Shinigami92 2022-07-09: check if file already exists
                    // In that case we need to generate a new id

                    vertex_file = OpenOptions::new()
                        .create_new(true)
                        .write(true)
                        .append(true)
                        .open(format!("{}/{}", path_to_user, rest_of_id))?;

                    let set_vertex_properties: Vec<(&String, &String)> = mutation.steps[1..]
                        .iter()
                        .filter(|step| step.action == MutationAction::SetVertexProperty)
                        .map(|step| {
                            (
                                step.args.get("property_name").unwrap(),
                                step.args.get("property_value").unwrap(),
                            )
                        })
                        .collect();

                    let content: String = vertex_definition
                        .property_definitions
                        .iter()
                        .map(|property_definition| {
                            let property_value: String = set_vertex_properties
                                .iter()
                                .find(|(property_name, _)| {
                                    // TODO @Shinigami92 2022-07-09: check the property datatype
                                    property_name == &&property_definition.name
                                })
                                .map(|(_, property_value)| property_value.to_string())
                                .unwrap_or_else(|| "".to_string());
                            property_value
                        })
                        .collect::<Vec<String>>()
                        .join("\n");

                    let _ = write!(vertex_file, "{}", content);

                    result = MutationResult {
                        vertex_name: vertex_name.to_string(),
                        vertex_id: id,
                        properties: set_vertex_properties
                            .iter()
                            .map(|(property_name, property_value)| {
                                (property_name.to_string(), property_value.to_string())
                            })
                            .collect(),
                    };
                }

                println!("Inserted vertex {}", vertex_name);
                Ok(result)
            }
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "Unsupported initial mutation action",
            )),
        };
        result
    }
}
