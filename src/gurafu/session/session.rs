use std::{
    collections::HashMap,
    fmt::Write as FmtWrite,
    fs::{self, File, OpenOptions},
    io::{self, Error, ErrorKind, Write as IoWrite},
    path::PathBuf,
};

use uuid::Uuid;

use crate::gurafu::{
    mutation::{MutationAction, MutationResult, MutationStatement},
    schema::{load_vertex_definition, SchemaAction, SchemaStatement, VertexDefinition},
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
        println!("Connecting to database...");
    }

    pub fn use_graph(&mut self, name: &str) -> io::Result<()> {
        println!("Using graph {}", name);

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
        println!("Executing schema statement...");

        let initial_step = &statement.steps[0];

        match initial_step.action {
            SchemaAction::CreateGraph => {
                let graph_name = initial_step.args.get("graph_name").unwrap();

                println!("Creating graph {}", graph_name);

                {
                    let path_to_db = PathBuf::from_iter(["gurafu", graph_name]);
                    fs::create_dir_all(&path_to_db).unwrap();
                }

                println!("Graph {} created", graph_name);
                Ok(())
            }
            SchemaAction::CreateVertex => {
                let vertex_name = initial_step.args.get("vertex_name").unwrap();

                println!("Creating vertex {}", vertex_name);

                {
                    // Create vertex directory
                    let path_to_vertex =
                        PathBuf::from_iter(["gurafu", &self.graph_name, "vertices", vertex_name]);

                    fs::create_dir_all(&path_to_vertex).unwrap();

                    // Create vertex definition file
                    let path_to_definition_file = path_to_vertex.join("definition");

                    let options = match statement.steps[1..]
                        .iter()
                        .any(|step| step.action == SchemaAction::AllowRedefine)
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
                    let definition = statement.steps[1..]
                        .iter()
                        .filter(|step| step.action == SchemaAction::CreateVertexProperty)
                        .fold(String::new(), |acc, step| {
                            format!(
                                "{}{},{}\n",
                                acc,
                                step.args.get("property_name").unwrap(),
                                step.args.get("property_datatype").unwrap()
                            )
                        })
                        .trim_end()
                        .to_owned();
                    definition_file.write_all(definition.as_bytes()).unwrap();
                }

                println!("Vertex {} created", vertex_name);
                Ok(())
            }
            _ => Err(Error::new(
                ErrorKind::Other,
                "Unsupported initial schema action",
            )),
        }
    }

    pub fn execute_mutation(&self, mutation: &MutationStatement) -> io::Result<MutationResult> {
        println!("Executing mutation statement...");
        let initial_mutation_step = &mutation.steps[0];

        let mut vertex_file: File;
        Ok(match initial_mutation_step.action {
            MutationAction::InsertVertex => {
                let vertex_name = initial_mutation_step.args.get("vertex_name").unwrap();

                let vertex_definition: VertexDefinition =
                    load_vertex_definition(&self.graph_name, vertex_name).unwrap();

                println!("Inserting vertex {}", vertex_name);

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

                    vertex_file = OpenOptions::new()
                        .create_new(true)
                        .write(true)
                        .append(true)
                        .open(path_to_vertex.join(rest_of_id))
                        .unwrap();

                    let mut set_vertex_properties: HashMap<String, String> = HashMap::new();
                    mutation.steps[1..]
                        .iter()
                        .filter(|step| step.action == MutationAction::SetVertexProperty)
                        .for_each(|step| {
                            set_vertex_properties.insert(
                                step.args.get("property_name").unwrap().to_string(),
                                step.args.get("property_value").unwrap().to_string(),
                            );
                        });

                    let mut content = String::new();
                    for property_definition in &vertex_definition.property_definitions {
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

                println!("Inserted vertex {}", vertex_name);
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
}
