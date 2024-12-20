use eyre::{ContextCompat, Result};
use neo4rs::{query, Graph};
use paths::generate_paths;
use sefira::create_nodes;
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

pub mod paths;
pub mod sefira;

use crate::{
    alchemy::Element,
    planet::{self, Planet},
    tarot::MajorArcana,
};

#[derive(Clone, Debug)]
pub struct Node {
    name: Sefira,
    meaning: String,
    planet: Planet,
    column: Column,
    realm: Realm,
    number: i32,
}

impl Node {
    pub fn new(
        name: Sefira,
        meaning: String,
        planet: Planet,
        column: Column,
        realm: Realm,
        number: i32,
    ) -> Self {
        Self {
            name,
            meaning,
            planet,
            column,
            realm,
            number,
        }
    }
}

#[derive(EnumString, Debug, Display, Clone, EnumIter)]
pub enum Sefira {
    Keter,
    Chokmah,
    Binah,
    Chesed,
    Gevurah,
    Tiferet,
    Netsach,
    Hod,
    Yesod,
    Malkuth,
}

#[derive(EnumString, Debug, Display, Clone)]
pub enum Realm {
    Atziluth,
    Beriah,
    Yetzirah,
    Assiah,
}

#[derive(EnumString, Debug, Display, Clone)]
pub enum Column {
    Mercy,
    Severity,
    Balance,
}

pub struct MyteriesPath {
    hebrew: char,
    arcana: MajorArcana,
    from: Sefira,
    to: Sefira,
}

impl MyteriesPath {
    pub fn new(hebrew: char, arcana: MajorArcana, from: Sefira, to: Sefira) -> Self {
        Self {
            hebrew,
            arcana,
            from,
            to,
        }
    }
}

pub struct TreeOfLife {
    nodes: Vec<Node>,
    mysteries: Vec<MyteriesPath>,
}

impl Default for TreeOfLife {
    fn default() -> Self {
        Self::new()
    }
}

impl TreeOfLife {
    pub fn new() -> Self {
        let nodes = create_nodes();
        let mysteries = generate_paths();
        Self { nodes, mysteries }
    }

    pub async fn insert_nodes(&self, graph: &Graph) -> Result<()> {
        for node in &self.nodes {
            let Node {
                name,
                meaning,
                column,
                realm,
                ..
            } = node;

            let mut result = graph
                .execute(
                    query("CREATE (a:Sefira {name: $name, meaning: $meaning, realm: $realm, column: $column}) RETURN a")
                        .param("name", name.to_string())
                        .param("column", column.to_string())
                        .param("realm", realm.to_string())
                        .param("meaning", meaning.to_string()),
                )
                .await?;

            if let Ok(row) = result.next().await {
                println!("{:?}", row.context("Failed to insert row!")?);
            }
        }

        Ok(())
    }

    pub async fn insert_hebrew_paths(&self, graph: &Graph) -> Result<()> {
        for path in &self.mysteries {
            let MyteriesPath {
                hebrew, from, to, ..
            } = path;

            let mut result = graph
                .execute(
                    query(
                        "MATCH (a:Sefira {name: $from}), (b:Hebrew {letter: $letter}), (c:Sefira {name: $to}), (c) 
                         CREATE (a)-[:IS_MYSTERY]->(b)-[:IS_MYSTERY]->(c) RETURN b",
                    )
                    .param("from", from.to_string())
                    .param("to", to.to_string())
                    .param("letter", hebrew.to_string()),
                )
                .await?;

            if let Ok(row) = result.next().await {
                println!("{:?}", row.context("Failed to insert row!")?);
            }
        }
        Ok(())
    }

    pub async fn insert_arcana_paths(&self, graph: &Graph) -> Result<()> {
        for path in &self.mysteries {
            let MyteriesPath {
                arcana, from, to, ..
            } = path;

            let mut result = graph
                .execute(
                    query(
                        "MATCH (a:Sefira {name: $from}), (b:TarotCard {name: $arcana}), (c:Sefira {name: $to}), (c) 
                         CREATE (a)-[:IS_ARCANA]->(b)-[:IS_ARCANA]->(c) RETURN b",
                    )
                    .param("from", from.to_string())
                    .param("to", to.to_string())
                    .param("arcana", arcana.to_string()),
                )
                .await?;

            if let Ok(row) = result.next().await {
                println!("{:?}", row.context("Failed to insert row!")?);
            }
        }
        Ok(())
    }

    pub async fn flaming_sword(&self, graph: &Graph) -> Result<()> {
        let s_nodes = self.nodes.clone();
        for (i, node) in self.nodes.iter().enumerate() {
            if i == 9 {
                return Ok(());
            }

            println!("FLAMING {:?} to {:?}", node, s_nodes[i + 1]);

            let mut result = graph
                .execute(
                    query(
                        "MATCH (a {name: $from}), (b {name: $to}), (c) CREATE (a)-[:DESCENDS]->(b) RETURN b",
                    )
                    .param("from", node.name.to_string())
                    .param("to", s_nodes[i + 1].name.to_string()),
                )
                .await?;

            if let Ok(row) = result.next().await {
                println!("{:?}", row.context("Failed to insert row!")?);
            }
        }

        Ok(())
    }
}
