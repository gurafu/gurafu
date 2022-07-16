#[derive(Clone, PartialEq)]
pub enum MutationStep {
    /// * `name` - The name of the vertex to insert.
    InsertVertex(String),
    /// * `name` - The name of the property to set.
    /// * `value` - The value of the property to set.
    SetVertexProperty(String, String),
}
