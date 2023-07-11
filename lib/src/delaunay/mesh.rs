pub fn mesh(polygons: &[Vec<usize>]) -> Vec<[usize; 2]> {
    // Provide an underestimate for capacity
    // For large polygons this will provide some relief
    // from constant reallocation.
    let mut mesh = Vec::with_capacity(polygons.len());
    for poly in polygons {
        if poly.is_empty() {
            return Vec::new();
        }
        let mut p: usize = *poly.last().unwrap();
        for q in poly {
            if q > &p {
                mesh.push([p, *q]);
            }
            p = *q;
        }
    }

    mesh
}
