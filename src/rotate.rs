use crate::geometry::Point;
// 晶圆图Notch方向
enum NotchDirection {
    Left,
    Right,
    Up,
    Down,
}

enum XAxisDirection {
    Left,
    Right,
}

enum YAxisDirection {
    Up,
    Down,
}

struct OriginalConf {
    notch: NotchDirection,
    posX: XAxisDirection,
    posY: YAxisDirection,
}

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
fn rotate_points_by_notch(
    points: Vec<Point>,
    originalConf: OriginalConf,
    targetNotch: NotchDirection,
    round: bool,
) -> Vec<Point> {
    // TODO: 函数完成
    let radian = notchDiff2radian(originalConf, targetNotch);
    if radian == 0 {
        return points;
    }

    // TODO: mut points 验证
    // TODO: 数学库导入
    let mut points: Vec<Point> = points;
    for p in points.iter_mut() {
        p.x += df_x;
        p.y += df_y;
    }
    return points;

    // points.forEach((p) => {
    //   const { x, y } = p;
    //   if (!isNumber(x) || !isNumber(y) || radian === 0) return;
    //   let newX = x * Math.cos(radian) + y * Math.sin(radian);
    //   let newY = -x * Math.sin(radian) + y * Math.cos(radian);
    //   if (round) {
    //     newX = Math.round(newX);
    //     newY = Math.round(newY);
    //   }
    //   p.x = newX;
    //   p.y = newY;
    // });
    // return points as Point[];
}

fn notchDiff2radian(originalConf: OriginalConf, targetNotch: NotchDirection) -> number {
    // const { notch, posX, posY } = originalConf;
    //     if (notch === targetNotch) return 0;
    //     const isSameX = posX === STANDARD_OPTIONS.posX;
    //     const isSameY = posY === STANDARD_OPTIONS.posY;
    //     const rotateDirection = Number(!isSameX) ^ Number(!isSameY) ? -1 : 1;

    //     // 对称方向 180
    //     if (
    //       (notch === 'left' && targetNotch === 'right') ||
    //       (notch === 'right' && targetNotch === 'left') ||
    //       (notch === 'up' && targetNotch === 'down') ||
    //       (notch === 'down' && targetNotch === 'up')
    //     ) {
    //       return Math.PI * rotateDirection;
    //     }
    //     // 相邻顺时针方向 90
    //     if (
    //       (notch === 'left' && targetNotch === 'up') ||
    //       (notch === 'up' && targetNotch === 'right') ||
    //       (notch === 'right' && targetNotch === 'down') ||
    //       (notch === 'down' && targetNotch === 'left')
    //     ) {
    //       return (Math.PI / 2) * rotateDirection;
    //     }
    //     // 相邻逆时针方向 90
    //     return ((Math.PI * 3) / 2) * rotateDirection;
}

//     const radian = notchDiff2radian(originalConf, targetNotch);
//     if (radian === 0) {
//       return points;
//     }
//     points.forEach((p) => {
//       const { x, y } = p;
//       if (!isNumber(x) || !isNumber(y) || radian === 0) return;
//       let newX = x * Math.cos(radian) + y * Math.sin(radian);
//       let newY = -x * Math.sin(radian) + y * Math.cos(radian);
//       if (round) {
//         newX = Math.round(newX);
//         newY = Math.round(newY);
//       }
//       p.x = newX;
//       p.y = newY;
//     });
//     return points as Point[];
//   }

//   /**
//    * notch 方向差异转换为 弧度
//    *
//    * @export
//    * @param {{
//    *     notch: NotchDirection;
//    *     posX: XAxisDirection;
//    *     posY: YAxisDirection;
//    *   }} originalConf
//    * @param {NotchDirection} targetNotch
//    * @return {*}  {number}
//    *
//    * 示例:
//    * notchDiff2radian({notch: 'right', posX: 'right', posY: 'up'}, 'down'); // 输出：Math.PI / 2
//    */
//   export function notchDiff2radian(
//     originalConf: {
//       notch: NotchDirection;
//       posX: XAxisDirection;
//       posY: YAxisDirection;
//     },
//     targetNotch: NotchDirection,
//   ): number {
//     const { notch, posX, posY } = originalConf;
//     if (notch === targetNotch) return 0;
//     const isSameX = posX === STANDARD_OPTIONS.posX;
//     const isSameY = posY === STANDARD_OPTIONS.posY;
//     const rotateDirection = Number(!isSameX) ^ Number(!isSameY) ? -1 : 1;

