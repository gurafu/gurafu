use std::collections::HashMap;

use uuid::Uuid;

pub struct MutationResult {
    pub vertex_name: String,
    pub vertex_id: Uuid,
    pub properties: HashMap<String, String>,
}
