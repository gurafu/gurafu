use strum_macros::{Display, EnumString};

#[derive(Clone, Display, EnumString, PartialEq)]
pub enum DataType {
    Text,
    Timestamp,
}
