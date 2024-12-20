use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

#[derive(EnumString, Debug, Display, Clone, EnumIter)]
pub enum Zodiac {
    Aries,
    Taurus,
    Gemini,
    Cancer,
    Leo,
    Virgo,
    Libra,
    Scorpio,
    Sagitarius,
    Capricorn,
    Aquarius,
    Piscis,
}
