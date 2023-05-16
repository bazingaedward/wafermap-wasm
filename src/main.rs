mod rotate;
mod geometry;
use rotate::*;

fn main(){
    let points = vec![Point{x: 0.0, y: 0.0}, Point{x: 1.0, y: 1.0}];
    println!("{:?}",translate_points(points, Point{x: 0.0, y: 0.0}, Point{x: 1.0, y: 1.0}));

}