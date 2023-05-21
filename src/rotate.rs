// mod crate::geometry;
use crate::geometry::Point;

#[derive(Copy, Clone, Debug)]
pub struct PointWithOrigin {
    pub x: f64,
    pub y: f64,
    pub ox: f64,
    pub oy: f64,
}

// 晶圆图Notch方向

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum NotchDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum XAxisDirection {
    Left,
    Right,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum YAxisDirection {
    Up,
    Down,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct OriginalConf {
    pub notch: NotchDirection,
    pub posX: XAxisDirection,
    pub posY: YAxisDirection,
}

#[derive(Debug)]
pub struct StandardOptions {
    notch: NotchDirection,
    posX: XAxisDirection,
    posY: YAxisDirection,
    canvasAxisX: XAxisDirection,
    canvasAxisY: YAxisDirection,
    centerDieX: i32,
    centerDieY: i32,
    centerReticleX: i32,
    centerReticleY: i32,
}

pub const STANDARD_OPTIONS: StandardOptions = StandardOptions {
    notch: NotchDirection::Down,
    posX: XAxisDirection::Right,
    posY: YAxisDirection::Up,
    canvasAxisX: XAxisDirection::Right,
    canvasAxisY: YAxisDirection::Down,
    centerDieX: 0,
    centerDieY: 0,
    centerReticleX: 0,
    centerReticleY: 0,
};

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
pub fn translate_points(points: Vec<Point>, from: Point, to: Point) -> Vec<Point> {
    if to.x == from.x && to.y == from.y {
        return points;
    }

    let df_x = to.x - from.x;
    let df_y = to.y - from.y;
    let mut points: Vec<Point> = points;
    for p in points.iter_mut() {
        p.x += df_x;
        p.y += df_y;
    }
    return points;
}

/**
 * 坐标旋转
 * 基于原点 x=0,y=0 旋转
 *
 * @param {Point[]} points 坐标列表
 * @param {{
 *     notch: NotchDirection;
 *     posX: XAxisDirection;
 *     posY: YAxisDirection;
 *   }} originalConf 原始 notch
 * @param {NotchDirection} targetNotch 目标 notch
 * @param {Boolean} round 是否对坐标四舍五入
 * @return {*}  {Point[]}
 *
 * 示例:
 * const points = [
 *   { x: 10, y: 20 },
 * ];
 * rotatePointsByNotch(points, {notch: 'right', posX: 'right', posY: 'up'}, 'down'); // 输出： [{ x: 20, y: -10 }]
 */
pub fn rotate_points_by_notch(
    points: Vec<Point>,
    original_conf: OriginalConf,
    target_notch: NotchDirection,
    round: bool,
) -> Vec<Point> {

    let radian = notch_to_radian(original_conf, target_notch);
    if radian == 0.0 {
        return points;
    }
    println!("{:?}, {:?}, {:?}", original_conf.notch,target_notch ,radian);

    let mut points: Vec<Point> = points;
    for p in points.iter_mut() {
        let x = p.x;
        let y = p.y;
        if radian == 0.0 {
            continue;
        }
        let new_x = x * (radian as f64).cos() + y * (radian as f64).sin();
        let new_y = -x * (radian as f64).sin() + y * (radian as f64).cos();
        if round {
            p.x = new_x.round();
            p.y = new_y.round();
        } else {
            p.x = new_x;
            p.y = new_y;
        }
    }

    return points
}

/**
 * 基于notch方向计算旋转角度
 */
pub fn notch_to_radian(original_conf: OriginalConf, target_notch: NotchDirection)->f32{
    if original_conf.notch == target_notch {
        return 0.0;
    }

    let is_same_x = original_conf.posX == STANDARD_OPTIONS.posX;
    let is_same_y = original_conf.posY == STANDARD_OPTIONS.posY;
    let rotate_direction = if is_same_x ^ is_same_y {-1.0} else {1.0};
    
    let OriginalConf{ notch, posX, posY } = original_conf;

    // 对称方向 180
    match (notch, target_notch) {
        (NotchDirection::Left, NotchDirection::Right) => {
            return std::f64::consts::PI as f32 * rotate_direction;
        },
        (NotchDirection::Right, NotchDirection::Left) => {
            return std::f64::consts::PI as f32 * rotate_direction;
        },
        (NotchDirection::Up, NotchDirection::Down) => {
            return std::f64::consts::PI as f32 * rotate_direction;
        },
        (NotchDirection::Down, NotchDirection::Up) => {
            return std::f64::consts::PI as f32 * rotate_direction;
        },
        _ => {}
    }

    // 相邻顺时针方向 90
    match (notch, target_notch) {
        (NotchDirection::Left, NotchDirection::Up) => {
            return (std::f64::consts::PI / 2.0) as f32 * rotate_direction;
        },
        (NotchDirection::Up, NotchDirection::Right) => {
            return (std::f64::consts::PI / 2.0) as f32 * rotate_direction;
        },
        (NotchDirection::Right, NotchDirection::Down) => {
            println!("right down, {}", std::f64::consts::PI / 2.0);
            return (std::f64::consts::PI / 2.0) as f32 * rotate_direction;
        },
        (NotchDirection::Down, NotchDirection::Left) => {
            return (std::f64::consts::PI / 2.0) as f32 * rotate_direction;
        },
        _ => {}
    }
    
    // 相邻逆时针方向 90
    return ((std::f64::consts::PI * 3.0) / 2.0) as f32 * rotate_direction;

}

/**
 * 根据轴方向旋转坐标
 *
 * @param {Point[]} points
 * @param {("left" | "right")} xAxisDirection
 * @param {("up" | "down")} yAxisDirection
 * @param {("left" | "right")} targetXAxisDirection
 * @param {("up" | "down")} targetYAxisDirection
 * @return {*}  {Point[]}
 *
 * 示例:
 * const points = [
 *   { x: 10, y: 20 },
 * ];
 * convertPointsByAxis(points, 'right', 'down', 'left', 'up'); // 输出： [{ x: -10, y: -20 }]
 */
pub fn convert_points_by_axis(
    points: Vec<Point>,
    x_axis_direction: XAxisDirection,
    y_axis_direction: YAxisDirection,
    target_x_axis_direction: XAxisDirection,
    target_y_axis_direction: YAxisDirection,
)->Vec<Point>{
    let is_same_x = x_axis_direction == target_x_axis_direction;
    let is_same_y = y_axis_direction == target_y_axis_direction;

    let mut points: Vec<Point> = points;
    for p in points.iter_mut() {
        let x = p.x;
        let y = p.y;
        if !is_same_x {
            p.x = -x;
        }
        if !is_same_y {
            p.y = -y;
        }
    }
    return points
}

/**
 * 根据Notch方向转换 row column
 *
 * @param {number} row
 * @param {number} column
 * @param {{
 *     notch: NotchDirection;
 *     posX: XAxisDirection;
 *     posY: YAxisDirection;
 *   }} originalConf
 * @param {NotchDirection} targetNotch
 * @return {*}  {{
 *   row: number;
 *   column: number;
 * }}
 *
 * 示例:
 * rotateRowCol(1, 3, {notch: 'right', posX: 'right', posY: 'up'}, 'down'); // 输出：{row: 3, column: 1}
 */
pub fn rotate_row_col(
    row: i32,
    column: i32,
    original_conf: OriginalConf,
    target_notch: NotchDirection,
)->(i32, i32){
    let radian = notch_to_radian(original_conf, target_notch);
    if radian == 0.0 {
        return (row, column);
    }
    let (new_row, new_column) = if radian.abs() == std::f64::consts::PI as f32 {
        (row, column)
    } else {
        (column, row)
    };
    return (new_row, new_column);
}

/**
 * 根据Notch方向转换 width height
 *
 * @param {number} width
 * @param {number} height
 * @param {{
 *     notch: NotchDirection;
 *     posX: XAxisDirection;
 *     posY: YAxisDirection;
 *   }} originalConf
 * @param {NotchDirection} targetNotch
 * @return {*}  {{
 *   width: number;
 *   height: number;
 * }}
 *
 * 示例:
 * rotateSize(2, 4, {notch: 'right', posX: 'right', posY: 'up'}, 'down'); // 输出：{width: 4, height: 2}
 */
pub fn rotate_size(
    width: i32,
    height: i32,
    original_conf: OriginalConf,
    target_notch: NotchDirection,
)->(i32, i32){
    let (row, column) = rotate_row_col(height, width, original_conf, target_notch);
    return (column, row);
}


/**
 * 根据Notch方向转换 centerOffsetX centerOffsetY
 *
 * @param {number} centerOffsetX
 * @param {number} centerOffsetY
 * @param {{
 *     notch: NotchDirection;
 *     posX: XAxisDirection;
 *     posY: YAxisDirection;
 *   }} originalConf
 * @param {NotchDirection} targetNotch
 * @return {*}  {{
 *   centerOffsetX: number;
 *   centerOffsetY: number;
 * }}
 *
 * 示例:
 * rotateCenterOffset(2, 4, {notch: 'right', posX: 'right', posY: 'up'}, 'down'); // 输出：{centerOffsetX: 4, centerOffsetY: -2}
 */
pub fn rotate_center_offset(
    center_offset_x: f64,
    center_offset_y: f64,
    original_conf: OriginalConf,
    target_notch: NotchDirection,
)->(f32, f32){
    let points = rotate_points_by_notch(vec![Point{x: center_offset_x, y: center_offset_y}], original_conf, target_notch, true);
    return (points[0].x as f32, points[0].y as f32);
}

/**
 * 根据Notch方向转换 rowOffset colOffset
 *
 * @param {number} row
 * @param {number} column
 * @param {number} rowOffset
 * @param {number} colOffset
 * @param {{
 *     notch: NotchDirection;
 *     posX: XAxisDirection;
 *     posY: YAxisDirection;
 *   }} originalConf
 * @param {NotchDirection} targetNotch
 * @return {*}  {{
 *   rowOffset: number;
 *   colOffset: number;
 * }}
 *
 * 示例:
 * rotateRowColOffset(2, 2, 0, 1, {notch: 'right', posX: 'right', posY: 'up'}, 'down'); // 输出：{rowOffset: 1, colOffset: 1}
 */
pub fn rotate_row_col_offset(
    row: i32,
    column: i32,
    row_offset: i32,
    col_offset: i32,
    original_conf: OriginalConf,
    target_notch: NotchDirection,
)->(f64, f64){
    if original_conf.notch == target_notch {
        return (row_offset, col_offset);
    }

    // 按照 rowOffset, colOffset 坐标系，生成 row column 范围内的点
    let mut made_all_points: Vec<Point> = vec![];
    for x in 0..row {
        for y in 0..column {
            made_all_points.push(Point{x: x as f64, y: y as f64});
        }
    }

    // 将 row column 范围内的点基于 (0,0) 按照 notch => targetNotch 变更方向旋转
    let rotated_points = rotate_points_by_notch(made_all_points, original_conf, target_notch, true);

    // 匹配 rowOffset, colOffset 旋转后的点
    let matched_offset_point = rotated_points.iter().find(|&p| p.x == row_offset as f64 && p.y == col_offset as f64).unwrap();

    // 获取旋转后的最小坐标点
    let mut min_x = std::f64::MAX;
    let mut min_y = std::f64::MAX;
    for p in rotated_points.iter() {
        if p.x < min_x {
            min_x = p.x;
        }
        if p.y < min_y {
            min_y = p.y;
        }
    }

    // 平移最小坐标点到 0，0 的向量距离
    let translated_points = translate_points(vec![*matched_offset_point], Point { x: min_x, y: min_y }, Point { x: 0.0, y: 0.0 });

    return (translated_points[0].x, translated_points[0].y)


}
