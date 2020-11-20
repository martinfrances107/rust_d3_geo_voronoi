use delaunator::Point;

use super::excess::excess;
use std::collections::HashMap;

pub fn edges(triangles: &Vec<Vec<usize>>, point: &[Point]) -> Vec<[usize; 2]> {
    if point.len() == 2usize {
        return vec![[0usize, 1usize]];
    }

    let mut h_index = HashMap::new();
    for tri in triangles {
        if tri[0] == tri[1] {
            return Vec::new();
        }

        let ex_in = vec![point[0].clone(), point[1].clone(), point[2].clone()];

        if excess(&ex_in) < 0f64 {
            return Vec::new();
        }

        for i in 0..3 {
            let j = (i + 1) % 3;
            let code = format!("{}-{}", tri[i], tri[j]);
            h_index.insert(code, true);
        }
    }

    let mut out: Vec<[usize; 2]> = Vec::new();
    for key in h_index.keys() {
        let a_split: Vec<&str> = key.split('-').collect();
        out.push([a_split[0].parse().unwrap(), a_split[1].parse().unwrap()]);
    }

    return out;
}

// function geo_edges(triangles, points) {
//   const _index = {};
//   if (points.length === 2) return [[0, 1]];
//   triangles.forEach(tri => {
//     if (tri[0] === tri[1]) return;
//     if (excess(tri.map(i => points[i])) < 0) return;
//     for (let i = 0, j; i < 3; i++) {
//       j = (i + 1) % 3;
//       _index[extent([tri[i], tri[j]]).join("-")] = true;
//     }
//   });
//   return Object.keys(_index).map(d => d.split("-").map(Number));
// }
