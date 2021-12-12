#![feature(type_alias_impl_trait)]

use std::time::Instant;

type Vertex = &'static str;
type Edge = (Vertex, Vertex);
type EdgeIterator<'a> = impl Iterator<Item = Edge>;

fn main() {
    let now = Instant::now();

    let input = include_str!("../input").trim_end().split('\n');
    let edges: Vec<Edge> = input
        .map(|l| l.split('-'))
        .map(|mut s| (s.next().unwrap(), s.next().unwrap()))
        .collect();

    let paths = dfs(&edges, &"start", &[]);

    println!("paths: {}", paths);
    assert_eq!(paths, 5958);

    let elapsed_time = now.elapsed();
    println!("done in {} microseconds", elapsed_time.as_micros())
}

fn edges_from_vertex<'a>(edges: &'a [Edge], vertex: &'a Vertex) -> EdgeIterator<'a> {
    let a = edges
        .iter()
        .filter(move |(u, _)| u == vertex)
        .map(|(u, v)| (*u, *v));
    let b = edges
        .iter()
        .filter(move |(_, v)| v == vertex)
        .map(|(u, v)| (*v, *u));
    a.chain(b)
}

fn dfs<'a>(edges: &'a [Edge], vertex: &'a Vertex, visited: &[Vertex]) -> u16 {
    let mut visited = visited.to_vec();
    let mut paths = 0u16;
    visited.push(vertex);
    if vertex == &"end" {
        // println!("{}", visited.join("-"));
        return paths + 1;
    }
    for (_, v) in edges_from_vertex(edges, vertex) {
        if v.chars().next().unwrap().is_lowercase() && visited.contains(&v) {
            continue;
        }
        paths += dfs(edges, &v, &visited);
    }
    paths
}
