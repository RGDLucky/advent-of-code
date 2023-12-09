use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

// Notes/thoughts: create a directed graph data structure, put all Vertexes and Indices into the the graph
// use BFS to find the the shortest path

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut result = 0;

    for line in reader.lines() {
        println!("TODO");
    }

    println!("{}", result);

    Ok(())
}   



// Directed Graph Data Structure
struct DirectedGraph {
    V: i32,
    E: i32,
    adj: HashMap<i32, Vec<i32>>,
}

// Directed Graph Methods
impl DirectedGraph {
    fn new(&mut self) {
        self.adj = HashMap::new();
        self.V = 0;
        self.E = 0;
    }

    fn addEdge(&mut self, v: i32, w: i32) {
        if self.adj.contains_key(&v) { self.adj[&v].push(w); }
        else {
            self.adj.insert(v, vec![w]);
        }
    }
    
    fn find_shortest_path(&self, v: i32, w: i32) {

    }
}
