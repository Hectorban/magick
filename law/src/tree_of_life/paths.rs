use crate::tarot::MajorArcana;

use super::{MyteriesPath, Sefira};

pub fn generate_paths() -> Vec<MyteriesPath> {
    vec![
        // aleph
        MyteriesPath::new('א', MajorArcana::Fool, Sefira::Keter, Sefira::Chokmah),
        // bet
        MyteriesPath::new('ב', MajorArcana::Magus, Sefira::Keter, Sefira::Binah),
        // gimel
        MyteriesPath::new(
            'ג',
            MajorArcana::HighPriestess,
            Sefira::Keter,
            Sefira::Tiferet,
        ),
        // dalet
        MyteriesPath::new('ד', MajorArcana::Empress, Sefira::Chokmah, Sefira::Binah),
        // he
        MyteriesPath::new('ה', MajorArcana::Star, Sefira::Chokmah, Sefira::Tiferet),
        // vav
        MyteriesPath::new(
            'ו',
            MajorArcana::Hierophant,
            Sefira::Chokmah,
            Sefira::Chesed,
        ),
        // zayin
        MyteriesPath::new('ז', MajorArcana::Lovers, Sefira::Binah, Sefira::Tiferet),
        // chet
        MyteriesPath::new('ח', MajorArcana::Chariot, Sefira::Binah, Sefira::Gevurah),
        // tet
        MyteriesPath::new('ט', MajorArcana::Lust, Sefira::Chesed, Sefira::Gevurah),
        // yod
        MyteriesPath::new('י', MajorArcana::Hermit, Sefira::Chesed, Sefira::Tiferet),
        // kaf
        MyteriesPath::new(
            'כ',
            MajorArcana::WheelOfFortune,
            Sefira::Chesed,
            Sefira::Netsach,
        ),
        // lamed
        MyteriesPath::new(
            'ל',
            MajorArcana::Adjustment,
            Sefira::Gevurah,
            Sefira::Tiferet,
        ),
        // mem
        MyteriesPath::new('מ', MajorArcana::HangedMan, Sefira::Gevurah, Sefira::Hod),
        // nun
        MyteriesPath::new('נ', MajorArcana::Death, Sefira::Tiferet, Sefira::Netsach),
        // samech
        MyteriesPath::new('ס', MajorArcana::Art, Sefira::Tiferet, Sefira::Yesod),
        // ayin
        MyteriesPath::new('ע', MajorArcana::Devil, Sefira::Tiferet, Sefira::Hod),
        // pe
        MyteriesPath::new('פ', MajorArcana::Tower, Sefira::Netsach, Sefira::Hod),
        // tsadi
        MyteriesPath::new('צ', MajorArcana::Emperor, Sefira::Netsach, Sefira::Yesod),
        // qof
        MyteriesPath::new('ק', MajorArcana::Moon, Sefira::Netsach, Sefira::Malkuth),
        // resh
        MyteriesPath::new('ר', MajorArcana::Sun, Sefira::Hod, Sefira::Yesod),
        // shin
        MyteriesPath::new('ש', MajorArcana::Eon, Sefira::Hod, Sefira::Malkuth),
        // tav
        MyteriesPath::new('ת', MajorArcana::Universe, Sefira::Yesod, Sefira::Malkuth),
    ]
}
