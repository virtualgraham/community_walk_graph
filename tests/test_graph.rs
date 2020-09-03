extern crate community_walk_graph;

use community_walk_graph::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;


#[test]
fn test_communities() {
    let mut graph = new_graph();
    add_edge(&mut graph,1,2);
    add_edge(&mut graph,2,3);
    add_edge(&mut graph,3,4);
    add_edge(&mut graph,4,5);
    add_edge(&mut graph,5,6);
    add_edge(&mut graph,6,7);
    add_edge(&mut graph,7,8);
    add_edge(&mut graph,8,9);
    add_edge(&mut graph,9,10);
    add_edge(&mut graph,10,11);
    add_edge(&mut graph,11,12);
    add_edge(&mut graph,12,13);
    add_edge(&mut graph,13,14);
    add_edge(&mut graph,14,15);
    add_edge(&mut graph,15,16);
    add_edge(&mut graph,16,17);
    add_edge(&mut graph,17,18);
    add_edge(&mut graph,18,19);
    add_edge(&mut graph,19,20);
    add_edge(&mut graph,20,1);

    //  len: usize, trials: usize, member_portion: usize
    let communities_result = communities(&graph, vec![1,2,3,4,5, 11,12,13,14,15], 5, 10000, 9900);
    println!("communities_result");
    for r in communities_result {
        print_community_result(&r)
    }
}


#[test]
fn test_graph() {
    let mut graph = new_graph();
    
    assert_eq!(len(&graph), 0);

    add_node(&mut graph, 1);
    assert_eq!(len(&graph), 1);

    add_edge(&mut graph,1,2);
    add_edge(&mut graph,2,3);
    add_edge(&mut graph,3,4);
    assert_eq!(len(&graph), 4);

    remove_node(&mut graph, 2);
    assert_eq!(len(&graph), 3);
    let n = neighbors(&graph, 3, 1);
    assert_eq!(n, vec![4]);

    remove_edge(&mut graph, 3, 4);
    assert_eq!(len(&graph), 3);
    let n = neighbors(&graph, 3, 1);
    assert_eq!(n, vec![]);

    add_edge(&mut graph,1,3);
    add_edge(&mut graph,3,4);
    add_edge(&mut graph,4,5);
    add_edge(&mut graph,5,6);
    assert_eq!(len(&graph), 5);

    let mut n = neighbors(&graph,1,2);
    n.sort();
    assert_eq!(n, vec![1,3,4]);

    let mut n = neighbors(&graph,1,3);
    n.sort();
    assert_eq!(n, vec![1,3,4,5]);

    let m = random_walk(&graph,4,1,10);
    print_random_walk_result(&m);

    let m = random_walk(&graph,4,2,100);
    print_random_walk_result(&m);

    let m = random_walk(&graph,4,4,1000);
    print_random_walk_result(&m);

    let c = community(&graph,4,2,100,50);
    print_community_result(&c);

    let c = community(&graph,4,4,1000,500);
    print_community_result(&c);


    let communities_result = communities(&graph, vec![1,2,3,4], 2, 10, 2);
    println!("communities_result");
    for r in communities_result {
        print_community_result(&r)
    }
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