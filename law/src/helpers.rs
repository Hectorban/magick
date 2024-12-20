use neo4rs::{query, Graph};

pub async fn delete_graph(graph: &Graph) {
    graph
        .run(query("match (a) -[r] -> () delete a, r"))
        .await
        .unwrap();

    graph.run(query("match (a) delete a")).await.unwrap();
}
