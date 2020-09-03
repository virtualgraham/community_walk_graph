# Community Walk Graph

An in-memory undirected graph for finding communities with random walks

Written in Rust using pyo3 for use in Python

## Build and Install
```
python -m pip install maturin
maturin build --release
python -m pip install <built_whl_file>
```

## Sample Usage
```python
import community_walk_graph as cwg

# Build a ring graph with 20 nodes
graph = cwg.new_graph()
cwg.add_edge(graph, 1, 2)
cwg.add_edge(graph, 2, 3)
cwg.add_edge(graph, 3, 4)
cwg.add_edge(graph, 4, 5)
cwg.add_edge(graph, 5, 6)
cwg.add_edge(graph, 6, 7)
cwg.add_edge(graph, 7, 8)
cwg.add_edge(graph, 8, 9)
cwg.add_edge(graph, 9, 10)
cwg.add_edge(graph, 10, 11)
cwg.add_edge(graph, 11, 12)
cwg.add_edge(graph, 12, 13)
cwg.add_edge(graph, 13, 14)
cwg.add_edge(graph, 14, 15)
cwg.add_edge(graph, 15, 16)
cwg.add_edge(graph, 16, 17)
cwg.add_edge(graph, 17, 18)
cwg.add_edge(graph, 18, 19)
cwg.add_edge(graph, 19, 20)
cwg.add_edge(graph, 20, 1)

# Query 5 nodes from each side of the ring
communities_result = cwg.communities(graph, [1,2,3,4,5, 11,12,13,14,15], 7, 100, 50)
print(communities_result)

# Returns the communities of each of the queried nodes
# [[19, 2, 20, 3, 1], [20, 3, 1, 5, 4, 2], [2, 5, 6, 20, 3, 4, 1], [3, 2, 5, 4, 6], [6, 4, 5, 2, 7, 3], [11, 10, 12, 13, 9], [14, 11, 15, 12, 10, 13], [12, 13, 15, 11, 14], [13, 14, 16, 12, 15], [14, 13, 17, 15, 12, 16]]
```

## Parameters

`communities(graph, node_ids, len, trials, member_portion)`

- `graph`: The graph created by `cwg.new_graph()`
- `node_ids`: A list of positive integer node ids 
- `len`: The length of each random walk. Larger lengths find larger communities
- `trials`: The number of random walks to run per node. More trials leads to more consistent results.
- `member_portion`: An integer between `0` and `trials`. Nodes are only included in community if they appear in `member_portion` or more number of random walks. High `member_portion` leads to smaller more consistent communities, and larger `member_portion` leads to larger more variable communities.