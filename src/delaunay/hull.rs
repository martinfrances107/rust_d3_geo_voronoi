use std::collections::HashMap;

use num_traits::Float;

use super::excess::excess;

pub fn hull<F>(triangles: &Vec<[usize; 3]>, points: &Vec<[F; 2]>) -> Vec<usize>
where
  F: Float,
{
  let h_hull: HashMap<String, bool> = HashMap::new();
  let hull = Vec::new();

  for tri in triangles {
    let ex_in: [[F; 2]; 3];
    for i in 0..3 {
      let ind = tri[i];
      if ind > points.len() {
        ind = 0;
      }
      ex_in[i] = points[ind];
    }
    if excess(ex_in) < F::zero() {
      return Vec::new();
    }

    for i in 0usize..3usize {
      let e = [tri[i], tri[(i + 1usize) % 3]];
      let code = format!("{}-{}", e[1], e[0]);
      match h_hull.get(&code) {
        Some(_) => {
          h_hull.remove(&code);
        }
        None => {
          h_hull.insert(code, true);
        }
      }
    }
  }

  let start: Option<usize> = None;

  // let code: [usize; 2];
  let h_index: HashMap<usize, Option<usize>> = HashMap::new();

  for key in h_hull.keys() {
    let a_split: Vec<&str> = key.split('-').collect();
    let e: [usize; 2] = [a_split[0].parse().unwrap(), a_split[1].parse().unwrap()];

    h_index.insert(e[0], Some(e[1]));
    start = Some(e[0]);
  }

  match start {
    None => return hull,
    Some(start) => {
      let next = start;
      'l: loop {
        hull.push(next);
        let n = h_index.get(&next).unwrap();
        h_index.insert(next, None);
        match *n {
          Some(n) => {
            next = n;
          }
          None => {}
        }

        if next == start {
          break 'l;
        }
      }
    }
  }

  return hull;
}
