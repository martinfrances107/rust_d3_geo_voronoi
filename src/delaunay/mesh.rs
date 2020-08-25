pub fn mesh(polygons: Vec<Vec<usize>>) -> Vec<[usize; 2]> {
  let mesh = Vec::new();
  for poly in polygons {
    if poly.len() == 0 {
      return Vec::new();
    }
    let p: usize = *poly.last().unwrap();
    for q in poly {
      if q > p {
        mesh.push([p, q]);
      }
      p = q;
    }
  }

  return mesh;
}
