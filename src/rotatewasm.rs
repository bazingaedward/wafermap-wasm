use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// #[derive(Copy, Clone, Debug)]
// pub struct Point {
//     pub x: f64,
//     pub y: f64,
// }

/**
 * 坐标平移
 * 基于起始坐标和目标坐标平移
 * @param {Point[]} points 坐标列表
 * @param {Point} from  起始坐标（可以为任意一点）
 * @param {Point} to 目标坐标
 * @return {*}  {Point[]}
 *
 * 示例:
 * const points = [
 *   { x: 10, y: 20 },
 * ];
 * translatePoints(points, {x: 1, y: 1}, {x: 0, y: 0}); // 输出： [{ x: 9, y: 19 }]
 */
#[wasm_bindgen] 
pub fn translate_points(from: JsValue) -> bool {
    return true
    // if to.x == from.x && to.y == from.y {
    //     return points;
    // }

    // let df_x = to.x - from.x;
    // let df_y = to.y - from.y;
    // let mut points: Vec<Point> = points;
    // for p in points.iter_mut() {
    //     p.x += df_x;
    //     p.y += df_y;
    // }
    // return points;
}
