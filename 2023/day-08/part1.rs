use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut result = 0;
    let mut graph = DirectedGraph::new();
    let mut directions: String = String::from("");
    let mut contents: Vec<String> = vec![];
    
    for line in reader.lines() { contents.push(line.unwrap()); }

    let mut index = 0;
    while index < contents.len() {
        if contents[index] == "" { index += 1; break; }
        directions = directions + &contents[index];
        index += 1;
    }

     while index < contents.len() {
        let nodes: Vec<&str> = contents[index].split(" = ").collect();
        let root = nodes[0];
        let children: Vec<&str> = nodes[1].split(", ").collect();
        graph.add_vertex(root, &children[0][1..], &children[1][..3]);
        index += 1;
    }

    let mut curr = "AAA";
    let mut found = false;

    loop {
        for direction in directions.chars() {
            if curr == "ZZZ" { found = true; break; }
            result += 1;
            curr = graph.get_child(direction.clone(), curr);
        }

        if found { break; }
    }

    println!("{}", result);

    Ok(())
}   



// Directed Graph Data Structure
struct DirectedGraph<'a> {
    v: i32,
    e: i32,
    adj: HashMap<&'a str, Vec<&'a str>>,
}

// Directed Graph Methods
impl<'a> DirectedGraph<'a> {
    fn new() -> Self {
        Self { 
            v: 0,
            e: 0,
            adj: HashMap::new(),
        }
    }

    fn add_vertex(&mut self, v: &'a str, child1: &'a str, child2: &'a str) {
        self.adj.insert(v, vec![child1, child2]);
        self.v += 1;
        self.e += 1;
    }

    fn get_child(&self, direction: char, node: &str) -> &str {
        let curr_adj = &self.adj[node];
        if direction == 'R' { return curr_adj[1]; }
        return curr_adj[0];
    }
}
