#![feature(test)]
#![feature(type_alias_impl_trait)]

extern crate test;

type Vertex = &'static str;
type Edge = (Vertex, Vertex);
type EdgeIterator<'a> = impl Iterator<Item = Edge>;

fn main() {
    let (paths_part1, paths_part2) = run(include_str!("../input"));

    println!("paths part 1: {}", paths_part1);
    println!("paths part 2: {}", paths_part2);
}

fn run(input: &'static str) -> (u16, u32) {
    let input = input.trim_end().split('\n');
    let edges: Vec<Edge> = input
        .map(|l| l.split('-'))
        .map(|mut s| (s.next().unwrap(), s.next().unwrap()))
        .collect();

    let paths_part1 = dfs_part1(&edges, &"start", &[]);
    let paths_part2 = dfs_part2(&edges, &"start", &[], false);
    (paths_part1, paths_part2)
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

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[test]
    fn test_input_test1() {
        let (paths_part1, paths_part2) = run(include_str!("../input-test1"));
        assert_eq!(paths_part1, 10);
        assert_eq!(paths_part2, 36);
    }

    #[test]
    fn test_input_test2() {
        let (paths_part1, paths_part2) = run(include_str!("../input-test2"));
        assert_eq!(paths_part1, 19);
        assert_eq!(paths_part2, 103);
    }

    #[test]
    fn test_input_test3() {
        let (paths_part1, paths_part2) = run(include_str!("../input-test3"));
        assert_eq!(paths_part1, 226);
        assert_eq!(paths_part2, 3509);
    }

    #[test]
    fn test_input_own() {
        let (paths_part1, paths_part2) = run(include_str!("../input"));
        assert_eq!(paths_part1, 5958);
        assert_eq!(paths_part2, 150426);
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = include_str!("../input");
        b.iter(|| run(input));
    }
}
