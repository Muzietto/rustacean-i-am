#![allow(warnings)]
mod funzies;
mod structs;

fn main() {
  let point: structs::Point = structs::Point { x: 0.3, y: 0.4 };
  let point2: structs::Point = structs::Point { x: 0.5, y: 0.5 };

  println!("point coordinates: ({}, {})", point.x, point.y);

  let structs::Point { x: my_x, y: my_y } = point;

  println!("MY point coordinates: ({}, {})", my_x, my_y);

  let _rectangle = structs::Rectangle {
    p1: structs::Point { x: my_x + 1.0, y: my_y + 1.0 },
    p2: point,
  };

  println!("rectangle is: ({}, {})({}, {})", 
  _rectangle.p1.x, 
  _rectangle.p1.y, 
  _rectangle.p2.x, 
  _rectangle.p2.y);

  println!("area _rectangle is {}", funzies::rect_area(_rectangle));
  println!("area squarre is {}", funzies::rect_area(funzies::squarre(point2, 2)));
}
