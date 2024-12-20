use eyre::{ContextCompat, Result};
use neo4rs::{query, Graph};

pub struct HebrewLetter {
    letter: char,
    meaning: String,
    pronuntiation: String,
}

impl HebrewLetter {
    pub fn new(letter: char, meaning: &str, pronuntiation: &str) -> Self {
        Self {
            letter,
            meaning: meaning.to_string(),
            pronuntiation: pronuntiation.to_string(),
        }
    }
}

pub fn generate_hebrew_alphabet() -> Vec<HebrewLetter> {
    vec![
        HebrewLetter::new('א', "Ox", "Aleph"),
        HebrewLetter::new('ב', "House", "Bet"),
        HebrewLetter::new('ג', "Camel", "Gimel"),
        HebrewLetter::new('ד', "Door", "Dalet"),
        HebrewLetter::new('ה', "Window", "He"),
        HebrewLetter::new('ו', "Nail", "Vav"),
        HebrewLetter::new('ז', "Sword", "Zayin"),
        HebrewLetter::new('ח', "Fence", "Chet"),
        HebrewLetter::new('ט', "Serpent", "Tet"),
        HebrewLetter::new('י', "Hand", "Yod"),
        HebrewLetter::new('כ', "Closed hand", "Kaf"),
        HebrewLetter::new('ל', "Ox goad", "Lamed"),
        HebrewLetter::new('מ', "Water", "Mem"),
        HebrewLetter::new('נ', "Fish", "Nun"),
        HebrewLetter::new('ס', "Prop", "Samech"),
        HebrewLetter::new('ע', "Eye", "Ayin"),
        HebrewLetter::new('פ', "Mouth", "Pe"),
        HebrewLetter::new('צ', "Fish hook", "Tsadi"),
        HebrewLetter::new('ק', "Back of head", "Qof"),
        HebrewLetter::new('ר', "Head", "Resh"),
        HebrewLetter::new('ש', "Tooth", "Shin"),
        HebrewLetter::new('ת', "Cross", "Tav"),
    ]
}

pub async fn insert_hebrew_alphabet(graph: &Graph, alphabet: Vec<HebrewLetter>) -> Result<()> {
    for letter in alphabet {
        let HebrewLetter {
            letter,
            meaning,
            pronuntiation,
        } = letter;

        let mut result = graph
            .execute(
                query("CREATE (a:Hebrew {letter: $letter, meaning: $meaning, pronuntiation: $pronuntiation}) RETURN a")
                    .param("letter", letter.to_string())
                    .param("pronuntiation", pronuntiation.to_string())
                    .param("meaning", meaning.to_string()),
            )
            .await?;

        if let Ok(row) = result.next().await {
            println!("{:?}", row.context("Failed to insert row!")?);
        }
    }

    Ok(())
}
