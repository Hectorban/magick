use eyre::{ContextCompat, Result};
use neo4rs::{query, Graph};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

use crate::zodiac::Zodiac;

#[derive(EnumString, Debug, Display, Clone, EnumIter)]
pub enum MajorArcana {
    Fool,
    Magus,
    HighPriestess,
    Empress,
    Emperor,
    Hierophant,
    Lovers,
    Chariot,
    Adjustment,
    Hermit,
    WheelOfFortune,
    Lust,
    HangedMan,
    Death,
    Art,
    Devil,
    Tower,
    Star,
    Moon,
    Sun,
    Eon,
    Universe,
}

#[derive(EnumString, Debug, Display, Clone, EnumIter)]
pub enum Suit {
    Major,
    Clubs,
    Cups,
    Swords,
    Pentacles,
}

pub struct TarotCard {
    suit: Suit,
    number: i32,
    name: MajorArcana,
    title: String,
    zodiac: Option<Zodiac>,
}

impl TarotCard {
    pub fn new(
        suit: Suit,
        number: i32,
        name: MajorArcana,
        title: String,
        zodiac: Option<Zodiac>,
    ) -> Self {
        Self {
            suit,
            number,
            name,
            title,
            zodiac,
        }
    }

    pub fn create_major(
        number: i32,
        name: MajorArcana,
        title: &str,
        zodiac: Option<Zodiac>,
    ) -> Self {
        Self::new(Suit::Major, number, name, title.to_string(), zodiac)
    }
}

pub fn generate_major_arcana() -> Vec<TarotCard> {
    vec![
        TarotCard::create_major(0, MajorArcana::Fool, "The spirit of Ether", None),
        TarotCard::create_major(1, MajorArcana::Magus, "The magus of power", None),
        TarotCard::create_major(
            2,
            MajorArcana::HighPriestess,
            "The princess of the silver star",
            None,
        ),
        TarotCard::create_major(3, MajorArcana::Empress, "Daughter of the mighty ones", None),
        TarotCard::create_major(
            4,
            MajorArcana::Emperor,
            "Son of the morning",
            Some(Zodiac::Aries),
        ),
        TarotCard::create_major(
            5,
            MajorArcana::Hierophant,
            "Magus of the eternal gods",
            Some(Zodiac::Taurus),
        ),
        TarotCard::create_major(
            6,
            MajorArcana::Lovers,
            "Children of the voice divine",
            Some(Zodiac::Gemini),
        ),
        TarotCard::create_major(
            7,
            MajorArcana::Chariot,
            "Lord of the Triumph of Light",
            Some(Zodiac::Cancer),
        ),
        TarotCard::create_major(
            8,
            MajorArcana::Adjustment,
            "Daughter of the Lord of Truth",
            Some(Zodiac::Libra),
        ),
        TarotCard::create_major(
            9,
            MajorArcana::Hermit,
            "Magus of the Voicee of Light",
            Some(Zodiac::Virgo),
        ),
        TarotCard::create_major(
            10,
            MajorArcana::WheelOfFortune,
            "Lord of the Forces of Life",
            None,
        ),
        TarotCard::create_major(
            11,
            MajorArcana::Lust,
            "Daughter of the flaming sword",
            Some(Zodiac::Leo),
        ),
        TarotCard::create_major(
            12,
            MajorArcana::HangedMan,
            "Spirit of the Mighty Waters",
            None,
        ),
        TarotCard::create_major(
            13,
            MajorArcana::Death,
            "Child of the Great Transformers",
            Some(Zodiac::Scorpio),
        ),
        TarotCard::create_major(
            14,
            MajorArcana::Art,
            "Daughter of the Reconcilers",
            Some(Zodiac::Sagitarius),
        ),
        TarotCard::create_major(
            15,
            MajorArcana::Devil,
            "Lord of the Gates of Matter",
            Some(Zodiac::Capricorn),
        ),
        TarotCard::create_major(
            16,
            MajorArcana::Tower,
            "Lord of the Hosts of the Mighty",
            None,
        ),
        TarotCard::create_major(
            17,
            MajorArcana::Star,
            "The Daughter of ther Firmament",
            Some(Zodiac::Aquarius),
        ),
        TarotCard::create_major(
            18,
            MajorArcana::Moon,
            "Ruler of Flux and Reflux",
            Some(Zodiac::Piscis),
        ),
        TarotCard::create_major(19, MajorArcana::Sun, "Lord of the Fire of the World", None),
        TarotCard::create_major(20, MajorArcana::Eon, "The spirit of the Primal Fire", None),
        TarotCard::create_major(
            21,
            MajorArcana::Universe,
            "The great One of the Night of Time",
            None,
        ),
    ]
}

pub async fn insert_major_arcana(graph: &Graph, cards: Vec<TarotCard>) -> Result<()> {
    for card in cards {
        let TarotCard {
            suit,
            number,
            name,
            title,
            ..
        } = card;

        let mut result = graph
            .execute(
                query("CREATE (a:TarotCard {name: $name, suit: $suit, number: $number, title: $title}) RETURN a")
                    .param("name", name.to_string())
                    .param("suit", suit.to_string())
                    .param("number", number)
                    .param("title", title.to_string())
            )
            .await?;

        if let Ok(row) = result.next().await {
            println!("{:?}", row.context("Failed to insert row!")?);
        }
    }
    Ok(())
}
