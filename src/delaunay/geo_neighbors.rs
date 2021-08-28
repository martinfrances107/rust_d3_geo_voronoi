use std::collections::HashMap;

pub fn geo_neighbors(triangles: &[Vec<usize>], npoints: usize) -> HashMap<usize, Vec<usize>> {
    let mut h_neighbors: HashMap<usize, Vec<usize>> = HashMap::new();
    for tri in triangles {
        for j in 0..3 {
            let a = tri[j];
            let b = tri[(j + 1) % 3];
            let entry = h_neighbors.entry(a).or_insert_with(Vec::new);
            (*entry).push(b);
        }
    }
    // degenerate cases
    if triangles.is_empty() {
        if npoints == 2usize {
            h_neighbors.insert(0usize, vec![1]);
            h_neighbors.insert(1usize, vec![0]);
        };
    } else if npoints == 1 {
        h_neighbors.insert(0usize, vec![]);
    }

    h_neighbors
}
