use std::collections::HashMap;
use std::rc::Rc;

use delaunator::Point;
use rust_d3_array::extent::extent;

pub fn urquhart(
  edges: Rc<Vec<[usize; 2]>>,
  triangles: Rc<Vec<Vec<usize>>>,
) -> Box<dyn Fn(&Vec<f64>) -> Vec<bool>>
{
  return Box::new(move |distances: &Vec<f64>| {
    let mut h_lengths: HashMap<String, f64> = HashMap::new();
    let mut h_urquhart: HashMap<String, bool> = HashMap::new();

    for (i, edge) in edges.iter().enumerate() {
      let u_lengths = format!("{}-{}", edge[0], edge[1]);
      let u_urquhart = format!("{}-{}", edge[0], edge[1]);
      h_lengths.insert(u_lengths, distances[i]);
      h_urquhart.insert(u_urquhart, true);
    }

    triangles.iter().for_each(|tri| {
      let mut l = 0f64;
      let mut remove: Option<String> = None;
      for j in 0..3 {
        // extent is used to order the two tri values  smallest to largest.
        let e = extent(vec![tri[j], tri[(j + 1usize) % 3usize]]);

        let u = format!("{}-{}", e[0], e[1]);
        if *h_lengths.get(&u).unwrap() > l {
          l = *h_lengths.get(&u).unwrap();
          remove = Some(u);
        }
        else {
          remove = None;
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
