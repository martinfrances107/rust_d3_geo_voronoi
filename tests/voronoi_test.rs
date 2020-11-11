mod voronoi_test {
    extern crate pretty_assertions;
    use delaunator::Point;
    use rust_d3_geo::data_object::DataObject;
    use rust_d3_geo::data_object::FeatureGeometry;
    use rust_d3_geo_voronoi::voronoi::Voronoi;

    #[cfg(test)]
    use pretty_assertions::assert_eq;

    #[test]
    pub fn voronoi_polygons_returns_polygons() {
        println!("geoVoronoi.polygons(sites) returns polygons.");
        let sites = DataObject::Vec(vec![
            Point { x: 0f64, y: 0f64 },
            Point { x: 10f64, y: 0f64 },
            Point { x: 0f64, y: 10f64 },
        ]);

        let v = Voronoi::new(sites);
        match v.polygons(DataObject::Blank) {
            None => {
                assert!(false, "Must return a DataObject.");
            }
            Some(DataObject::FeatureCollection { features }) => {
                println!("Found a Features Collection.");
                let g = &features[0].geometry[0];
                match g {
                    FeatureGeometry::Polygon { coordinates } => {
                        let u = coordinates[0][0].clone();
                        let v = Point {
                            x: -175f64,
                            y: -4.981069f64,
                        };
                        assert!((u.x - v.x).abs() < 1e-6f64);
                        assert!((u.y - v.y).abs() < 1e-6f64);
                    }
                    _ => {
                        assert!(false, "Expected a polygon object.");
                    }
                }
            }
            _ => {
                assert!(false, "Expected a FeaturesCollection.");
            }
        }
    }

    #[test]
    fn test_polygon_tollerates_nan() {
        println!("geoVoronoi.polygons(sites) tolerates NaN.");
        //var u = geoVoronoi.geoVoronoi().polygons(sites)[0][0], v = [ 5, 4.981069 ];
        //test.ok( (Math.abs(u[0]-v[0]) < 1e-6) && (Math.abs(u[1]-v[1]) < 1e-6) );
        let sites = DataObject::Vec(vec![
            Point { x: 0f64, y: 0f64 },
            Point { x: 2f64, y: 1f64 },
            Point {
                x: f64::NAN,
                y: -1f64,
            },
            Point {
                x: 4f64,
                y: f64::NAN,
            },
            Point { x: 5f64, y: 10f64 },
        ]);
        let u = Voronoi::new(sites).polygons(DataObject::Blank);
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

    // tape("geoVoronoi.hull() computes the hull.", function(test) {
    //   var sites = [[10,0],[10,10],[3,5],[-2,5],[0,0]];
    //   test.deepEqual(
    //   	geoVoronoi.geoVoronoi().hull(sites),
    //   	{ type: 'Polygon', coordinates: [ [ [ 10, 10 ], [ 10, 0 ], [ 0, 0 ], [ -2, 5 ], [ 10, 10 ] ] ] }
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
