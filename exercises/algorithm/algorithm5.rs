/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;

// Define a graph
/*
┌─────────────────────────────────────────┐
│            Graph 结构体                  │
│  ┌─────────────────────────────────┐   │
│  │  adj: Vec<Vec<usize>>          │   │  ← 邻接表存储
│  │  [0]: [1, 4]  (节点0的邻居)     │   │
│  │  [1]: [0, 2, 3, 4]             │   │
│  │  ...                           │   │
│  └─────────────────────────────────┘   │
└─────────────────────────────────────────┘
         │ new()    │ add_edge()   │ bfs_with_return()
         ▼          ▼              ▼
      创建图      添加无向边       遍历

 */
struct Graph {
    adj: Vec<Vec<usize>>, //邻接表（Adjacency List）表示法
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        // BFS 队列
        let mut queue = VecDeque::new();
        // 访问标记数组
        let mut visited = vec![false; self.adj.len()];
        let mut visit_order = vec![];
        // 起点入队
        queue.push_back(start);
        visited[start] = true;

        // BFS 循环
        while let Some(current) = queue.pop_front() {
            visit_order.push(current);

            // 遍历当前节点的所有邻居, 如果邻居未访问过, 则入队
            for &next in &self.adj[current] {
                if !visited[next] {
                    visited[next] = true;
                    queue.push_back(next);
                }
            }
        }
        visit_order
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

