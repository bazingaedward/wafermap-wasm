mod lib;

fn main(){
    let rect = [lib::Point{x: 0.0, y: 0.0}, lib::Point{x: 1.0, y: 1.0}];
    let circle = lib::Circle{x: 0.5, y: 0.5, radius: 0.5};
    let result = lib::is_rect_in_circle(rect, circle);
    println!("{:?}", result);
}