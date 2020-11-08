use std::collections::HashMap;

use delaunator::Point;

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

    let mut start: Option<usize> = None;
    let mut h_index: HashMap<usize, Option<usize>> = HashMap::new();

    for key in h_hull.keys() {
        let a_split: Vec<&str> = key.split('-').collect();
        let e: [usize; 2] = [a_split[0].parse().unwrap(), a_split[1].parse().unwrap()];

        h_index.insert(e[0], Some(e[1]));
        start = Some(e[0]);
    }

    match start {
        None => return hull,
        Some(start) => {
            let mut next = start;
            'l: loop {
                let n: Option<usize> = h_index.get(&next).unwrap().clone();
                hull.push(next.clone());
                h_index.insert(next.clone(), None);
                match n {
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
