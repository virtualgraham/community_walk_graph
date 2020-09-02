use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::HashMap;
use rand::Rng;
use rand::thread_rng;

use dashmap::DashMap;
use rayon::prelude::*;

#[pyclass]
pub struct Graph {
    pub nodes: BTreeMap<u64, Vec<u64>>
}


#[pyfunction]
pub fn new_graph() -> Graph {
    Graph {
        nodes: BTreeMap::new()
    }
}

#[pyfunction]
pub fn len(graph: &Graph) -> usize {
    graph.nodes.len()
}

#[pyfunction]
pub fn add_node(graph: &mut Graph, node_id: u64) {
    graph.nodes.insert(node_id, Vec::new());
}

#[pyfunction]
pub fn add_edge(graph: &mut Graph, node_id_a: u64, node_id_b: u64) {
    if !graph.nodes.contains_key(&node_id_a) {
        add_node(graph, node_id_a.clone())
    }
    if !graph.nodes.contains_key(&node_id_b) {
        add_node(graph, node_id_b.clone())
    }
    
    let edge_list_a = graph.nodes.get_mut(&node_id_a).unwrap();
    if !edge_list_a.contains(&node_id_b) {
        edge_list_a.push(node_id_b)
    }

    let edge_list_b = graph.nodes.get_mut(&node_id_b).unwrap();
    if !edge_list_b.contains(&node_id_a) {
        edge_list_b.push(node_id_a)
    }
}

#[pyfunction]
pub fn remove_node(graph: &mut Graph, node_id: u64) {
    if let Some(edge_list) = graph.nodes.remove(&node_id) {

        // for each node_id_b in edge_list remove node_id from that node_id_b's edge_list
        for node_id_b in edge_list {
            if let Some(edge_list_b) = graph.nodes.get_mut(&node_id_b) {
                if let Some(i) = edge_list_b.iter().position(|&n| n == node_id) {
                    edge_list_b.remove(i);
                }
            }
        } 

    }
}

#[pyfunction]
pub fn remove_edge(graph: &mut Graph, node_id_a: u64, node_id_b: u64) {
    if let Some(edge_list_a) = graph.nodes.get_mut(&node_id_a) {
        if let Some(i) = edge_list_a.iter().position(|&n| n == node_id_b) {
            edge_list_a.remove(i);
        }
    }
    if let Some(edge_list_b) = graph.nodes.get_mut(&node_id_b) {
        if let Some(i) = edge_list_b.iter().position(|&n| n == node_id_a) {
            edge_list_b.remove(i);
        }
    }
}

#[pyfunction]
pub fn neighbors(graph: &Graph, node_id: u64, radius: usize) -> Vec<u64> {
    if radius == 1 {
        return match graph.nodes.get(&node_id) {
            Some(n) => n.iter().cloned().collect(),
            None => Vec::new()
        }
    }

    if radius == 0 {
        return Vec::new()
    }

    return neighbors_recursive(graph, &node_id, &radius, HashSet::new()).iter().cloned().collect()
}

fn neighbors_recursive(graph: &Graph, node_id: &u64, radius: &usize, path: HashSet<u64>) -> HashSet<u64> {
    let mut result:HashSet<u64> = HashSet::new();
    if let Some(edge_list) = graph.nodes.get(node_id) {
        for node_id_b in edge_list {
            if path.contains(&node_id_b) {
                continue
            }
            result.insert(node_id_b.clone());
            if path.len() + 1 < *radius {
                let mut next_path = path.clone();
                next_path.insert(node_id_b.clone());
                result.extend(&neighbors_recursive(graph, &node_id_b, radius, next_path));
            }
        }
    }
    return result
}

#[pyfunction]
pub fn community(graph: &Graph, node_id: u64, len: usize, trials: usize, member_portion: usize) -> Vec<u64> {
    let visited = random_walk(graph, node_id, len, trials);
    visited.iter().filter(|x| *x.1 > member_portion).map(|x| x.0.clone()).collect()
}

#[pyfunction]
pub fn random_walk(graph: &Graph, node_id: u64, len: usize, trials: usize) -> HashMap<u64, usize> {
    let visited = DashMap::new();

    (0..trials).into_par_iter().for_each(|_| {
        let mut rng = thread_rng();
        let mut cur = node_id.clone();
        for _ in 0..len {
            if let Some(nei) = graph.nodes.get(&cur) {
                if !nei.is_empty() {
                    cur = nei[rng.gen_range(0, nei.len())].clone();
                    if visited.contains_key(&cur) {
                        visited.alter(&cur, |_, v| v + 1);
                    } else {
                        visited.insert(cur, 1);
                    }
                    continue
                }
            }
            break
        }
    });

    visited.iter().map(|x| (*x.key(), *x.value())).collect()
}

#[pymodule]
fn community_walk_graph(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Graph>()?;
    m.add_wrapped(wrap_pyfunction!(new_graph)).unwrap();
    m.add_wrapped(wrap_pyfunction!(len)).unwrap();
    m.add_wrapped(wrap_pyfunction!(add_node)).unwrap();
    m.add_wrapped(wrap_pyfunction!(add_edge)).unwrap();
    m.add_wrapped(wrap_pyfunction!(remove_node)).unwrap();
    m.add_wrapped(wrap_pyfunction!(remove_edge)).unwrap();
    m.add_wrapped(wrap_pyfunction!(neighbors)).unwrap();
    m.add_wrapped(wrap_pyfunction!(community)).unwrap();
    m.add_wrapped(wrap_pyfunction!(random_walk)).unwrap();
    Ok(())
}



