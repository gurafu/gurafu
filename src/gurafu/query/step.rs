use uuid::Uuid;

#[derive(Clone, PartialEq)]
pub enum QueryStep {
    /// * `name` - The name of the vertex to find.
    FindVertex(String),
    /// * `id` - The id of the vertex to find.
    WithId(Uuid),
}
