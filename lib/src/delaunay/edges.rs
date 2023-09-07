use std::collections::HashSet;

use geo::CoordFloat;
use geo_types::Coord;

use crate::extent::extent;

use super::excess::excess;
use super::EdgeIndex;
use super::TriIndex;

pub fn edges<T: CoordFloat>(triangles: &[TriIndex], point: &[Coord<T>]) -> HashSet<EdgeIndex> {
    if point.len() == 1 {
        return HashSet::from([(0usize, 1usize)]);
    }
    // capacity is a underestimate but if triangles is large
    // it will provide some relief from constant reallocation.
    let mut h_index = HashSet::with_capacity(triangles.len());
    let zero = T::zero();
    for tri in triangles {
        if tri[0] == tri[1] {
            continue;
        }

        let ex_in: Vec<Coord<T>> = tri.iter().map(|i| point[*i]).collect();
        if excess(&ex_in) < zero {
            continue;
        }

        for i in 0..3 {
            let j = (i + 1) % 3;
            let code = extent(vec![tri[i], tri[j]], &None);
            h_index.insert(code);
        }
    }

    h_index
}
