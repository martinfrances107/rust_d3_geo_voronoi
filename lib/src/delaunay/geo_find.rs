use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::Float;

use super::cartesian::cartesian;
use crate::delaunay::FindReturn;

#[inline]
fn distance2<T: Float>(a: &[T; 3], b: &[T; 3]) -> T {
    let x = a[0] - b[0];
    let y = a[1] - b[1];
    let z = a[2] - b[2];
    x * x + y * y + z * z
}

#[allow(clippy::similar_names)]
pub fn geo_find<'a, T: CoordFloat + 'static>(
    neighbors: Rc<RefCell<HashMap<usize, Vec<usize>>>>,
    points: Rc<Vec<Coord<T>>>,
) -> FindReturn<'a, T> {
    Box::new(
        move |p: &Coord<T>, next_p: Option<usize>| -> Option<usize> {
            let next_or_none = next_p.map_or(Some(0usize), Some);
            let mut dist: T;
            let mut found = next_or_none;
            let xyz = cartesian(&Coord { x: p.x, y: p.y });
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
