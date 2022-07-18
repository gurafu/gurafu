use strum_macros::{Display, EnumString};

#[derive(Clone, Copy, Display, EnumString, PartialEq)]
pub enum DataType {
    Text,
    Timestamp,
}
