use std::{collections::HashMap, fmt::Display};

use uuid::Uuid;

pub struct QueryResult<'a> {
    pub vertex_name: String,
    pub vertex_id: Uuid,
    pub properties: HashMap<&'a str, Box<dyn Display + 'static>>,
}
