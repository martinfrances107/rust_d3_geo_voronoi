#![allow(clippy::many_single_char_names)]
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use geo::{CoordFloat, Coordinate};
use num_traits::Float;

use super::cartesian::cartesian;

#[inline]
fn distance2<T: Float>(a: &[T; 3], b: &[T; 3]) -> T {
    let x = a[0] - b[0];
    let y = a[1] - b[1];
    let z = a[2] - b[2];
    x * x + y * y + z * z
}

pub fn geo_find<'a, T: CoordFloat + 'static>(
    neighbors: Rc<RefCell<HashMap<usize, Vec<usize>>>>,
    points: Rc<Vec<Coordinate<T>>>,
) -> Box<dyn Fn(&Coordinate<T>, Option<usize>) -> Option<usize> + 'a> {
    Box::new(
        move |p: &Coordinate<T>, next_p: Option<usize>| -> Option<usize> {
            let next_or_none = match next_p {
                Some(n) => Some(n),
                None => Some(0usize),
            };
            let mut dist: T;
            let mut found = next_or_none;
            let xyz = cartesian(&Coordinate { x: p.x, y: p.y });
            'outer: loop {
                let cell = next_or_none.unwrap();
                let mut next_or_no = None;
                dist = distance2(&xyz, &cartesian(&points[cell]));
                let n = neighbors.borrow();
                let row = n.get(&cell);
                if let Some(row) = row {
                    for i in row {
                        let ndist = distance2(&xyz, &cartesian(&points[*i]));
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
            }
            found
        },
    )
}
