extern crate rust_d3_geo;
extern crate rust_d3_array;

pub mod voronoi;
pub mod delaunay;
mod math;


#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
