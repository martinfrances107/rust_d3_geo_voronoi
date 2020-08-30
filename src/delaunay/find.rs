use std::collections::HashMap;
use std::rc::Rc;

use num_traits::Float;

use rust_d3_geo::cartesian::cartesian;

fn distance2<F>(a: [F; 3], b: [F; 3]) -> F
where
  F: Float,
{
  let x = a[0] - b[0];
  let y = a[1] - b[1];
  let z = a[2] - b[2];
  return x * x + y * y + z * z;
}

pub fn find<'a, F>(
  neighbors: Rc<HashMap<usize, Vec<usize>>>,
  points: Rc<Vec<[F; 2]>>,
) -> Box<dyn Fn(F, F, Option<usize>) -> Option<usize> + 'a>
where
  F: Float + 'static,
{
  let points = points.clone();
  return Box::new(move |x: F, y: F, next_p: Option<usize>| -> Option<usize> {
    let next_or_none = match next_p {
      Some(n) => Some(n),
      None => Some(0usize),
    };
    let mut dist: F;
    let mut found = next_or_none;
    let xyz = cartesian(&[x, y]);
    'outer: loop {
      let cell = next_or_none.unwrap();
      let mut next_or_no = None;
      dist = distance2(xyz, cartesian(&points[cell]));
      let row = neighbors.get(&cell);
      match row {
        Some(row) => {
          for i in row {
            let ndist = distance2(xyz, cartesian(&points[*i]));
            if ndist < dist {
              dist = ndist;
              next_or_no = Some(*i);
              found = Some(*i);
            }
          }

          if next_or_no.is_some() {
            break 'outer;
          }
        }
        None => {}
      }
    }
    return found;
  });
}
