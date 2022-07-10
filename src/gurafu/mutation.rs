use std::collections::HashMap;

use uuid::Uuid;

#[derive(PartialEq)]
pub enum MutationAction {
    InsertVertex,
    SetVertexProperty,
}

pub struct MutationStep {
    pub action: MutationAction,
    pub args: HashMap<String, String>,
}

pub struct MutationStatement<'a> {
    pub steps: &'a [MutationStep],
}

impl MutationStatement<'_> {
    pub fn new(steps: &[MutationStep]) -> MutationStatement {
        // TODO @Shinigami92 2022-07-09: I assume that this is bad practice
        MutationStatement { steps }
    }
}

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

    pub fn build(&self) -> MutationStatement {
        // TODO @Shinigami92 2022-07-09: validate mutation steps
        MutationStatement::new(&self.steps)
    }
}

pub struct MutationResult {
    pub vertex_name: String,
    pub vertex_id: Uuid,
    pub properties: HashMap<String, String>,
}
