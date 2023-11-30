use geo::Coord;
use geo::CoordFloat;

use d3_geo_rs::distance::distance;

use super::Voronoi;

impl<T> Voronoi<T>
where
    T: CoordFloat,
{
    /// Returns the index associated with the given point.
    pub fn find(&mut self, p: &Coord<T>, radius: Option<T>) -> Option<usize> {
        self.found = self.delaunay.find(p, self.found);
        match radius {
            Some(radius) => match self.found {
                Some(found) => {
                    if distance(p, &self.points[found]) < radius {
                        Some(found)
                    } else {
                        None
                    }
                }
                None => None,
            },
            None => self.found,
        }
    }
}
