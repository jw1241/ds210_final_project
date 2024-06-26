use std::collections::{VecDeque, HashMap};

pub fn compute_min_distance(graph: &Vec<Vec<usize>>) -> HashMap<usize, Vec<usize>> {
    let mut distance_all: HashMap<usize, Vec<usize>> = HashMap::new();
    //Perform BFS
    for i in 0..graph.len() {
        let mut min_distance: Vec<usize> = vec![usize::MAX; graph.len()];
        let mut queue: VecDeque<usize> = VecDeque::new();

        min_distance[i] = 0;
        queue.push_back(i); //New unprocessed vertex

        while let Some(v) = queue.pop_front() { //Look at unprocessed neighbors of the list
            for &u in &graph[v] {
                if min_distance[u] == usize::MAX { 
                    min_distance[u] = min_distance[v] + 1;
                    queue.push_back(u);
                }
            }
        }
        let mut min_distances = Vec::new();
        for dist in min_distance.iter() { 
            if *dist != usize::MAX { //Find the distances from BFS and add to Hashmap
                min_distances.push(*dist);
            }
        }
        distance_all.insert(i, min_distances);
    }
    return distance_all;
}