use std::collections::HashMap;

use super::SchemaAction;

#[derive(Clone)]
pub struct SchemaStep {
    pub action: SchemaAction,
    pub args: HashMap<String, String>,
}
