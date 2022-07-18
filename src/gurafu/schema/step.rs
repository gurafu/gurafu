use crate::gurafu::datatype::DataType;

#[derive(Clone, PartialEq)]
pub enum SchemaStep {
    /// * `name` - The name of the graph.
    CreateGraph(String),
    /// * `name` - The name of the vertex.
    CreateVertex(String),
    /// * `name` - The name of the property.
    /// * `datatype` - The data type of the property.
    CreateVertexProperty(String, DataType),
    AllowRedefine,
}
