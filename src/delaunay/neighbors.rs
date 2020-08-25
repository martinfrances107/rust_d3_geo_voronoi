use std::collections::HashMap;

pub fn neighbors(triangles: Vec<[usize; 3]>, npoints: usize) -> HashMap<usize, Vec<usize>> {
  let h_neighbors: HashMap<usize, Vec<usize>> = HashMap::new();
  for tri in triangles {
    for j in 0..3 {
      let a = tri[j];
      let b = tri[j + 1];
      let entry = h_neighbors.entry(a).or_insert(vec![]);
      (*entry).push(b);
    }
  }
  // degenerate cases
  if triangles.len() == 0usize {
    if npoints == 2usize {
      h_neighbors.insert(0usize, vec![1usize]);
      h_neighbors.insert(1usize, vec![0]);
    };
  } else if npoints == 1 {
    h_neighbors.insert(0usize, vec![]);
  }

  return h_neighbors;
}
