# What do I remember ?
1. Tree with cycles are graphs =) ; Graph without cycles are trees!!!
2. BFS / DFS should work but visited nodes must be accounted for.
3. Dijkstra for shortest path; A+ even better for shortest path.
4. Applicable in networks. 
5. Could be (un-)directed and or (un-)weighted. and or (a)cyclic

# 
Edges - connections
Vertex - node

DAG - directed acyclic graph
 - prerequisites
 - build system
 - task scheduling 

# how do you encode graps
Are these two vertices connected ?
with 2d array (Adjacency Matrix)
1 are connected or weight  
0 not connected 
i .... j
. 1
. 0 
.
j

time  O(1) look up
space O(Vertex^2) space


Who are these vertices neighbours ?
More common:
Adjacency list
store all neighbours in vector/array?

time O(Edges) - iterate over list to find yes/no
space O(Edges) - store only what you need

TRAVERCE -> and track what you are interested in ( changes )

Keep visited set so skip what you have seen.
BFS - queue 
DFS - stack or recursion

Graph is implicit not in memory
ask
1. What is vertex here ?
2. How are they connected ?


| What is the problem asking for?                               | Probable Solution you can go for |
| --- | --- |
| Can you reach a node, group exists or not, path exists or not | BFS or DFS |
| Fewest steps with unweighted graph                            | BFS |
| Cheapest cost with **non-negative weights**                   | Dijkstra's Algorithm *(BFS with a priority queue)* |
| Valid order with dependencies, cycle detection                | Topological Sort *(BFS where you also count)* |
| Connect everything for the *lowest* total cost                | Minimum Spanning Tree *(Kruskal or Prim)* |
| When the graph *isn't* given explicitly                       | Write "get_neighbours" yourself |