//     // 对称方向 180
//     if (
//       (notch === 'left' && targetNotch === 'right') ||
//       (notch === 'right' && targetNotch === 'left') ||
//       (notch === 'up' && targetNotch === 'down') ||
//       (notch === 'down' && targetNotch === 'up')
//     ) {
//       return Math.PI * rotateDirection;
//     }
//     // 相邻顺时针方向 90
//     if (
//       (notch === 'left' && targetNotch === 'up') ||
//       (notch === 'up' && targetNotch === 'right') ||
//       (notch === 'right' && targetNotch === 'down') ||
//       (notch === 'down' && targetNotch === 'left')
//     ) {
//       return (Math.PI / 2) * rotateDirection;
//     }
//     // 相邻逆时针方向 90
//     return ((Math.PI * 3) / 2) * rotateDirection;
//   }

//   /**
//    * 根据轴方向旋转坐标
//    *
//    * @param {Point[]} points
//    * @param {("left" | "right")} xAxisDirection
//    * @param {("up" | "down")} yAxisDirection
//    * @param {("left" | "right")} targetXAxisDirection
//    * @param {("up" | "down")} targetYAxisDirection
//    * @return {*}  {Point[]}
//    *
//    * 示例:
//    * const points = [
//    *   { x: 10, y: 20 },
//    * ];
//    * convertPointsByAxis(points, 'right', 'down', 'left', 'up'); // 输出： [{ x: -10, y: -20 }]
//    */
//   export function convertPointsByAxis(
//     points: Point[],
//     xAxisDirection: XAxisDirection,
//     yAxisDirection: YAxisDirection,
//     targetXAxisDirection: XAxisDirection,
//     targetYAxisDirection: YAxisDirection,
//   ): Point[] {
//     const isSameX = xAxisDirection === targetXAxisDirection;
//     const isSameY = yAxisDirection === targetYAxisDirection;

//     points.forEach((p) => {
//       const { x, y } = p;
//       if (!isNumber(x) || !isNumber(y)) return;
//       p.x = isSameX ? x : -x;
//       p.y = isSameY ? y : -y;
//     });
//     return points as Point[];
//   }

//   /**
//    * 根据Notch方向转换 row column
//    *
//    * @param {number} row
//    * @param {number} column
//    * @param {{
//    *     notch: NotchDirection;
//    *     posX: XAxisDirection;
//    *     posY: YAxisDirection;
//    *   }} originalConf
//    * @param {NotchDirection} targetNotch
//    * @return {*}  {{
//    *   row: number;
//    *   column: number;
//    * }}
//    *
//    * 示例:
//    * rotateRowCol(1, 3, {notch: 'right', posX: 'right', posY: 'up'}, 'down'); // 输出：{row: 3, column: 1}
//    */
//   export function rotateRowCol(
//     row: number,
//     column: number,
//     originalConf: {
//       notch: NotchDirection;
//       posX: XAxisDirection;
//       posY: YAxisDirection;
//     },
//     targetNotch: NotchDirection,
//   ): {
//     row: number;
//     column: number;
//   } {
//     const radian = notchDiff2radian(originalConf, targetNotch);
//     if (radian === 0) {
//       return {
//         row,
//         column,
//       };
//     }
//     const [newRow, newColumn] = [0, Math.PI].some((item) => item === Math.abs(radian))
//       ? [row, column]
//       : [column, row];

//     return {
//       row: newRow,
//       column: newColumn,
//     };
//   }

//   /**
//    * 根据Notch方向转换 width height
//    *
//    * @param {number} width
//    * @param {number} height
//    * @param {{
//    *     notch: NotchDirection;
//    *     posX: XAxisDirection;
//    *     posY: YAxisDirection;
//    *   }} originalConf
//    * @param {NotchDirection} targetNotch
//    * @return {*}  {{
//    *   width: number;
//    *   height: number;
//    * }}
//    *
//    * 示例:
//    * rotateSize(2, 4, {notch: 'right', posX: 'right', posY: 'up'}, 'down'); // 输出：{width: 4, height: 2}
//    */
//   export function rotateSize(
//     width: number,
//     height: number,
//     originalConf: {
//       notch: NotchDirection;
//       posX: XAxisDirection;
//       posY: YAxisDirection;
//     },
//     targetNotch: NotchDirection,
//   ): {
//     width: number;
//     height: number;
//   } {
//     // 此处复用 row col 的旋转逻辑
//     const { row, column } = rotateRowCol(height, width, originalConf, targetNotch);

