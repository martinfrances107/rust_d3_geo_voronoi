use geo::{CoordFloat, Coordinate};

use super::excess::excess;
use rust_d3_array::extent::extent;

use std::collections::HashSet;

pub fn geo_edges<T: CoordFloat>(
    triangles: &[[usize; 3]],
    point: &[Coordinate<T>],
) -> HashSet<[usize; 2]> {
    let mut h_index = HashSet::new();
    if point.len() == 1 {
        h_index.insert([0usize, 1usize]);
        return h_index;
    }
    let zero = T::zero();
    for tri in triangles {
        if tri[0] == tri[1] {
            continue;
        }

        let ex_in: Vec<Coordinate<T>> = tri.iter().map(|i| point[*i]).collect();

        if excess(&ex_in) < zero {
            continue;
        }

        for i in 0..3 {
            let j = (i + 1) % 3;
            let code = extent(vec![tri[i], tri[j]], None);
            h_index.insert(code);
        }
    }

    h_index
}
