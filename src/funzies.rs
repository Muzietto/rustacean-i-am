#![allow(warnings)]
use structs;

pub fn rect_area(rect: structs::Rectangle) -> f64 {
  (rect.p2.x - rect.p1.x) * (rect.p2.y - rect.p1.y)
}
pub fn squarre(point: structs::Point, side: i32) -> structs::Rectangle {
  structs::Rectangle {
    p1: structs::Point { x: point.x, y: point.y },
    p2: structs::Point { x: point.x + side as f64, y: point.y + side as f64},
  }
}
