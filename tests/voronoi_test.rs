mod voronoi_test {
    extern crate pretty_assertions;

    use geo::Point;
    use rust_d3_geo::data_object::feature_collection::FeatureCollection;

    use geo::algorithm::cyclic_match::CyclicMatch;
    use geo::Geometry;
    use geo::LineString;
    use geo::MultiPoint;
    use rust_d3_geo_voronoi::voronoi::Voronoi;

    #[cfg(test)]
    use pretty_assertions::assert_eq;

    #[test]
    pub fn voronoi_polygons_returns_polygons() {
        println!("geoVoronoi.polygons(sites) returns polygons.");
        let sites = MultiPoint(vec![
            Point::new(0f64, 0f64),
            Point::new(10f64, 0f64),
            Point::new(0f64, 10f64),
        ]);

        match Voronoi::new(Some(Geometry::MultiPoint(sites))).polygons(None) {
            None => {
                assert!(false, "Must return a DataObject<T>.");
            }
            Some(FeatureCollection(features)) => {
                println!("Found a Features Collection.");
                let g = &features[0].geometry[0];
                match g {
                    Geometry::Polygon(polygon) => {
                        let ls = polygon.exterior();
                        let u = ls.points_iter().next().unwrap();
                        let v = Point::new(-175f64, -4.981069f64);
                        assert!((u.x() - v.x()).abs() < 1e-6f64);
                        assert!((u.y() - v.y()).abs() < 1e-6f64);
                    }
                    _ => {
                        assert!(false, "Expected a polygon object.");
                    }
                }
            }
        }
    }
    #[test]
    fn test_polygon_tollerates_nan() {
        println!("geoVoronoi.polygons(sites) tolerates NaN.");
        //var u = geoVoronoi.geoVoronoi().polygons(sites)[0][0], v = [ 5, 4.981069 ];
        //test.ok( (Math.abs(u[0]-v[0]) < 1e-6) && (Math.abs(u[1]-v[1]) < 1e-6) );
        let sites = MultiPoint(vec![
            Point::new(0f64, 0f64),
            Point::new(2f64, 1f64),
            Point::new(f64::NAN, -1f64),
            Point::new(4f64, f64::NAN),
            Point::new(5f64, 10f64),
        ]);
        // TODO the javascript version makes no assertions - if the test ends without expception then PASS!
        // This should be tightened up.
        let _u = Voronoi::new(Some(Geometry::MultiPoint(sites))).polygons(None);
    }

    #[test]
    fn test_computes_the_hull() {
        let sites = MultiPoint(vec![
            Point::new(10f64, 0f64),
            Point::new(10f64, 10f64),
            Point::new(3f64, 5f64),
            Point::new(-2f64, 5f64),
            Point::new(0f64, 0f64),
        ]);
        let hull = Voronoi::new(None).hull(Some(Geometry::MultiPoint(sites)));
        match hull {
            Some(polygon) => {
                let actual_ls = polygon.exterior();
                let expected_ls = LineString::from(vec![
                    Point::new(10f64, 10f64),
                    Point::new(10f64, 0f64),
                    Point::new(0f64, 0f64),
                    Point::new(-2f64, 5f64),
                    Point::new(10f64, 10f64),
                ]);
                assert!(actual_ls.is_cyclic_match(expected_ls));
            }
            None => {
                panic!("expecting a polygon");
            }
        }
    }

    // #[test]
    // pub fn voronoi_polygons_returns_polygons_tollerates_nan() {
    //   println!("geoVoronoi.polygons(sites) tolerates NaN.");
    //   let sites: Vec<[f64; 2]> = vec![[0f64, 0f64], [10f64, 0f64]];
    //   let mut u = Voronoi::<f64>::new(DataType::Blank);
    //   let up = u.polygons(DataType::<f64>::Vec(sites)).unwrap()[0][0];

    //   let sites_bad = vec![[0f64, 0f64], [2f64, 1f64], [f64::NAN, -1f64], [4f64, f64::NAN], [5f64,10f64]];
    //   let u = Voronoi::new(sites).polygons();
    //   assert!(u.is_some());
    // }

    // tape("geoVoronoi.polygons([no valid site]) returns an empty collection.", function(test) {
    //   const sites = [[NaN, -1], [4, NaN], [Infinity,10]];
    //   var u = geoVoronoi.geoVoronoi(sites).polygons();
    //   test.deepEqual(u.features, []);
    //   test.end();
    // });

    // tape("geoVoronoi.polygons([1 site]) returns a Sphere.", function(test) {
    //   const sites = [[NaN, -1], [4, NaN], [5,10]];
    //   var u = geoVoronoi.geoVoronoi(sites).polygons();
    //   test.equal(u.features[0].type, "Feature");
    //   test.equal(u.features[0].geometry.type, "Sphere");
    //   test.end();
    // });

    // var sites = [[0,0], [10,0], [0,10]];

    // tape("geoVoronoi.links() returns urquhart.", function(test) {
    //   test.deepEqual(geoVoronoi.geoVoronoi().links(sites).features.map(function(d) { return d.properties.urquhart; }), [ false, true, true ]);
    //   test.end();
    // });

    // tape("geoVoronoi.x() changes accessor.", function(test) {
    //   var sites = [{lon:10,lat:0}, {lon:3, lat:5}, {lon:-2, lat:5}];
    //   test.deepEqual(
    //   	geoVoronoi.geoVoronoi().x(d => +d.lon).y(d => +d.lat)
    //   		(sites).points,
    //   	[ [ 10, 0 ], [ 3, 5 ], [ -2, 5 ] ]
    //   );
    //   test.end();
    // });

    // tape("geoVoronoi.mesh() computes the Delaunay mesh.", function(test) {
    //   var sites = [[10,0],[10,10],[3,5],[-2,5],[0,0]];
    //   test.deepEqual(
    //   	geoVoronoi.geoVoronoi().mesh(sites),
    //   	{ type: 'MultiLineString', coordinates: [ [ [ 3, 5 ], [ -2, 5 ] ], [ [ 3, 5 ], [ 0, 0 ] ], [ [ -2, 5 ], [ 0, 0 ] ], [ [ 10, 10 ], [ -2, 5 ] ], [ [ 10, 10 ], [ 3, 5 ] ], [ [ 10, 0 ], [ 3, 5 ] ], [ [ 10, 0 ], [ 0, 0 ] ], [ [ 10, 0 ], [ 10, 10 ] ] ] }
    //   );
    //   test.end();
    // });

    use approx::AbsDiffEq;

    #[test]
    fn computes_the_delaunay_mesh() {
        let sites = MultiPoint(vec![
            Point::new(10f64, 0f64),
            Point::new(10f64, 10f64),
            Point::new(3f64, 5f64),
            Point::new(-2f64, 5f64),
            Point::new(0f64, 0f64),
        ]);
        let mesh = Voronoi::new(Some(Geometry::MultiPoint(sites))).mesh(None);

        let golden: Vec<LineString<f64>> = vec![
            vec![[3., 5.], [-2., 5.]].into(),
            vec![[3., 5.], [0., 0.]].into(),
            vec![[-2., 5.], [0., 0.]].into(),
            vec![[10., 10.], [-2., 5.]].into(),
            vec![[10., 10.], [3., 5.]].into(),
            vec![[10., 0.], [3., 5.]].into(),
            vec![[10., 0.], [0., 0.]].into(),
            vec![[10., 0.], [10., 10.]].into(),
        ];

        match mesh {
            Some(mesh) => {
                for computed_ls in mesh {
                    let mut found = false;
                    for golden_ls in &golden {
                        if computed_ls.abs_diff_eq(&golden_ls, 1e-2) {
                            found = true;
                            continue; // Skip golden loop.
                        }
                    }
                    if !found {
                        assert!(false, "linestring not found in golden list.");
                    }
                }
                // All mesh points inspected no rejections
                assert!(true, "all linestrings found.")
            }
            None => {
                assert!(false);
            }
        }
    }

    // tape("geoVoronoi.cellMesh() computes the Polygons mesh.", function(test) {
    //   var sites = [[10,0],[10,10],[3,5],[-2,5],[0,0]];
    //   var cellMesh = geoVoronoi.geoVoronoi().cellMesh(sites),
    //     coords = cellMesh.coordinates
    //       .map(d => d.map(e => e.map(Math.round).join(" ")).sort().join("/")).sort();
    //   test.deepEqual(
    //   	coords,
    //   	[ '-175 -5/-175 -5', '-175 -5/0 3', '-175 -5/1 15', '-175 -5/5 0', '-175 -5/8 5', '0 3/1 15', '0 3/5 0', '1 15/8 5', '5 0/8 5' ]
    //   );
    //   test.end();
    // });

    // tape("geoVoronoi.find() finds p", function(test) {
    //   var sites = [[10,0],[10,10],[3,5],[-2,5],[0,0]],
    //       voro = geoVoronoi.geoVoronoi(sites);
    //   test.equal(voro.find(1,1),4);
    //   test.equal(voro.find(1,1,4),4);
    //   test.end();
    // });

    // fn geoVoronoi_finds_p() {
    //     let sites = MultiPoint(vec![
    //         Point::new(10f64, 0f64),
    //         Point::new(10f64, 10f64),
    //         Point::new(3f64, 5f64),
    //         Point::new(-2f64, 5f64),
    //         Point::new(0f64, 0f64),
    //     ]);
    //     let voro = Voronoi::new(Some(Geometry::MultiPoint(sites)));
    //     assert!(voro.find(Coordinate{x:1,y:1}, Some(4)));

    // }

    // tape("geoVoronoi.links(sites) returns links.", function(test) {
    //   test.deepEqual(geoVoronoi.geoVoronoi().links(sites).features.map(function(d) { return d.properties.source[0]; }), [ 10, 0, 0 ]);
    //   test.end();
    // });

    // tape("geoVoronoi.triangles(sites) returns geojson.", function(test) {
    //   const tri = geoVoronoi.geoVoronoi().triangles(sites);
    //   test.equal(tri.type, "FeatureCollection");
    //   test.equal(tri.features.length, 1);
    //   test.end();
    // });

    // tape("geoVoronoi.links(sites) returns urquhart graph.", function(test) {
    //   test.deepEqual(geoVoronoi.geoVoronoi().links(sites).features.map(function(d) { return d.properties.urquhart; }), [ false, true, true ]);
    //   test.end();
    // });

    // tape("geoVoronoi.triangles(sites) returns circumcenters.", function(test) {
    //     var u = geoVoronoi.geoVoronoi().triangles(sites).features[0].properties.circumcenter, v = [ 5, 4.981069 ], w = [ -180 + v[0], -v[1] ];
    //   test.ok( (Math.abs(u[0]-v[0]) < 1e-6 && Math.abs(u[1]-v[1]) < 1e-6)
    //    || (Math.abs(u[0]-w[0]) < 1e-6 && Math.abs(u[1]-w[1]) < 1e-6) );
    //   test.end();
    // });

    // tape("geoVoronoi’s delaunay does not list fake points in its triangles", function(test) {
    //   const u = geoVoronoi.geoVoronoi()(sites);
    //   test.equal(Math.max(...u.delaunay.delaunay.triangles), sites.length - 1);
    //   test.end();
    // });
}
