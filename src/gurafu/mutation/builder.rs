use uuid::Uuid;

use super::{MutationStatement, MutationStep};

pub struct MutationBuilder {
    steps: Vec<MutationStep>,
}

impl MutationBuilder {
    pub fn new() -> MutationBuilder {
        MutationBuilder { steps: Vec::new() }
    }

    pub fn insert_vertex(&mut self, name: &str) -> &mut MutationBuilder {
        self.steps
            .push(MutationStep::InsertVertex(name.to_string()));
        self
    }

    pub fn property(&mut self, name: &str, value: &str) -> &mut MutationBuilder {
        self.steps.push(MutationStep::SetVertexProperty(
            name.to_string(),
            value.to_string(),
        ));
        self
    }

    pub fn drop_vertex(&mut self, name: &str, id: Uuid) -> &mut MutationBuilder {
        self.steps
            .push(MutationStep::DropVertex(name.to_string(), id));
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
