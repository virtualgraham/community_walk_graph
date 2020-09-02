extern crate community_walk_graph;

use community_walk_graph::*;
use std::collections::HashMap;

#[test]
fn test_graph() {
    let mut graph = Graph::new();
    assert_eq!(graph.len(), 0);

    graph.add_node(1);
    assert_eq!(graph.len(), 1);

    graph.add_edge(1,2);
    graph.add_edge(2,3);
    graph.add_edge(3,4);
    assert_eq!(graph.len(), 4);

    graph.remove_node(&2);
    assert_eq!(graph.len(), 3);
    let n = graph.neighbors(&3, &1);
    assert_eq!(n, vec![4]);

    graph.remove_edge(&3, &4);
    assert_eq!(graph.len(), 3);
    let n = graph.neighbors(&3, &1);
    assert_eq!(n, vec![]);

    graph.add_edge(1,3);
    graph.add_edge(3,4);
    graph.add_edge(4,5);
    graph.add_edge(5,6);
    assert_eq!(graph.len(), 5);

    let mut n = graph.neighbors(&1, &2);
    n.sort();
    assert_eq!(n, vec![1,3,4]);

    let mut n = graph.neighbors(&1, &3);
    n.sort();
    assert_eq!(n, vec![1,3,4,5]);

    let m = graph.random_walk(&4, &1, &10);
    print_random_walk_result(&m);

    let m = graph.random_walk(&4, &2, &100);
    print_random_walk_result(&m);

    let m = graph.random_walk(&4, &4, &1000);
    print_random_walk_result(&m);

    let c = graph.community(&4, &2, &100, &50);
    print_community_result(&c);

    let c = graph.community(&4, &4, &1000, &500);
    print_community_result(&c);
}


fn print_random_walk_result(m: &HashMap<u64, usize>) {
    for (k, v) in m {
        print!("({}: {}) ", k, v);
    }
    println!("");
}

fn print_community_result(c: &Vec<u64>) {
    for v in c {
        print!("{} ", v);
    }
    println!("");
}