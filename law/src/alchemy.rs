use strum::{Display, EnumString};
#[derive(EnumString, Debug, Display, Clone)]
pub enum Element {
    Fire,
    Water,
    Air,
    Earth,
}
