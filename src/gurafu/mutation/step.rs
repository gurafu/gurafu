use std::collections::HashMap;

use super::MutationAction;

#[derive(Clone)]
pub struct MutationStep {
    pub action: MutationAction,
    pub args: HashMap<String, String>,
}
