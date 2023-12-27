use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

use num_traits::Float;

use super::EdgeIndex;
use super::TriIndex;
use super::UTransform;

use crate::extent::extent;

pub fn urquhart<T: Float>(
    edges: Rc<HashSet<EdgeIndex>>,
    triangles: Rc<Vec<TriIndex>>,
) -> UTransform<T> {
    Box::new(move |distances: &Vec<T>| {
        let len = edges.len();
        let mut h_lengths: HashMap<EdgeIndex, T> = HashMap::with_capacity(len);
        let mut h_urquhart: HashMap<EdgeIndex, bool> =
            HashMap::with_capacity(len);

        for (i, edge) in edges.iter().enumerate() {
            let u = (edge.0, edge.1);
            h_lengths.insert(u, distances[i]);
            h_urquhart.insert(u, true);
        }

        triangles.iter().for_each(|tri| {
            let mut l = T::zero();
            let mut remove: Option<EdgeIndex> = None;
            for j in 0..3 {
                // extent is used to order the two tri values  smallest to largest.
                let e = extent(vec![tri[j], tri[(j + 1usize) % 3usize]], &None);

                if let Some(l_found) = h_lengths.get(&e) {
                    if *l_found > l {
                        l = *l_found;
                        remove = Some(e);
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
                let code = (edge.0, edge.1);
                return *h_urquhart.get(&code).unwrap();
            })
            .collect();

        out
    })
}
