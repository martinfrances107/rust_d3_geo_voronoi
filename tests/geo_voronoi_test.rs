#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod geo_voronoi_test {
    extern crate pretty_assertions;

    use approx::AbsDiffEq;
    use geo::algorithm::cyclic_match::CyclicMatch;
    use geo::coords_iter::CoordsIter;
    use geo::line_string;
    use geo::Coordinate;
    use geo::Geometry;
    use geo::LineString;
    use geo::MultiPoint;
    use geo::Point;

    use rust_d3_geo::data_object::FeatureCollection;
    use rust_d3_geo::data_object::FeatureProperty;
    use rust_d3_geo::stream::StreamDrainStub;
    use rust_d3_geo_voronoi::voronoi::GeoVoronoi;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_two_hemispheres() {
        println!("two points leads to two hemispheres.");
        let sites = MultiPoint(vec![Point::new(-20f64, -20f64), Point::new(20f64, 20f64)]);

        let mut gv: GeoVoronoi<StreamDrainStub<f64>, f64> =
            GeoVoronoi::new(Some(Geometry::MultiPoint(sites)));
        match gv.polygons(None) {
            None => {
                assert!(false, "Must return a DataObject<T>.");
            }
            Some(FeatureCollection(mut features)) => {
                let last_cell = line_string![
                    Coordinate {
                        x: -90.0,
                        y: 43.21917889371418
                    },
                    Coordinate { x: 180.0, y: -0. },
                    Coordinate {
                        x: 90.0,
                        y: -43.21917889371418
                    },
                    Coordinate { x: 0., y: 0. },
                    Coordinate {
                        x: -90.,
                        y: 43.21917889371418
                    }
                ];
                let g: Geometry<f64> = features.pop().unwrap().geometry.pop().unwrap();
                match g {
                    Geometry::Polygon(p) => {
                        let ls = p.exterior();
                        assert_eq!(last_cell, *ls);
                    }
                    _ => {
                        assert!(false, "expecting a polygon");
                    }
                };

                let first_cell = line_string![
                    Coordinate {
                        x: 0.0_f64,
                        y: 0.0_f64
                    },
                    Coordinate {
                        x: 90.0_f64,
                        y: -43.21917889371418_f64
                    },
                    Coordinate {
                        x: 180.0_f64,
                        y: -0.0_f64
                    },
                    Coordinate {
                        x: -90_f64,
                        y: 43.21917889371418_f64
                    },
                    Coordinate { x: 0_f64, y: 0_f64 },
                ];
                let g: Geometry<f64> = features.pop().unwrap().geometry.pop().unwrap();
                match g {
                    Geometry::Polygon(p) => {
                        let ls = p.exterior();
                        assert_eq!(first_cell, *ls);
                    }
                    _ => {
                        assert!(false, "expecting a polygon");
                    }
                };
            }
        }
    }

    #[test]
    pub fn voronoi_polygons_returns_polygons() {
        println!("geoVoronoi.polygons(sites) returns polygons.");
        let sites = MultiPoint(vec![
            Point::new(0f64, 0f64),
            Point::new(10f64, 0f64),
            Point::new(0f64, 10f64),
        ]);

        let mut gv: GeoVoronoi<StreamDrainStub<f64>, f64> =
            GeoVoronoi::new(Some(Geometry::MultiPoint(sites)));
        match gv.polygons(None) {
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
        let g = Geometry::MultiPoint(sites);
        let mut gv: GeoVoronoi<StreamDrainStub<f64>, f64> = GeoVoronoi::new(Some(g));
        let _u = gv.polygons(None);
    }

    // it("geoVoronoi.polygons([no valid site]) returns an empty collection.", () => {
    //     const sites = [[NaN, -1], [4, NaN], [Infinity,10]];
    //     const u = geoVoronoi.geoVoronoi(sites).polygons();
    //     assert.deepStrictEqual(u.features, []);
    //   });

    //   it("geoVoronoi.polygons([1 site]) returns a Sphere.", () => {
    //     const sites = [[NaN, -1], [4, NaN], [5,10]];
    //     const u = geoVoronoi.geoVoronoi(sites).polygons();
    //     assert.strictEqual(u.features[0].type, "Feature");
    //     assert.strictEqual(u.features[0].geometry.type, "Sphere");
    //   });

    //   it("geoVoronoi.links() returns urquhart.", () => {
    //     assert.deepStrictEqual(geoVoronoi.geoVoronoi().links(sites).features.map(function(d) { return d.properties.urquhart; }), [ false, true, true ]);
    //   });

    // it("geoVoronoi.x() changes accessor.", () => {
    //     const sites = [{lon:10,lat:0}, {lon:3, lat:5}, {lon:-2, lat:5}];
    //     assert.deepStrictEqual(
    //       geoVoronoi.geoVoronoi().x(d => +d.lon).y(d => +d.lat) (sites).points,
    //       [ [ 10, 0 ], [ 3, 5 ], [ -2, 5 ] ]
    //     );
    //   });

    #[test]
    fn test_computes_the_hull() {
        let sites = MultiPoint(vec![
            Point::new(10f64, 0f64),
            Point::new(10f64, 10f64),
            Point::new(3f64, 5f64),
            Point::new(-2f64, 5f64),
            Point::new(0f64, 0f64),
        ]);
        let gv: GeoVoronoi<StreamDrainStub<f64>, f64> = GeoVoronoi::new(None);
        let hull = gv.hull(Some(Geometry::MultiPoint(sites)));
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

    #[test]
    fn computes_the_delaunay_mesh() {
        let sites = MultiPoint(vec![
            Point::new(10f64, 0f64),
            Point::new(10f64, 10f64),
            Point::new(3f64, 5f64),
            Point::new(-2f64, 5f64),
            Point::new(0f64, 0f64),
        ]);
        let gv: GeoVoronoi<StreamDrainStub<f64>, f64> =
            GeoVoronoi::new(Some(Geometry::MultiPoint(sites)));
        let mesh = gv.mesh(None);

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

    #[test]
    fn computes_the_polygons_mesh() {
        let sites = MultiPoint(vec![
            Point::new(10f64, 0f64),
            Point::new(10f64, 10f64),
            Point::new(3f64, 5f64),
            Point::new(-2f64, 5f64),
            Point::new(0f64, 0f64),
        ]);

        let ls_string_golden: Vec<String> = vec![
            "-175 -5/-175 -5".into(),
            "-175 -5/0 3".into(),
            "-175 -5/1 15".into(),
            "-175 -5/5 0".into(),
            "-175 -5/8 5".into(),
            "0 3/1 15".into(),
            "0 3/5 0".into(),
            "1 15/8 5".into(),
            "5 0/8 5".into(),
        ];
        let gv: GeoVoronoi<StreamDrainStub<f64>, f64> = GeoVoronoi::new(None);
        let cell_mesh_maybe = gv.cell_mesh(Some(Geometry::MultiPoint(sites)));
        match cell_mesh_maybe {
            Some(cell_mesh) => {
                let c_string: Vec<Vec<String>> = cell_mesh
                    .iter()
                    .map(|ls| {
                        let d = ls.coords_iter();
                        let mut e: Vec<String> = d
                            .map(|p| format!("{} {}", p.x.round(), p.y.round()))
                            .collect::<Vec<String>>();
                        e.sort();
                        e
                    })
                    .collect();
                let mut ls_string: Vec<String> = c_string.iter().map(|ls| ls.join("/")).collect();
                ls_string.sort();
                assert_eq!(ls_string, ls_string_golden);
            }
            None => {
                panic!("expecting a MultiLineString");
            }
        }
    }

    #[test]
    fn geo_voronoi_finds_p() {
        let sites = MultiPoint(vec![
            Point::new(10f64, 0f64),
            Point::new(10f64, 10f64),
            Point::new(3f64, 5f64),
            Point::new(-2f64, 5f64),
            Point::new(0f64, 0f64),
        ]);
        let mut voro: GeoVoronoi<StreamDrainStub<f64>, f64> =
            GeoVoronoi::new(Some(Geometry::MultiPoint(sites.clone())));

        assert_eq!(voro.find(Coordinate { x: 1.0, y: 1.0 }, None), Some(4));
        // TODO bug ... strange bug/hang ... unless I define voro twice.
        let mut voro2: GeoVoronoi<StreamDrainStub<f64>, f64> =
            GeoVoronoi::new(Some(Geometry::MultiPoint(sites)));
        assert_eq!(
            voro2.find(Coordinate { x: 1.0, y: 1.0 }, Some(4.0)),
            Some(4)
        );
    }

    #[test]
    fn geo_voronoi_link() {
        let sites = Geometry::MultiPoint(MultiPoint(vec![
            Point::new(0f64, 0f64),
            Point::new(10f64, 0f64),
            Point::new(0f64, 10f64),
        ]));
        let mut gv: GeoVoronoi<StreamDrainStub<f64>, f64> = GeoVoronoi::new(None);
        match gv.links(Some(sites)) {
            Some(FeatureCollection(features)) => {
                let mut out: Vec<f64> = features
                    .iter()
                    .map(|d| match d.properties[0] {
                        FeatureProperty::Source(p) => p.x,
                        _ => {
                            panic!("Did not find a source property.");
                        }
                    })
                    .collect();
                // The JS version does not sort here..
                // BUT as we are using hashmaps in geo_edges
                // the order of the FeatureStructs is unpredicatable.
                out.sort_by(|a, b| a.partial_cmp(b).unwrap());

                assert_eq!(out, vec![0.0, 0.0, 10.0])
            }
            None => {
                panic!("Was expecting a feature collection.")
            }
        }
    }

    #[test]
    fn geo_voronoi_triangles_returns_geojson() {
        let sites = Geometry::MultiPoint(MultiPoint(vec![
            Point::new(0f64, 0f64),
            Point::new(10f64, 0f64),
            Point::new(0f64, 10f64),
        ]));
        let gv: GeoVoronoi<StreamDrainStub<f64>, f64> = GeoVoronoi::new(None);
        match gv.triangles(Some(sites)) {
            Some(FeatureCollection(features)) => {
                assert_eq!(features.len(), 1);
            }
            None => {
                panic!("Was expecting a feature collection.")
            }
        }
    }

    #[test]
    fn geo_voronoi_links_returns_urquhart_graph() {
        let sites = Geometry::MultiPoint(MultiPoint(vec![
            Point::new(0f64, 0f64),
            Point::new(10f64, 0f64),
            Point::new(0f64, 10f64),
        ]));

        let mut gv: GeoVoronoi<StreamDrainStub<f64>, f64> = GeoVoronoi::new(None);
        match gv.links(Some(sites)) {
            Some(FeatureCollection(features)) => {
                let mut results: Vec<bool> = Vec::new();
                for fs in features {
                    // Extract the Urquhart property from the FeatureStruct.
                    let fs_u: Vec<bool> = fs
                        .properties
                        .iter()
                        .filter_map(|fs| match fs {
                            FeatureProperty::Urquhart(u) => Some(*u),
                            _ => None,
                        })
                        .collect();
                    assert_eq!(fs_u.len(), 1);
                    results.push(fs_u[0]);
                }
                // The JS version does not sort here..
                // BUT as we are using hashmaps in geo_edges
                // the order of the FeatureStructs is unpredicatable.
                // TODO: future work would be to link the boolean result to
                // the point?
                results.sort_by(|a, b| a.partial_cmp(b).unwrap());
                assert_eq!(results, [false, true, true]);
            }
            None => {
                panic!("Was expecting a feature collection.")
            }
        }
    }

    #[test]
    fn geo_voronoi_links_returns_circumcenters() {
        let sites = Geometry::MultiPoint(MultiPoint(vec![
            Point::new(0f64, 0f64),
            Point::new(10f64, 0f64),
            Point::new(0f64, 10f64),
        ]));

        let gv: GeoVoronoi<StreamDrainStub<f64>, f64> = GeoVoronoi::new(None);
        match gv.triangles(Some(sites)) {
            Some(FeatureCollection(features)) => {
                println!("features {:?}", features);
                match &features[0].properties[0] {
                    FeatureProperty::Circumecenter(u) => {
                        println!("c {:?}", u);
                        let v = Coordinate {
                            x: 5.0,
                            y: 4.981069,
                        };
                        let w = Coordinate {
                            x: -180.0 + v.x,
                            y: -1.0 * v.y,
                        };
                        if ((u.x - v.x).abs() < 1e-6 && (u.y - v.y).abs() < 1e-6)
                            || ((u.x - w.x).abs() < 1e-6 && (u.y - w.y).abs() < 1e-6)
                        {
                            assert!(true);
                        } else {
                            assert!(false);
                        }
                    }
                    _ => {
                        panic!("was expecting a circumcenter");
                    }
                }
            }
            None => {
                panic!("Was expecting a feature collection.")
            }
        }
    }
    // tape("geoVoronoi’s delaunay does not list fake points in its triangles", function(test) {
    //   const u = geoVoronoi.geoVoronoi()(sites);
    //   test.equal(Math.max(...u.delaunay.delaunay.triangles), sites.length - 1);
    //   test.end();
    // });

    #[test]
    fn test_does_not_list_fake_points() {
        println!("geoVoronoi’s delaunay does not list fake points in its triangles");
        let sites = vec![
            Point::new(0f64, 0f64),
            Point::new(10f64, 0f64),
            Point::new(0f64, 10f64),
        ];

        let u: GeoVoronoi<StreamDrainStub<f64>, f64> =
            GeoVoronoi::new(Some(Geometry::MultiPoint(MultiPoint(sites.clone()))));
        assert_eq!(
            u.geo_delaunay.unwrap().delaunay.triangles.iter().max(),
            Some(&(sites.len() - 1usize))
        );
    }
}
