/// The main crate for lodestone-bearing
/// 
/// ## Overview
/// 
/// Calculates the initial [bearing](http://www.movable-type.co.uk/scripts/latlong.html)
/// between two points.

// Standard lib packages
use std::f64::consts::{PI};

// Third party packages
extern crate lodestone_point;

use lodestone_point::FeaturePoint;

pub extern fn bearing(
    from_point: &FeaturePoint,
    to_point: &FeaturePoint) -> f64 {

  let coord1 = from_point.coordinates();
  let coord2 = to_point.coordinates();

  let lng1 = to_rad(coord1[0]);
  let lng2 = to_rad(coord2[0]);
  let lat1 = to_rad(coord1[1]);
  let lat2 = to_rad(coord2[1]);
  let a = (lng2 - lng1).sin() * lat2.cos();
  let b = lat1.cos() * lat2.sin() -
          lat1.sin() * lat2.cos() * (lng2 - lng1).cos();

  to_deg(a.atan2(b))
}

fn to_rad(degree: f64) -> f64 {
  degree * PI / 180.0
}

fn to_deg(radian: f64) -> f64 {
  radian * 180.0 / PI
}

#[cfg(test)]
mod tests {
  use lodestone_point::FeaturePoint;
  use super::bearing;

  #[test]
  fn test_sf_to_ny() {
    let sf = vec![-122.4167,37.7833];
    let ny = vec![-74.0059,40.7127];

    let sf_point = FeaturePoint::new(sf);
    let ny_point = FeaturePoint::new(ny);
    let brng = bearing(&sf_point, &ny_point);
    
    assert_eq!(brng, 69.9194454755196);
  }

  #[test]
  fn test_sf_to_la() {
    let sf = vec![-122.4167,37.7833];
    let la = vec![-118.2500,34.0500];

    let sf_point = FeaturePoint::new(sf);
    let la_point = FeaturePoint::new(la);
    let brng = bearing(&sf_point, &la_point);

    assert_eq!(brng, 136.6491858805329);
  }
}
