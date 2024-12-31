use csv::ReaderBuilder;
use petgraph::graph::{DiGraph, NodeIndex};
use serde::Deserialize;
use std::{fs::File, io::{self, BufReader}};

#[derive(Deserialize)]
pub struct Game {
    #[serde(rename = "HOME_TEAM_ID")]
    home_team_id: String,
    #[serde(rename = "VISITOR_TEAM_ID")]
    visitor_team_id: String,
    #[serde(rename = "HOME_TEAM_WINS")]
    home_team_wins: i32,
}

impl Game {
    // Construct a directed graph representing game outcomes
    pub fn construct_graph() -> DiGraph<String, i32> {
        let mut graph: DiGraph<String, i32> = DiGraph::new();

        // Read and validate game data
        let games = read_data();
        let validated_games = validate_data(games);

        // Iterate over validated games and build the graph
        for game in &validated_games {
            let home_team = &game.home_team_id;
            let visitor_team = &game.visitor_team_id;

            // Ensure nodes exist or add them
            let home_index = add_node_if_not_exists(home_team, &mut graph);
            let visitor_index = add_node_if_not_exists(visitor_team, &mut graph);

            // Add directed edge representing the game outcome
            let weight = game.home_team_wins;
            graph.add_edge(home_index, visitor_index, weight);
        }

        graph
    }
}

// Read game data from a CSV file
pub fn read_data() -> Vec<Game> {
    let file_result = File::open("games.csv");

    match file_result {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

            // Deserialize records, filter out errors, and collect into a vector
            csv_reader
                .deserialize::<Game>()
                .filter_map(|result| result.ok())
                .collect()
        }
        Err(_) => Vec::new(), // Handle file opening errors
    }
}

// Validate game data by removing games with invalid outcomes
pub fn validate_data(mut games: Vec<Game>) -> Vec<Game> {
    games.retain(|game| game.home_team_wins == 0 || game.home_team_wins == 1);
    games
}

// Add a node to the graph if it doesn't already exist
fn add_node_if_not_exists(node_id: &str, graph: &mut DiGraph<String, i32>) -> NodeIndex {
    // Find the node index with the given ID or add a new node
    graph.node_indices().find(|&idx| graph[idx] == node_id).unwrap_or_else(|| {
        let index = graph.add_node(node_id.to_string());
        index
    })
}
