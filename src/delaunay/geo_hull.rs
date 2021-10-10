use std::borrow::Borrow;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use delaunator::EMPTY;
use geo::{CoordFloat, Coordinate};

use super::excess::excess;

pub fn geo_hull<T>(triangles: Rc<Vec<[usize; 3]>>, points: &[Coordinate<T>]) -> Vec<usize>
where
    T: CoordFloat,
{
    let mut h_hull: HashSet<(usize, usize)> = HashSet::new();
    let mut hull = Vec::new();

    let triangles_borrowed: &Vec<[usize; 3]> = triangles.borrow();
    for tri in triangles_borrowed {
        let ex_in: Vec<Coordinate<T>> = tri
            .iter()
            .map(|i: &usize| {
                let index;
                if i > &points.len() {
                    index = 0;
                } else {
                    index = *i;
                };
                points[index]
            })
            .collect();

        if excess(&ex_in) < T::zero() {
            continue;
        }

        for i in 0usize..3usize {
            let e = [tri[i], tri[(i + 1) % 3]];
            let code = (e[1], e[0]);
            match h_hull.get(&code) {
                Some(_) => {
                    h_hull.remove(&code);
                }
                None => {
                    let code = (e[0], e[1]);
                    h_hull.insert(code);
                }
            }
        }
    }

    let mut start: Option<usize> = None;
    let mut h_index: HashMap<usize, usize> = HashMap::with_capacity(h_hull.len());

    // TODO Unresolved. The javascript implementation enumerates the keys differently.
    // does this make a difference?
    for key in h_hull.drain() {
        h_index.insert(key.0, key.1);
        start = Some(key.0);
    }

    match start {
        None => hull,
        Some(start) => {
            let mut next = start;
            'l: loop {
                hull.push(next);
                let n = *h_index
                    .get(&next)
                    .expect("must pull a valid value from h_index");
                h_index.insert(next, EMPTY);
                next = n;
                if next == EMPTY || next == start {
                    break 'l;
                }
            }
            hull
        }
    }
}
