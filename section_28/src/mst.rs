﻿use disjoint_sets::UnionFind;

type Node = usize;
type Weight = usize;

struct Edge {
    dest: Node,
    weight: Weight,
}

type Graph = Vec<Vec<Edge>>;

fn edges_by_weight(graph: &Graph) -> Vec<(Node, Node, Weight)> {
    let mut edges = vec![];

    for (src, dest) in graph.iter().enumerate() {
        for edge in dest {
            edges.push((src, edge.dest, edge.weight));
        }
    }

    edges.sort_by_key(|&(_, _, weight)| weight);
    edges
}

fn mst(graph: &Graph) -> Vec<(Node, Node)> {
    let mut result = vec![];
    let mut union_find = UnionFind::new(graph.len());

    for (src, dest, _) in edges_by_weight(graph) {
        if !union_find.equiv(src, dest) {
            union_find.union(src, dest);
            result.push((src, dest));
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mst() {
        let graph = vec![
            vec![Edge { dest: 1, weight: 3 }, Edge { dest: 3, weight: 6 }, Edge { dest: 5, weight: 2 }],
            vec![Edge { dest: 3, weight: 5 }, Edge { dest: 5, weight: 4 }, Edge { dest: 2, weight: 1 }],
            vec![Edge { dest: 3, weight: 2 }, Edge { dest: 4, weight: 3 }],
            vec![Edge { dest: 4, weight: 7 }],
            vec![Edge { dest: 5, weight: 2 }],
            vec![],
        ];

        assert_eq!(vec![(1, 2), (0, 5), (2, 3), (4, 5), (0, 1)], mst(&graph));
    }
}
