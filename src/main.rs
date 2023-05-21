mod rotate;
mod geometry;
use std::vec;

use rotate::*;

fn main(){
    // let origin: OriginalConf = OriginalConf{
    //     notch: NotchDirection::Right,
    //     posX: XAxisDirection::Right,
    //     posY: YAxisDirection::Up,
    // };
    // println!("{:?}, {:?}",notch_to_radian(origin, NotchDirection::Right));
    // let points = vec![geometry::Point{x: 10.0, y: 20.0}];
    // println!("{:?}",rotate_points_by_notch(points, origin, NotchDirection::Down, true));

    // println!("{:?}",std::f64::consts::PI as i32);

    // let points = vec![geometry::Point{x: 10.0, y: 20.0}];
    // println!("{:?}",convert_points_by_axis(points, XAxisDirection::Right, YAxisDirection::Down, XAxisDirection::Left, YAxisDirection::Up));

    
    let origin: OriginalConf = OriginalConf{
        notch: NotchDirection::Right,
        posX: XAxisDirection::Right,
        posY: YAxisDirection::Up,
    };
    println!("{:?}",rotate_row_col(1,3,origin, NotchDirection::Down));

}