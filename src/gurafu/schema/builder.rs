use std::collections::HashMap;

use crate::gurafu::datatype::DataType;

use super::{SchemaAction, SchemaStatement, SchemaStep};

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
