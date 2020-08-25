use std::collections::HashMap;

use num_traits::Float;
use rust_d3_array::extent::extent;

pub fn urquhart<F>(
  edges: Vec<[usize; 2]>,
  triangles: Vec<[usize; 3]>,
) -> Box<dyn Fn(Vec<F>) -> Vec<bool>>
where
  F: Float,
{
  return Box::new(|distances: Vec<F>| {
    let h_lengths: HashMap<String, F> = HashMap::new();
    let h_urquhart: HashMap<String, bool> = HashMap::new();

    for (i, edge) in edges.iter().enumerate() {
      let u = format!("{}-{}", edge[0], edge[1]);
      h_lengths.insert(u, distances[i]);
      h_urquhart.insert(u, true);
    }

    triangles.iter().for_each(|tri| {
      let l = F::zero();
      let remove: Option<String>;
      for j in 0..3 {
        // extent is used to order the two tri values  smallest to largest.
        let e = extent(vec![tri[j], tri[(j + 1usize) % 3usize]]);

        let u = format!("{}-{}", e[0], e[1]);
        if *h_lengths.get(&u).unwrap() > l {
          l = *h_lengths.get(&u).unwrap();
          remove = Some(u);
        }
      }
      match remove {
        Some(r) => {
          h_urquhart.insert(r, false);
        }
        None => {}
      }
    });

    let out: Vec<bool> = edges
      .iter()
      .map(|edge| {
        let code: String = format!("{}-{}", edge[0], edge[1]);
        return *h_urquhart.get(&code).unwrap();
      })
      .collect();

    return out;
  });
}

// function urquhart(edges, triangles) {
//   return function(distances) {
//     const _lengths = {},
//       _urquhart = {};
//     edges.forEach((edge, i) => {
//       const u = edge.join("-");
//       _lengths[u] = distances[i];
//       _urquhart[u] = true;
//     });

//     triangles.forEach(tri => {
//       let l = 0,
//         remove = -1;
//       for (var j = 0; j < 3; j++) {
//         let u = extent([tri[j], tri[(j + 1) % 3]]).join("-");
//         if (_lengths[u] > l) {
//           l = _lengths[u];
//           remove = u;
//         }
//       }
//       _urquhart[remove] = false;
//     });

//     return edges.map(edge => _urquhart[edge.join("-")]);
//   };
// }
