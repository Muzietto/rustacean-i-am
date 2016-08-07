#![allow(warnings)]
pub struct Nil;

pub struct Pair(i32, f64);

#[derive(Debug, Copy, Clone)]
pub struct Point {
  pub x: f64,
  pub y: f64,
}//;

pub struct Rectangle {
  pub p1: Point,
  pub p2: Point,
}
