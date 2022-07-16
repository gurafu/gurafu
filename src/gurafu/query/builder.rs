use uuid::Uuid;

use super::{QueryStatement, QueryStep};

pub struct QueryBuilder {
    steps: Vec<QueryStep>,
}

impl QueryBuilder {
    pub fn new() -> QueryBuilder {
        QueryBuilder { steps: Vec::new() }
    }

    pub fn find_vertex(&mut self, name: String) -> &mut QueryBuilder {
        self.steps.push(QueryStep::FindVertex(name));
        self
    }

    pub fn with_id(&mut self, id: Uuid) -> &mut QueryBuilder {
        self.steps.push(QueryStep::WithId(id));
        self
    }

    pub fn build(&mut self) -> QueryStatement {
        // TODO @Shinigami92 2022-07-13: validate query steps
        let statement = QueryStatement {
            steps: self.steps.clone(),
        };
        self.steps.clear();
        statement
    }
}
