use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::{VisitMap, Visitable};
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;

pub struct GraphDistances {
    pub graph: UnGraph<String, ()>,
    pub avg_distance: f64,
    pub maximum_distance: usize,
    pub median_distance: f64,
    pub mode_distance: usize,
}

impl GraphDistances {
    pub fn new(graph: UnGraph<String, ()>) -> Result<GraphDistances, Box<dyn Error>> {
        let avg_distance = compute_average(&graph);
        let maximum_distance = compute_maximum(&graph);
        let median_distance = compute_median(&graph)?;
        let mode_distance = compute_mode(&graph)?;
        
        Ok(GraphDistances {
            graph,
            avg_distance,
            maximum_distance,
            median_distance,
            mode_distance,
        })
    }
}

// reads file and creates graph 
pub fn file_graph(path: &str) -> Result<UnGraph<String, ()>, Box<dyn Error>> {
    let file = File::open(path).expect("Could not open file");
    let buf_reader = BufReader::new(file);
    let mut graph = UnGraph::<String, ()>::new_undirected();
    let mut map: HashMap<String, NodeIndex> = HashMap::new();

    for line in buf_reader.lines() {
        let line = line.expect("Error");
        let mut iter = line.split(',');

        let kdrama_name = iter.next().expect("Error").to_string();
        let actor = iter.next().expect("Error").to_string();

        let drama = *map.entry(kdrama_name.clone()).or_insert_with(|| graph.add_node(kdrama_name));
        let actor = *map.entry(actor.clone()).or_insert_with(|| graph.add_node(actor));
        graph.add_edge(drama, actor, ());
    }

    Ok(graph)
}

// computes all distances
pub fn compute_distances(graph: &UnGraph<String, ()>, start: NodeIndex) -> Vec<usize> {
    let mut visited = graph.visit_map();
    let mut queue = VecDeque::new();
    visited.visit(start);
    queue.push_back((start, 0));

    let mut distances = Vec::new();
    while let Some((node, dist)) = queue.pop_front() {
        for w in graph.neighbors(node) {
            if !visited.is_visited(&w) {
                visited.visit(w);
                queue.push_back((w, dist + 1));
                distances.push(dist + 1);
            }
        }
    }
    distances
}

// computes the average of all distances
pub fn compute_average(graph: &UnGraph<String, ()>) -> f64 {
    let mut total_distance = 0;
    let mut total_paths = 0;

    for node in graph.node_indices() {
        let distances = compute_distances(graph, node);
        total_paths += distances.len();
        total_distance += distances.iter().sum::<usize>();
    }

    if total_paths == 0 {
        return 0.0; 
    }

    total_distance as f64 / total_paths as f64
}

// computes the maximum of all distances
pub fn compute_maximum(graph: &UnGraph<String, ()>) -> usize {
    let mut max_distance = 0;

    for node in graph.node_indices() {
        let distances = compute_distances(graph, node);
        if let Some(&max) = distances.iter().max() {
            if max > max_distance {
                max_distance = max;
            }
        }
    }

    max_distance
}

// computes median of all distances
pub fn compute_median(graph: &UnGraph<String, ()>) -> Result<f64, Box<dyn Error>> {
    let mut all_distances = Vec::new();

    for node in graph.node_indices() {
        let distances = compute_distances(graph, node);
        all_distances.extend(distances);
    }

    if all_distances.is_empty() {
        return Ok(0.0); 
    }

    all_distances.sort();

    let mid_val = all_distances.len() / 2;
    if all_distances.len() % 2 == 0 {
        Ok((all_distances[mid_val - 1] + all_distances[mid_val]) as f64 / 2.0)
    } else {
        Ok(all_distances[mid_val] as f64)
    }
}

// computes mode of all distances
pub fn compute_mode(graph: &UnGraph<String, ()>) -> Result<usize, Box<dyn Error>> {
    let mut times = HashMap::new();
    for start in graph.node_indices() {
        let distances = compute_distances(graph, start);
        for dist in distances {
            *times.entry(dist).or_insert(0) += 1;
        }
    }

    let mut mode_distance = 0;
    let mut max_count = 0;
    for (dist, count) in times {
        if count > max_count {
            mode_distance = dist;
            max_count = count;
        }
    }

    Ok(mode_distance)
}