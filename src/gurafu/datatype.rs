use strum_macros::{Display, EnumString};

#[derive(Display, EnumString)]
pub enum DataType {
    Text,
    Timestamp,
}
