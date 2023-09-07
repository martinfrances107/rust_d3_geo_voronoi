use std::collections::HashMap;
use std::collections::HashSet;

use delaunator::EMPTY;
use geo::CoordFloat;
use geo_types::Coord;

use super::excess::excess;
use super::EdgeIndex;
use super::TriIndex;

pub fn hull<T>(triangles: &[TriIndex], points: &[Coord<T>]) -> Vec<usize>
where
    T: CoordFloat,
{
    let mut h_hull: HashSet<EdgeIndex> = HashSet::new();
    let mut hull = Vec::new();

    for tri in triangles {
        let ex_in: Vec<Coord<T>> = tri
            .iter()
            .map(|i: &usize| {
                let index: usize = if i > &points.len() { 0 } else { *i };
                points[index]
            })
            .collect();

        if excess(&ex_in) < T::zero() {
            continue;
        }

        for i in 0usize..3usize {
            let e = (tri[i], tri[(i + 1) % 3]);
            let code = (e.1, e.0);
            if h_hull.get(&code).is_some() {
                h_hull.remove(&code);
            } else {
                h_hull.insert(e);
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
