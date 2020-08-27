use num_traits::cast::FromPrimitive;
use num_traits::Float;
use num_traits::FloatConst;

pub enum FeatureGeometry<F>
where F: Float {
  Polygon { coordinates: Vec<usize> },
  LineString { coordinate: [[F; 2]; 2] },
}

pub enum FeatureProperty<F>
where
  F: Float,
{
  Circumecenter(Vec<[F; 2]>),
  Length(F),
  Source(usize),
  Target(usize),
  Urquhart(F),
}

pub struct FeatureStruct<F>
where
  F: Float,
{
  properties: Vec<FeatureProperty<F>>,
  geometry: FeatureGeometry<F>,
}

/// The input data type use in D3
///  Can be special object ( DataObject )
///  or a vector of stuff
///  Null - here a blank.

pub enum DataObject<F>
where
  F: Float,
{
  //   * Point - a single position.
  // * MultiPoint - an array of positions.
  // * LineString - an array of positions forming a continuous line.
  /// MultiLineString - an array of arrays of positions forming several lines.
  MultiLineString {
    coordinates: Vec<[F; 2]>,
  },
  // * Polygon - an array of arrays of positions forming a polygon (possibly with holes).
  // * MultiPolygon - a multidimensional array of positions forming multiple polygons.
  // * GeometryCollection - an array of geometry objects.
  Feature(FeatureStruct<F>),
  /// FeatruesCollection - An array of feature objects.
  FeaturesCollection {
    features: Vec<FeatureStruct<F>>,
  },
  // A feature containing one of the above geometry objects.
  // Polygon{coordinates: Vec<usize>},
}

pub enum DataType<F>
where
  F: Float,
{
  Object(DataObject<F>),
  Vec(Vec<usize>),
  Blank,
}
