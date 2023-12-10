use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

// Notes/thoughts: create a directed graph data structure, put all Vertexes and Indices into the the graph
// use BFS to find the the shortest path
//
// Store all lines into a vec then when you need use directions to find zzz

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut result = 0;
    let graph = DirectedGraph::new();

    for line in reader.lines() {
        let new_line = line.unwrap();
        let nodes: Vec<&str> = new_line.split(" = ").collect();
        let root = nodes[0];
        let children = nodes[1].split(", ").collect();
        graph.addEdge(root.clone(), children[0]);
        graph.addEdge(root, children[1]);
    }

    println!("{}", );

    Ok(())
}   



// Directed Graph Data Structure
struct DirectedGraph {
    V: i32,
    E: i32,
    adj: HashMap<&str, Vec<&str>>,
}

// Directed Graph Methods
impl DirectedGraph {
    fn new(&mut self) {
        self.adj = HashMap::new();
        self.V = 0;
        self.E = 0;
    }

    fn addEdge(&mut self, v: &str, w: &str) {
        if self.adj.contains_key(&v) { 
            let mut new_vec: Vec<&str> = self.adj[&v].clone();
            new_vec.push(w); 
            self.adj.insert(v, new_vec);
        }
        else {
            self.adj.insert(v, vec![w]);
        }
    }
    
    fn find_shortest_path(&self, v: &str, w: &str) {

    }
}
