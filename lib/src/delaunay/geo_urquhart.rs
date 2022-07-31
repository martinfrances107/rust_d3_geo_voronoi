use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use num_traits::Float;
use rust_d3_array::extent::extent;

use super::EdgeIndex;
use super::TriIndex;


pub fn geo_urquhart<T: Float>(
    edges: Rc<HashSet<EdgeIndex>>,
    triangles: Rc<Vec<TriIndex>>,
) -> Box<dyn Fn(&Vec<T>) -> Vec<bool>> {
    Box::new(move |distances: &Vec<T>| {
        let len = edges.len();
        let mut h_lengths: HashMap<(usize, usize), T> = HashMap::with_capacity(len);
        let mut h_urquhart: HashMap<(usize, usize), bool> = HashMap::with_capacity(len);

        for (i, edge) in edges.iter().enumerate() {
            let u = (edge[0], edge[1]);
            h_lengths.insert(u, distances[i]);
            h_urquhart.insert(u, true);
        }

        triangles.iter().for_each(|tri| {
            let mut l = T::zero();
            let mut remove: Option<(usize, usize)> = None;
            for j in 0..3 {
                // extent is used to order the two tri values  smallest to largest.
                let e = extent(vec![tri[j], tri[(j + 1usize) % 3usize]], None);

                let u = (e[0], e[1]);
                if let Some(l_found) = h_lengths.get(&u) {
                    if *l_found > l {
                        l = *l_found;
                        remove = Some(u);
                    }
                }
            }
            if let Some(r) = remove {
                h_urquhart.insert(r, false);
            }
        });
        let out: Vec<bool> = edges
            .iter()
            .map(|edge| {
                let code = (edge[0], edge[1]);
                return *h_urquhart.get(&code).unwrap();
            })
            .collect();

        out
    })
}
