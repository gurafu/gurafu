use std::fs;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;
use std::str::FromStr;

use super::datatype::DataType;

pub struct PropertyDefinition {
    name: String,
    datatype: DataType,
}

pub struct Schema {
    pub graph_name: String,
    pub name: String,
    pub property_definitions: Vec<PropertyDefinition>,
}

#[derive(Debug)]
enum SchemaBuilderState {
    Initial,
    CreateGraph,
    CreateVertex,
}

pub struct SchemaBuilder {
    state: SchemaBuilderState,
    schema: Schema,
}

impl SchemaBuilder {
    pub fn new() -> SchemaBuilder {
        SchemaBuilder {
            state: SchemaBuilderState::Initial,
            schema: Schema {
                graph_name: String::new(),
                name: String::new(),
                property_definitions: Vec::new(),
            },
        }
    }

    pub fn create_graph(&mut self, name: &str) -> &mut SchemaBuilder {
        self.state = SchemaBuilderState::CreateGraph;
        self.schema.graph_name = name.to_string();
        self
    }

    pub fn use_graph(&mut self, name: &str) -> &mut SchemaBuilder {
        self.state = SchemaBuilderState::Initial;
        self.schema.graph_name = name.to_string();
        self
    }

    pub fn create_vertex(&mut self, name: &str) -> &mut SchemaBuilder {
        self.state = SchemaBuilderState::CreateVertex;
        self.schema.name = name.to_string();
        self
    }

    pub fn property(&mut self, name: &str, datatype: DataType) -> &mut SchemaBuilder {
        self.schema.property_definitions.push(PropertyDefinition {
            name: name.to_string(),
            datatype,
        });
        self
    }

    pub fn create(&mut self) -> io::Result<()> {
        match self.state {
            SchemaBuilderState::CreateGraph => {
                println!("creating graph {}", self.schema.name);

                {
                    let path_to_db = format!("gurafu/{}", self.schema.graph_name);
                    fs::create_dir_all(&path_to_db).unwrap();
                }

                self.schema.name = String::new();
                self.schema.property_definitions.clear();
            }
            SchemaBuilderState::CreateVertex => {
                println!("creating vertex {}", self.schema.name);

                {
                    let path_to_vertex = format!(
                        "gurafu/{}/{}/{}",
                        self.schema.graph_name, "vertices", self.schema.name
                    );
                    fs::create_dir_all(&path_to_vertex)?;

                    let mut definition_file =
                        File::create(format!("{}/{}", path_to_vertex, "definition")).unwrap();
                    let definition = self
                        .schema
                        .property_definitions
                        .iter()
                        .fold(String::new(), |acc, property_definition| {
                            format!(
                                "{}{},{}\n",
                                acc, property_definition.name, property_definition.datatype
                            )
                        })
                        .trim_end()
                        .to_owned();
                    definition_file.write_all(definition.as_bytes()).unwrap();
                }

                self.schema.name = String::new();
                self.schema.property_definitions.clear();
            }
            _ => {
                println!("illegal state {:?}", self.state);
            }
        }
        Ok(())
    }
}

pub struct VertexPropertyDefinition {
    pub name: String,
    pub datatype: DataType,
}

pub struct VertexDefinition {
    pub name: String,
    pub property_definitions: Vec<VertexPropertyDefinition>,
}

pub fn load_vertex_definition(graph_name: &str, vertex_name: &str) -> io::Result<VertexDefinition> {
    let path_to_vertex_definition_file =
        format!("gurafu/{}/vertices/{}/definition", graph_name, vertex_name);

    let mut property_definitions: Vec<VertexPropertyDefinition> = Vec::new();

    if let Ok(lines) = read_lines(path_to_vertex_definition_file) {
        for line in lines.flatten() {
            let parts: Vec<String> = line.split(',').map(String::from).collect();
            if parts.len() != 2 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "invalid vertex definition",
                ));
            }
            let name: String = parts[0].clone();
            let datatype: DataType = DataType::from_str(parts[1].as_str()).unwrap();
            property_definitions.push(VertexPropertyDefinition { name, datatype });
        }
    }

    Ok(VertexDefinition {
        name: vertex_name.to_string(),
        property_definitions,
    })
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
