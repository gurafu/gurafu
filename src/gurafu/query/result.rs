use uuid::Uuid;

use crate::gurafu::datatype::DataType;

pub struct QueryResultProperty {
    pub name: String,
    pub value: String,
    pub datatype: DataType,
}

pub struct QueryResult {
    pub vertex_name: String,
    pub vertex_id: Uuid,
    pub properties: Vec<QueryResultProperty>,
}
