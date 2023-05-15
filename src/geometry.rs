use std::collections::VecDeque;


use wasm_bindgen::prelude::*;

#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

#[derive(Copy, Clone)]
pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

// 判断坐标点是否在圆内
pub fn is_point_in_circle(point: Point, circle: Circle)->bool {
    return (point.x-circle.x).powf(2.0) + (point.y-circle.y).powf(2.0) <= circle.radius.powf(2.0);
}

/* 获取各个点是否在圆内的状态列表
* @param rect
* @param circle
* @returns
*/

#[allow(dead_code)]
pub fn is_rect_in_circle(rect: [Point;2], circle: Circle)->VecDeque<bool>{
    let p1 = rect[0];
    let p2 = rect[1];
    let p3 = Point{x: p1.x, y:p2.y};
    let p4 = Point{x:p2.x, y:p1.y};

    return [p1, p2, p3, p4].iter().map(|&p| is_point_in_circle(p, circle)).collect();
}

// 矩形是否与圆相交
pub fn is_rect_cross_circle(rect: [Point;2], circle:Circle)->bool{
    let p1 = rect[0];
    let p2 = rect[1];
    let p3 = Point{x: p1.x, y:p2.y};
    let p4 = Point{x:p2.x, y:p1.y};

    return [p1, p2, p3, p4].iter().any(|&p| is_point_in_circle(p, circle));
}