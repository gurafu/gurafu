use std::collections::HashMap;

use super::{MutationAction, MutationStatement, MutationStep};

pub struct MutationBuilder {
    steps: Vec<MutationStep>,
}

impl MutationBuilder {
    pub fn new() -> MutationBuilder {
        MutationBuilder { steps: Vec::new() }
    }

    pub fn insert_vertex(&mut self, name: &str) -> &mut MutationBuilder {
        self.steps.push(MutationStep {
            action: MutationAction::InsertVertex,
            args: HashMap::from([("vertex_name".to_owned(), name.to_string())]),
        });
        self
    }

    pub fn property(&mut self, name: &str, value: &str) -> &mut MutationBuilder {
        self.steps.push(MutationStep {
            action: MutationAction::SetVertexProperty,
            args: HashMap::from([
                ("property_name".to_owned(), name.to_string()),
                ("property_value".to_owned(), value.to_string()),
            ]),
        });
        self
    }

    pub fn build(&mut self) -> MutationStatement {
        // TODO @Shinigami92 2022-07-09: validate mutation steps
        let statement = MutationStatement {
            steps: self.steps.clone(),
        };
        self.steps.clear();
        statement
    }
}