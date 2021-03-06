use std::{
    collections::HashMap,
    fmt::Write as FmtWrite,
    fs::{self, File, OpenOptions},
    io::{self, Error, ErrorKind, Read, Write as IoWrite},
    path::PathBuf,
};

use log::debug;
use uuid::Uuid;

use crate::gurafu::{
    mutation::{MutationResult, MutationStatement, MutationStep},
    query::{QueryResult, QueryResultProperty, QueryStatement, QueryStep},
    schema::{load_vertex_definition, SchemaStatement, SchemaStep},
};

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
        debug!("Connecting to database...");
    }

    pub fn use_graph(&mut self, name: &str) -> io::Result<()> {
        debug!("Using graph {}", name);

        let path = PathBuf::from_iter(["gurafu", name]);
        if !path.exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Graph {} does not exist.", name),
            ));
        }

        self.graph_name = name.to_string();

        Ok(())
    }

    pub fn execute_schema(&self, statement: &SchemaStatement) -> io::Result<()> {
        debug!("Executing schema statement...");

        let initial_step = &statement.steps[0];

        match initial_step {
            SchemaStep::CreateGraph(graph_name) => {
                debug!("Creating graph {}", graph_name);

                {
                    let path_to_db = PathBuf::from_iter(["gurafu", graph_name]);
                    fs::create_dir_all(&path_to_db).unwrap();
                }

                debug!("Graph {} created", graph_name);
                Ok(())
            }
            SchemaStep::CreateVertex(vertex_name) => {
                debug!("Creating vertex {}", vertex_name);

                {
                    // Create vertex directory
                    let path_to_vertex =
                        PathBuf::from_iter(["gurafu", &self.graph_name, "vertices", vertex_name]);

                    fs::create_dir_all(&path_to_vertex).unwrap();

                    // Create vertex definition file
                    let path_to_definition_file = path_to_vertex.join("definition");

                    let options = match statement.steps[1..]
                        .iter()
                        .any(|step| step == &SchemaStep::AllowRedefine)
                    {
                        true => OpenOptions::new().create(true).write(true).to_owned(),
                        false => OpenOptions::new().create_new(true).write(true).to_owned(),
                    };
                    let mut definition_file = match options.open(&path_to_definition_file) {
                        Ok(file) => file,
                        Err(_) => {
                            return Err(Error::new(
                                ErrorKind::AlreadyExists,
                                format!("Vertex for {} definition already exists. Did you miss calling allow_redefine()?", vertex_name),
                            ));
                        }
                    };
                    let mut definition_content = String::new();
                    for step in statement.steps[1..].iter() {
                        if let SchemaStep::CreateVertexProperty(property_name, datatype) = step {
                            writeln!(definition_content, "{},{}", property_name, datatype).unwrap();
                        }
                    }
                    definition_content = definition_content.trim_end().to_string();
                    definition_file
                        .write_all(definition_content.as_bytes())
                        .unwrap();
                }

                debug!("Vertex {} created", vertex_name);
                Ok(())
            }
            _ => Err(Error::new(
                ErrorKind::Other,
                "Unsupported initial schema action",
            )),
        }
    }

    pub fn execute_mutation(&self, mutation: &MutationStatement) -> io::Result<MutationResult> {
        debug!("Executing mutation statement...");
        let initial_mutation_step = &mutation.steps[0];

        Ok(match initial_mutation_step {
            MutationStep::InsertVertex(vertex_name) => {
                let vertex_definition =
                    load_vertex_definition(&self.graph_name, vertex_name).unwrap();

                debug!("Inserting vertex {}", vertex_name);

                let result: MutationResult;

                {
                    let id = Uuid::new_v4();
                    let id_simple = id.simple().to_string();

                    let first_two_chars = &id_simple[..2];
                    let path_to_vertex = PathBuf::from_iter([
                        "gurafu",
                        &self.graph_name,
                        "vertices",
                        vertex_name,
                        first_two_chars,
                    ]);

                    match fs::create_dir_all(&path_to_vertex) {
                        Ok(it) => it,
                        Err(err) => return Err(err),
                    };

                    let rest_of_id = id_simple[2..].to_string();

                    // TODO @Shinigami92 2022-07-09: check if file already exists
                    // In that case we need to generate a new id

                    let mut vertex_file = OpenOptions::new()
                        .create_new(true)
                        .write(true)
                        .append(true)
                        .open(path_to_vertex.join(rest_of_id))
                        .unwrap();

                    let mut set_vertex_properties: HashMap<String, String> = HashMap::new();

                    for step in mutation.steps.iter() {
                        if let MutationStep::SetVertexProperty(property_name, value) = step {
                            set_vertex_properties
                                .insert(property_name.to_string(), value.to_string());
                        }
                    }

                    let mut content = String::new();

                    for property_definition in vertex_definition.property_definitions {
                        if let Some(property_value) =
                            set_vertex_properties.get(&property_definition.name)
                        {
                            // TODO @Shinigami92 2022-07-09: check the property datatype
                            write!(content, "{property_value}").unwrap();
                        }
                        content.push('\n');
                    }
                    content.pop();

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

                debug!("Inserted vertex {}", vertex_name);
                result
            }
            MutationStep::DropVertex(vertex_name, id) => {
                debug!("Dropping vertex {} with id {}", vertex_name, id);

                let result: MutationResult;

                {
                    let id_simple = id.simple().to_string();

                    let first_two_chars = &id_simple[..2];
                    let path_to_vertex = PathBuf::from_iter([
                        "gurafu",
                        &self.graph_name,
                        "vertices",
                        vertex_name,
                        first_two_chars,
                    ]);

                    let rest_of_id = id_simple[2..].to_string();
                    let path_to_vertex_file = path_to_vertex.join(rest_of_id);

                    match fs::remove_file(&path_to_vertex_file) {
                        Ok(it) => it,
                        Err(err) => return Err(err),
                    };

                    result = MutationResult {
                        vertex_name: vertex_name.to_string(),
                        vertex_id: id.to_owned(),
                        properties: HashMap::new(),
                    }
                }

                debug!("Dropped vertex {} with id {}", vertex_name, id);
                result
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Unsupported initial mutation action",
                ))
            }
        })
    }

    pub fn execute_query(&self, query: &QueryStatement) -> io::Result<QueryResult> {
        debug!("Executing query statement...");
        let initial_query_step = &query.steps[0];

        let mut vertex_file: File;
        Ok(match initial_query_step {
            QueryStep::FindVertex(vertex_name) => {
                let vertex_definition =
                    load_vertex_definition(&self.graph_name, vertex_name).unwrap();

                debug!("Find vertex {}", vertex_name);

                let result;

                match query.steps[1] {
                    QueryStep::WithId(id) => {
                        let id_simple = id.simple().to_string();

                        let first_two_chars = &id_simple[..2];
                        let path_to_vertex = PathBuf::from_iter([
                            "gurafu",
                            &self.graph_name,
                            "vertices",
                            vertex_name,
                            first_two_chars,
                        ]);

                        let rest_of_id = id_simple[2..].to_string();

                        vertex_file = OpenOptions::new()
                            .read(true)
                            .open(path_to_vertex.join(rest_of_id))
                            .unwrap();

                        let mut content = String::new();

                        vertex_file.read_to_string(&mut content).unwrap();

                        let lines = content.lines();

                        let mut properties: Vec<QueryResultProperty> = Vec::new();

                        for (index, property_definition) in
                            vertex_definition.property_definitions.iter().enumerate()
                        {
                            properties.push(QueryResultProperty {
                                name: property_definition.name.to_string(),
                                value: lines.clone().nth(index).unwrap().to_string(),
                                datatype: property_definition.datatype,
                            });
                        }

                        result = QueryResult {
                            vertex_name: vertex_name.to_string(),
                            vertex_id: id,
                            properties,
                        }
                    }
                    _ => {
                        return Err(Error::new(
                            ErrorKind::Other,
                            "Unsupported second query step",
                        ))
                    }
                }

                debug!("Found vertex {}", vertex_name);
                result
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Unsupported initial query step",
                ))
            }
        })
    }
}
