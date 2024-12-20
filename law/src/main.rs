use eyre::Result;
use hebrew::{generate_hebrew_alphabet, insert_hebrew_alphabet};
use helpers::delete_graph;
use neo4rs::*;
use tarot::{generate_major_arcana, insert_major_arcana};
use tree_of_life::{Node, TreeOfLife};

pub mod alchemy;
pub mod angel;
pub mod chakra;
pub mod hebrew;
pub mod helpers;
pub mod planet;
pub mod tarot;
pub mod tree_of_life;
pub mod zodiac;

#[tokio::main]
async fn main() -> Result<()> {
    let graph = Graph::new("localhost:7687", "neo4j", "password")
        .await
        .unwrap();

    let tree_of_life = TreeOfLife::new();
    let alphabet = generate_hebrew_alphabet();
    let major_arcana = generate_major_arcana();

    delete_graph(&graph).await;

    tree_of_life.insert_nodes(&graph).await?;
    // TODO DEBUG
    //tree_of_life.flaming_sword(&graph).await?;

    insert_hebrew_alphabet(&graph, alphabet).await?;
    tree_of_life.insert_hebrew_paths(&graph).await?;

    insert_major_arcana(&graph, major_arcana).await?;
    tree_of_life.insert_arcana_paths(&graph).await?;

    Ok(())
}
