mod function;
mod construct;
use function::betweenness_centrality;
use petgraph::prelude::*;
use petgraph::visit::IntoNodeIdentifiers;

fn main() {
    // Read the graph from the dataset using the Game module's construct_graph function
    let graph = construct::Game::construct_graph();

    // Collect node names for easier reference
    let node_names: Vec<_> = graph.node_identifiers().map(|node| graph.node_weight(node).unwrap().to_string()).collect();

    // Get the count of nodes in the graph
    let node_count = graph.node_count();

    // Calculate betweenness centrality for each node
    let initial_centrality: Vec<_> = (0..node_count)
        .map(|v_index| (v_index, betweenness_centrality(&graph, NodeIndex::new(v_index))[v_index]))
        .collect();

    // Sort nodes by their centrality scores in descending order
    let mut sorted_centrality = initial_centrality.clone();
    sorted_centrality.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Print the nodes and their corresponding centrality scores
    for (index, centrality_score) in sorted_centrality {
        println!("Node {}: Centrality {}", node_names[index], centrality_score);
    }
}

#[test]
fn test_sum() {
    // Create a test vector
    let test_vector = vec![1, 2, 3, 4, 5];

    // Calculate the expected sum
    let expected_sum = 15;

    // Calculate the actual sum using your function or any other sum function
    let actual_sum = calculate_sum(&test_vector);

    // Verify that the expected sum and actual sum are equal
    assert_eq!(actual_sum, expected_sum);
}

// Replace this function with your actual implementation
fn calculate_sum(vector: &Vec<i32>) -> i32 {
    vector.iter().sum()
}
