use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;

pub fn geo_neighbors(triangles: Rc<Vec<[usize; 3]>>, npoints: usize) -> HashMap<usize, Vec<usize>> {
    let mut h_neighbors: HashMap<usize, Vec<usize>> = HashMap::new();
    let triangles_borrowed: &Vec<[usize; 3]> = triangles.borrow();
    for tri in triangles_borrowed {
        for j in 0..3 {
            let a: usize = tri[j];
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