//     return {
//       height: row,
//       width: column,
//     };
//   }

//   /**
//    * 根据Notch方向转换 centerOffsetX centerOffsetY
//    *
//    * @param {number} centerOffsetX
//    * @param {number} centerOffsetY
//    * @param {{
//    *     notch: NotchDirection;
//    *     posX: XAxisDirection;
//    *     posY: YAxisDirection;
//    *   }} originalConf
//    * @param {NotchDirection} targetNotch
//    * @return {*}  {{
//    *   centerOffsetX: number;
//    *   centerOffsetY: number;
//    * }}
//    *
//    * 示例:
//    * rotateCenterOffset(2, 4, {notch: 'right', posX: 'right', posY: 'up'}, 'down'); // 输出：{centerOffsetX: 4, centerOffsetY: -2}
//    */
//   export function rotateCenterOffset(
//     centerOffsetX: number | undefined,
//     centerOffsetY: number | undefined,
//     originalConf: {
//       notch: NotchDirection;
//       posX: XAxisDirection;
//       posY: YAxisDirection;
//     },
//     targetNotch: NotchDirection,
//   ): {
//     centerOffsetX?: number;
//     centerOffsetY?: number;
//   } {
//     const [{ x: newCenterOffsetX, y: newCenterOffsetY }] = rotatePointsByNotch(
//       [{ x: centerOffsetX, y: centerOffsetY }],
//       originalConf,
//       targetNotch,
//     );

//     return {
//       centerOffsetX: newCenterOffsetX,
//       centerOffsetY: newCenterOffsetY,
//     };
//   }

//   /**
//    * 根据Notch方向转换 rowOffset colOffset
//    *
//    * @param {number} row
//    * @param {number} column
//    * @param {number} rowOffset
//    * @param {number} colOffset
//    * @param {{
//    *     notch: NotchDirection;
//    *     posX: XAxisDirection;
//    *     posY: YAxisDirection;
//    *   }} originalConf
//    * @param {NotchDirection} targetNotch
//    * @return {*}  {{
//    *   rowOffset: number;
//    *   colOffset: number;
//    * }}
//    *
//    * 示例:
//    * rotateRowColOffset(2, 2, 0, 1, {notch: 'right', posX: 'right', posY: 'up'}, 'down'); // 输出：{rowOffset: 1, colOffset: 1}
//    */
//   export function rotateRowColOffset(
//     row: number,
//     column: number,
//     rowOffset: number,
//     colOffset: number,
//     originalConf: {
//       notch: NotchDirection;
//       posX: XAxisDirection;
//       posY: YAxisDirection;
//     },
//     targetNotch: NotchDirection,
//   ): {
//     rowOffset: number;
//     colOffset: number;
//   } {
//     if (originalConf.notch === targetNotch) return { rowOffset, colOffset };

//     // 按照 rowOffset, colOffset 坐标系，生成 row column 范围内的点
//     const madeAllPoints: Point[] = [];
//     for (let x = 0; x < row; x++) {
//       for (let y = 0; y < column; y++) {
//         madeAllPoints.push({ x, y, ox: x, oy: y });
//       }
//     }
//     // 将 row column 范围内的点基于 (0,0) 按照 notch => targetNotch 变更方向旋转
//     const rotatedPoints = rotatePointsByNotch(madeAllPoints, originalConf, targetNotch);
//     // 匹配 rowOffset, colOffset 旋转后的点
//     const matchedOffsetPoint = rotatedPoints.find(
//       ({ ox, oy }) => ox === rowOffset && oy === colOffset,
//     )!;
//     // 获取旋转后的最小坐标点
//     let minX = Number.MAX_SAFE_INTEGER;
//     let minY = Number.MAX_SAFE_INTEGER;
//     rotatedPoints.forEach(({ x, y }) => {
//       if (!isNil(x) && x < minX) minX = x;
//       if (!isNil(y) && y < minY) minY = y;
//     });
//     // 平移最小坐标点到 0，0 的向量距离
//     const [{ x: newRowOffset, y: newColOffset }] = translatePoints(
//       [matchedOffsetPoint],
//       { x: minX, y: minY },
//       { x: 0, y: 0 },
//     );
//     return { rowOffset: newRowOffset!, colOffset: newColOffset! };
//   }
