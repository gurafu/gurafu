use crate::gurafu::datatype::DataType;

use super::{SchemaStatement, SchemaStep};

pub struct SchemaBuilder {
    steps: Vec<SchemaStep>,
}

impl SchemaBuilder {
    pub fn new() -> SchemaBuilder {
        SchemaBuilder { steps: Vec::new() }
    }

    pub fn create_graph(&mut self, name: &str) -> &mut SchemaBuilder {
        self.steps.push(SchemaStep::CreateGraph(name.to_string()));
        self
    }

    pub fn create_vertex(&mut self, name: &str) -> &mut SchemaBuilder {
        self.steps.push(SchemaStep::CreateVertex(name.to_string()));
        self
    }

    pub fn allow_redefine(&mut self) -> &mut SchemaBuilder {
        self.steps.push(SchemaStep::AllowRedefine);
        self
    }

    pub fn property(&mut self, name: &str, datatype: DataType) -> &mut SchemaBuilder {
        self.steps.push(SchemaStep::CreateVertexProperty(
            name.to_string(),
            datatype.to_string(),
        ));
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
