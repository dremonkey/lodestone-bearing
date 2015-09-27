/// The main crate for lodestone-bearing
/// 
/// ## Overview
/// 
/// Calculates the initial [bearing](http://www.movable-type.co.uk/scripts/latlong.html)
/// between two points.

// Third party crates
extern crate lodestone_point;

use lodestone_point::FeaturePoint;

/// Convert from -180° ... +180° to 0° ... 360°
pub extern fn abs_bearing(bearing: f64) -> f64 {
  let mut bearing = bearing % 360.0;
  if bearing < 0.0 { bearing = bearing + 360.0 };
  bearing
}

/// Calculate bearing between -180.0° ... +180.0°
pub extern fn bearing(
    from_point: &FeaturePoint,
    to_point: &FeaturePoint) -> f64 {

  let coord1 = from_point.coordinates();
  let coord2 = to_point.coordinates();

  let lng1 = coord1[0].to_radians();
  let lng2 = coord2[0].to_radians();
  let lat1 = coord1[1].to_radians();
  let lat2 = coord2[1].to_radians();
  let a = (lng2 - lng1).sin() * lat2.cos();
  let b = lat1.cos() * lat2.sin() -
          lat1.sin() * lat2.cos() * (lng2 - lng1).cos();

  a.atan2(b).to_degrees()
}

#[cfg(test)]
mod tests {
  use lodestone_point::FeaturePoint;
  use super::{abs_bearing, bearing};

  #[test]
  fn test_abs_bearing() {
    let x1 = 30.0;
    let x2 = -30.0;
    let x3 = 710.0;
    let x4 = -180.0;

    assert_eq!(abs_bearing(x1), 30.0);
    assert_eq!(abs_bearing(x2), 330.0);
    assert_eq!(abs_bearing(x3), 350.0);
    assert_eq!(abs_bearing(x4), 180.0);
  }

  #[test]
  fn test_simple() {
    let pt1 = FeaturePoint::new(vec![0.0, 0.0]);
    let pt2 = FeaturePoint::new(vec![1.0, 0.0]);
    let pt3 = FeaturePoint::new(vec![0.0, 1.0]);
    let pt4 = FeaturePoint::new(vec![1.0, 1.0]);

    assert_eq!(bearing(&pt1, &pt2), 90.0);
    assert_eq!(bearing(&pt1, &pt3), 0.0);
    assert_eq!(bearing(&pt3, &pt1), 180.0);
    assert_eq!(bearing(&pt2, &pt1), -90.0);
    assert_eq!(bearing(&pt1, &pt4) - 44.9956 < 1e-4, true); 
  }

  #[test]
  fn test_sf_to_ny() {
    let sf = vec![-122.4167,37.7833];
    let ny = vec![-74.0059,40.7127];

    let sf_point = FeaturePoint::new(sf);
    let ny_point = FeaturePoint::new(ny);
    let brng = bearing(&sf_point, &ny_point);
    
    assert_eq!(brng, 69.91944547551958);
  }

  #[test]
  fn test_sf_to_la() {
    let sf = vec![-122.4167,37.7833];
    let la = vec![-118.2500,34.0500];

    let sf_point = FeaturePoint::new(sf);
    let la_point = FeaturePoint::new(la);
    let brng = bearing(&sf_point, &la_point);

    assert_eq!(brng, 136.64918588053285);
  }
}
