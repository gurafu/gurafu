use std::collections::HashMap;

#[derive(PartialEq)]
pub enum MutationAction {
    InsertVertex,
    SetVertexProperty,
}

pub struct MutationStep {
    pub action: MutationAction,
    pub args: HashMap<String, String>,
}

pub struct Mutation<'a> {
    pub steps: &'a [MutationStep],
}

impl Mutation<'_> {
    pub fn new(steps: &[MutationStep]) -> Mutation {
        // TODO @Shinigami92 2022-07-09: I assume that this is bad practice
        Mutation { steps }
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

    pub fn build(&self) -> Mutation {
        // TODO @Shinigami92 2022-07-09: validate mutation steps
        let mutation = Mutation::new(&self.steps);
        mutation
    }
}
