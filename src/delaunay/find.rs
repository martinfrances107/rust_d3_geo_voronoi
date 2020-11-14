use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use delaunator::Point;

use super::cartesian::cartesian;

fn distance2(a: [f64; 3], b: [f64; 3]) -> f64 {
    let x = a[0] - b[0];
    let y = a[1] - b[1];
    let z = a[2] - b[2];
    return x * x + y * y + z * z;
}

pub fn find<'a>(
    neighbors: Rc<RefCell<HashMap<usize, Vec<usize>>>>,
    points: Rc<Vec<Point>>,
) -> Box<dyn Fn(f64, f64, Option<usize>) -> Option<usize> + 'a> {
    let points = points.clone();
    return Box::new(
        move |x: f64, y: f64, next_p: Option<usize>| -> Option<usize> {
            let next_or_none = match next_p {
                Some(n) => Some(n),
                None => Some(0usize),
            };
            let mut dist: f64;
            let mut found = next_or_none;
            let xyz = cartesian(&Point { x, y });
            'outer: loop {
                let cell = next_or_none.unwrap();
                let mut next_or_no = None;
                dist = distance2(xyz, cartesian(&points[cell]));
                let n = neighbors.borrow();
                let row = n.get(&cell);
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
        },
    );
}
