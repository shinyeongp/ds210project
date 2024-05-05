use std::error::Error;
mod module;
use module::*;

fn main() -> Result<(), Box<dyn Error>> {
    let graph = file_graph("actors.csv")?;
    let graph_distance = GraphDistances::new(graph)?;

    println!("Average distance: {:?}", graph_distance.avg_distance);
    println!("Maximum distance: {:?}", graph_distance.maximum_distance);
    println!("Median distance: {:?}", graph_distance.median_distance);
    println!("Mode distance: {:?}", graph_distance.mode_distance);

    Ok(())
}

#[test]
fn test() {
let graph = file_graph("actors.csv");
let graph_distance = GraphDistances::new(graph.expect("REASON"));
let mode = graph_distance.unwrap().mode_distance;
assert_eq!(mode,6);
}