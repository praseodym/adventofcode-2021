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

    let paths_part1 = dfs_part1(&edges, &"start", &[]);
    let paths_part2 = dfs_part2(&edges, &"start", &[], false);

    println!("paths part 1: {}", paths_part1);
    println!("paths part 2: {}", paths_part2);

    assert_eq!(paths_part1, 5958);
    assert_eq!(paths_part2, 150426);

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

fn dfs_part1<'a>(edges: &'a [Edge], vertex: &'a Vertex, visited: &[Vertex]) -> u16 {
    let mut visited = visited.to_vec();
    let mut paths = 0u16;
    visited.push(vertex);
    if vertex == &"end" {
        // println!("{}", visited.join(","));
        return paths + 1;
    }
    for (_, v) in edges_from_vertex(edges, vertex) {
        if v.chars().next().unwrap().is_lowercase() && visited.contains(&v) {
            continue;
        }
        paths += dfs_part1(edges, &v, &visited);
    }
    paths
}

fn dfs_part2<'a>(
    edges: &'a [Edge],
    vertex: &'a Vertex,
    visited: &[Vertex],
    small_cave_twice: bool,
) -> u32 {
    let mut visited = visited.to_vec();
    let mut paths = 0u32;
    visited.push(vertex);
    if vertex == &"end" {
        // println!("{}", visited.join(","));
        return paths + 1;
    }
    for (_, v) in edges_from_vertex(edges, vertex) {
        if v == "start" {
            continue;
        }
        let small_cave_once = v.chars().next().unwrap().is_lowercase() && visited.contains(&v);
        if small_cave_once && small_cave_twice {
            continue;
        }
        paths += dfs_part2(edges, &v, &visited, small_cave_once || small_cave_twice);
    }
    paths
}
