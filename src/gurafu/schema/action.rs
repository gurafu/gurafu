#[derive(Clone, PartialEq)]
pub enum SchemaAction {
    CreateGraph,
    CreateVertex,
    CreateVertexProperty,
    AllowRedefine,
}
