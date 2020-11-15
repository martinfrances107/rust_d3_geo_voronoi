use std::collections::HashMap;

use delaunator::Point;
use delaunator::EMPTY;

use super::excess::excess;

pub fn hull(triangles: &Vec<Vec<usize>>, points: &Vec<Point>) -> Vec<usize> {
    let mut h_hull: HashMap<String, bool> = HashMap::new();
    let mut hull = Vec::new();

    for tri in triangles {
        let ex_in: Vec<Point> = tri
            .iter()
            .map(|i: &usize| {
                let index;
                if i > &points.len() {
                    index = 0;
                } else {
                    index = *i;
                };
                return points[index].clone();
            })
            .collect();

        if excess(&ex_in) < 0f64 {
            continue;
        }

        for i in 0usize..3usize {
            let e = [tri[i], tri[(i + 1) % 3]];
            let code = format!("{}-{}", e[1], e[0]);
            match h_hull.get(&code) {
                Some(value) => {
                    if *value {
                        h_hull.remove(&code);
                    } else {
                        let code = format!("{}-{}", e[0], e[1]);
                        h_hull.insert(code, true);
                    }
                }
                None => {
                    let code = format!("{}-{}", e[0], e[1]);
                    h_hull.insert(code, true);
                }
            }
        }
    }

    let mut start: Option<usize> = None;
    let mut h_index: HashMap<usize, usize> = HashMap::new();

    // TODO Unresolved. The javascript implementation enumerates the keys differently.
    // does this make a difference?
    for key in h_hull.keys() {
        let e_split: Vec<&str> = key.split('-').collect();
        let e: [usize; 2] = [e_split[0].parse().unwrap(), e_split[1].parse().unwrap()];
        h_index.insert(e[0], e[1]);
        start = Some(e[0]);
    }

    match start {
        None => return hull,
        Some(start) => {
            let mut next = start;
            'l: loop {
                hull.push(next.clone());
                let n: usize = *h_index.get(&next).expect("must pull a valid element");
                h_index.insert(next.clone(), EMPTY);
                next = n;
                if next == EMPTY || next == start {
                    break 'l;
                }
            }
        }
    }

    return hull;
}
