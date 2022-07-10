use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;
use std::str::FromStr;

use super::datatype::DataType;

#[derive(Clone, PartialEq)]
pub enum SchemaAction {
    CreateGraph,
    CreateVertex,
    CreateVertexProperty,
    AllowRedefine,
}

#[derive(Clone)]
pub struct SchemaStep {
    pub action: SchemaAction,
    pub args: HashMap<String, String>,
}

pub struct SchemaStatement {
    pub steps: Vec<SchemaStep>,
}

pub struct SchemaBuilder {
    steps: Vec<SchemaStep>,
}

impl SchemaBuilder {
    pub fn new() -> SchemaBuilder {
        SchemaBuilder { steps: Vec::new() }
    }

    pub fn create_graph(&mut self, name: &str) -> &mut SchemaBuilder {
        self.steps.push(SchemaStep {
            action: SchemaAction::CreateGraph,
            args: HashMap::from([("graph_name".to_owned(), name.to_string())]),
        });
        self
    }

    pub fn create_vertex(&mut self, name: &str) -> &mut SchemaBuilder {
        self.steps.push(SchemaStep {
            action: SchemaAction::CreateVertex,
            args: HashMap::from([("vertex_name".to_owned(), name.to_string())]),
        });
        self
    }

    pub fn allow_redefine(&mut self) -> &mut SchemaBuilder {
        self.steps.push(SchemaStep {
            action: SchemaAction::AllowRedefine,
            args: HashMap::from([]),
        });
        self
    }

    pub fn property(&mut self, name: &str, datatype: DataType) -> &mut SchemaBuilder {
        self.steps.push(SchemaStep {
            action: SchemaAction::CreateVertexProperty,
            args: HashMap::from([
                ("property_name".to_owned(), name.to_string()),
                ("property_datatype".to_owned(), datatype.to_string()),
            ]),
        });
        self
    }

    pub fn build(&mut self) -> SchemaStatement {
        // TODO @Shinigami92 2022-07-09: validate schema steps
        let statement = SchemaStatement {
            steps: self.steps.clone(),
        };
        self.steps.clear();
        statement
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
                    "Invalid vertex definition",
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
