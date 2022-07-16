use uuid::Uuid;

#[derive(Clone, PartialEq)]
pub enum QueryStep {
    FindVertex(String),
    WithId(Uuid),
}
