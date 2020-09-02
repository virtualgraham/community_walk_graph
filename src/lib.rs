pub struct Graph {
    pub nodes: BTreeMap<u64, Vec<u64>>
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: BTreeMap::new()
        }
    }

    pub fn len(self: &Self) -> usize {

    }

    pub fn add_node(self: &mut Self, node_id: u64) {

    }

    pub fn add_edge(self: &mut Self, node_id_a: u64, node_id_b: u64) {
        // if node_id_a does not exist, create it
        // if node_id_b does not exist, create it
        // add node_id_a to edge_list of node_id_b ensuring no duplicates
        // add node_id_b to edge_list of node_id_a ensuring no duplicates
    }

    pub fn remove_node(self: &mut Self, node_id: u64) {
        // get edge_list for node_id
        // delete node
        // for each node_id in edge_list remove node_id from that node's edge_list
    }

    pub fn remove_edge(self: &mut Self, node_id_a: u64, node_id_b: u64) {
        // remove node_id_a from edge_list of node_id_b
        // remove node_id_b from edge_list of node_id_a
    }

    pub fn neighbors(self: &Self, radius: u32) -> Vec<u64> {

    }

    pub fn community(self: &Self, len: u32, trials: u32, member_portion: u32) -> Vec<u64> {
        
    }
}

