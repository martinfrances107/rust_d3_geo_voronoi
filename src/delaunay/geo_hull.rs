use std::collections::HashMap;
use std::collections::HashSet;

use delaunator::EMPTY;
use geo::{CoordFloat, Coordinate};

use super::excess::excess;

pub fn geo_hull<T>(triangles: &[[usize; 3]], points: &[Coordinate<T>]) -> Vec<usize>
where
    T: CoordFloat,
{
    let mut h_hull: HashSet<String> = HashSet::new();
    let mut hull = Vec::new();

    for tri in triangles {
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
            let code = format!("{}-{}", e[1], e[0]);
            match h_hull.get(&code) {
                Some(_) => {
                    h_hull.remove(&code);
                }
                None => {
                    let code = format!("{}-{}", e[0], e[1]);
                    h_hull.insert(code);
                }
            }
        }
    }

    let mut start: Option<usize> = None;
    let mut h_index: HashMap<usize, usize> = HashMap::new();

    // TODO Unresolved. The javascript implementation enumerates the keys differently.
    // does this make a difference?
    for key in h_hull.drain() {
        let e_split: Vec<&str> = key.split('-').collect();
        let e: [usize; 2] = [e_split[0].parse().unwrap(), e_split[1].parse().unwrap()];
        h_index.insert(e[0], e[1]);
        start = Some(e[0]);
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
            println!("hull {:?}", hull);
            hull
        }
    }
}
