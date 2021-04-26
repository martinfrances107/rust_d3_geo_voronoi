use geo::{CoordFloat, Coordinate};
use num_traits::Float;

use super::excess::excess;
use rust_d3_array::extent::extent;

use std::collections::HashSet;

pub fn geo_edges<T: CoordFloat>(
    triangles: &[Vec<usize>],
    point: &[Coordinate<T>],
) -> Vec<[usize; 2]> {
    if point.len() == 2usize {
        return vec![[0usize, 1usize]];
    }
    let zero = T::zero();
    let mut h_index = HashSet::new();

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

    let out = h_index.into_iter().collect::<Vec<_>>();

    return out;
}
