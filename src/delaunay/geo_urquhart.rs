use std::collections::HashMap;
use std::rc::Rc;

use num_traits::Float;
use rust_d3_array::extent::extent;

pub fn geo_urquhart<T: Float>(
    edges: Rc<Vec<[usize; 2]>>,
    triangles: Rc<Vec<[usize; 3]>>,
) -> Box<dyn Fn(&Vec<T>) -> Vec<bool>> {
    Box::new(move |distances: &Vec<T>| {
        let mut h_lengths: HashMap<String, T> = HashMap::new();
        let mut h_urquhart: HashMap<String, bool> = HashMap::new();

        for (i, edge) in edges.iter().enumerate() {
            let u = format!("{}-{}", edge[0], edge[1]);
            h_lengths.insert(u.clone(), distances[i]);
            h_urquhart.insert(u, true);
        }

        triangles.iter().for_each(|tri| {
            let mut l = T::zero();
            let mut remove: Option<String> = None;
            for j in 0..3 {
                // extent is used to order the two tri values  smallest to largest.
                let e = extent(vec![tri[j], tri[(j + 1usize) % 3usize]], None);

                let u = format!("{}-{}", e[0], e[1]);
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
                let code: String = format!("{}-{}", edge[0], edge[1]);
                return *h_urquhart.get(&code).unwrap();
            })
            .collect();

        out
    })
}
