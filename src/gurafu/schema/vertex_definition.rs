use std::{
    fs::File,
    io::{self, BufRead, Error, ErrorKind},
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::gurafu::datatype::DataType;

use super::VertexPropertyDefinition;

pub struct VertexDefinition {
    pub name: String,
    pub property_definitions: Vec<VertexPropertyDefinition>,
}

pub fn load_vertex_definition(graph_name: &str, vertex_name: &str) -> io::Result<VertexDefinition> {
    let path_to_vertex_definition_file =
        PathBuf::from_iter(["gurafu", graph_name, "vertices", vertex_name, "definition"]);

    let mut property_definitions: Vec<VertexPropertyDefinition> = Vec::new();

    if let Ok(lines) = read_lines(path_to_vertex_definition_file) {
        for line in lines.flatten() {
            let parts: Vec<String> = line.split(',').map(String::from).collect();
            if parts.len() != 2 {
                return Err(Error::new(
                    ErrorKind::InvalidData,
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
    let file = File::open(filename).unwrap();
    Ok(io::BufReader::new(file).lines())
}
