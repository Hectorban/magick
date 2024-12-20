use strum::{Display, EnumString};
#[derive(EnumString, Debug, Display, Clone)]
pub enum Planet {
    Sun,
    Moon,
    Mercury,
    Mars,
    Venus,
    Earth,
    Jupiter,
    Saturn,
    Neptune,
    Pluto,
}
