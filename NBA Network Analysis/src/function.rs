use petgraph::graph::{Graph, NodeIndex};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

type Distance = i64;

// Function to compute betweenness centrality using Dijkstra's algorithm
pub fn betweenness_centrality(graph: &Graph<String, i32>, start: NodeIndex) -> Vec<f64> {
    // Calculate the shortest paths from the start node
    let shortest_paths = compute_shortest_paths(graph, start);

    // Initialize a vector to store betweenness centrality scores
    let mut betweenness_centrality: Vec<f64> = vec![0.0; graph.node_count()];

    // Iterate over all nodes to count the number of shortest paths and update centrality scores
    for node in graph.node_indices() {
        let mut num_shortest = 0.0;

        // Check if there is a shortest path from the start node to the current node
        if let Some(node_dist) = shortest_paths[node.index()] {
            // Store the centrality value of the current node outside the loop
            let node_centrality = betweenness_centrality[node.index()];

            // Iterate over incoming neighbors
            for incoming_neighbor in get_incoming_neighbors(graph, node) {
                let neighbor_index = incoming_neighbor.index();
                let neighbor_dist = shortest_paths[neighbor_index];

                // Check if there is a shortest path from the neighbor to the current node
                if let Some(dist) = neighbor_dist {
                    let edge = graph.find_edge(incoming_neighbor, node).unwrap();
                    let weight = *graph.edge_weight(edge).unwrap();

                    // Check if the distance is equal to the shortest path
                    if dist + weight as i64 == node_dist {
                        num_shortest += 1.0;

                        // Update centrality scores
                        let neighbor_centrality = &mut betweenness_centrality[neighbor_index];
                        *neighbor_centrality += (1.0 + node_centrality) / num_shortest;
                    }
                }
            }
        }

        // Divide by 2 because each shortest path is counted twice
        if node != start {
            betweenness_centrality[node.index()] *= 0.5;
        }
    }

    // Return the final betweenness centrality scores
    betweenness_centrality
}

// Function to find the shortest paths from the start node
fn compute_shortest_paths(graph: &Graph<String, i32>, start: NodeIndex) -> Vec<Option<Distance>> {
    // Initialize a vector to store the shortest distances
    let mut shortest_distances: Vec<Option<Distance>> = vec![None; graph.node_count()];
    shortest_distances[start.index()] = Some(0);

    // Initialize a priority queue to track nodes to visit, with the starting node having the highest priority
    let mut priority_queue = BinaryHeap::<Reverse<(Distance, NodeIndex)>>::new();
    priority_queue.push(Reverse((0, start)));

    // Visit all nodes and check for shorter paths
    while let Some(Reverse((dist, current_node))) = priority_queue.pop() {
        if let Some(current_dist) = shortest_distances[current_node.index()] {
            if dist > current_dist {
                continue;
            }
        }

        // Loop over all neighbors to calculate the distance to the neighbor
        for outgoing_neighbor in get_outgoing_neighbors(graph, current_node) {
            let edge_distance = *graph.edge_weight(graph.find_edge(current_node, outgoing_neighbor).unwrap()).unwrap();
            let new_distance = dist + edge_distance as i64;

            // Check if the new distance is shorter than the current shortest distance
            let update_distance = match shortest_distances[outgoing_neighbor.index()] {
                None => true,
                Some(d) => new_distance < d,
            };

            // If the distance is shorter, update the shortest distance and add the neighbor to the priority queue
            if update_distance {
                shortest_distances[outgoing_neighbor.index()] = Some(new_distance);
                priority_queue.push(Reverse((new_distance, outgoing_neighbor)));
            }
        }
    }

    // Return the vector of shortest distances
    shortest_distances
}

// Function to get incoming neighbors of a node
fn get_incoming_neighbors(graph: &Graph<String, i32>, node: NodeIndex) -> Vec<NodeIndex> {
    graph.neighbors_directed(node, petgraph::Direction::Incoming).collect()
}

// Function to get outgoing neighbors of a node
fn get_outgoing_neighbors(graph: &Graph<String, i32>, node: NodeIndex) -> Vec<NodeIndex> {
    graph.neighbors_directed(node, petgraph::Direction::Outgoing).collect()
}
