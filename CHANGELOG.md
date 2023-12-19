# Changelog

## [0.10.0] - 19th Dec 2023

Now using idomatic TryFrom where possible

commit 1db90fd34f0d87aaaa8e9eb4c2d4e356aa29b5b4
Date:   Thu Nov 30 17:48:25 2023 +0000

    Breaking change: Split Voronoi::triangles() into two functions.

commit 32daeafe5d4228a204bdba0d99836778e623dff7
Date:   Thu Nov 30 16:47:43 2023 +0000

    Breaking change: Split Voronoi::polygons() into two functions.

commit 33195b10a23b0803b61bf77ccd2c3d29f86cc5c9
Date:   Thu Nov 30 14:34:58 2023 +0000

    Breaking change: Split Voronoi::mesh() into two functions.

commit 9dacfdf42a3a6d234abf91e0441bb7085199eb52
Date:   Thu Nov 30 14:14:46 2023 +0000

    Breaking change: Split Voronoi::links() into two functions.

commit d868e319a97a0c303f7699431bbbb811ae739504
Date:   Thu Nov 30 12:51:23 2023 +0000

    Breaking change: Split Voronoi::hull() into two functions.

commit 66fdfb5bcb8ddd2192df250ca9ceb633503947d8
Date:   Thu Nov 30 10:57:24 2023 +0000

    Breaking change: Split Voronoi::cell_mesh() into two functions.

commit 315e8ce74812e083e7501c04d9036023853f148f
Date:   Thu Nov 30 10:08:51 2023 +0000

    Breaking Change: Delaunay "new" functions that return Option are not idomatic

    ```rustlang
    -    pub fn new(points: &[Coord<T>]) -> Option<Self> {
    +    fn try_from(points: &Vec<Coord<T>>) -> Result<Self, NotEnoughPointsError> {
    ```

## [0.1.3] - 11th Feb 2023

Cleaned up benchmark scripts "start", "build" and "serve".
Now using packages rimraf, and serve.
Updated README instructions.

Updated Cargo packages.
Expect to see performance boosts from rust_d3_geo when rendering to SVG.
