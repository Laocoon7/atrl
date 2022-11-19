use crate::prelude::*;

pub struct VisibilityData<'a, T: VisibilityMap> {
    pub range: i32,
    pub octant: usize,
    pub origin: IVec2,
    pub map: &'a mut T,
    pub top: &'a mut Slope,
    pub bottom: &'a mut Slope,
    pub vision_component: &'a Vision,
}

pub fn compute_octant<T: VisibilityMap>(x: i32, data: &mut VisibilityData<T>) {
    for x in x..=data.range {
        let y_coords = compute_y_coordinate(x, data);
        let top_y = y_coords.x;
        let bottom_y = y_coords.y;

        if !compute_visiblity(top_y, bottom_y, x, data) {
            break;
        }
    }
}

fn compute_y_coordinate<T: VisibilityMap>(x: i32, data: &VisibilityData<T>) -> IVec2 {
    let mut top_y;
    if data.top.x == 1 {
        top_y = x;
    } else {
        top_y = ((x * 2 - 1) * data.top.y + data.top.x) / (data.top.x * 2);

        if blocks_light(x, top_y, data) {
            if data.top.greater_or_equal(top_y * 2 + 1, x * 2) && !blocks_light(x, top_y + 1, data)
            {
                top_y += 1;
            }
        } else {
            let mut ax = x * 2;
            if blocks_light(x + 1, top_y + 1, data) {
                ax += 1;
            }
            if data.top.greater(top_y * 2 + 1, ax) {
                top_y += 1;
            }
        }
    }

    let mut bottom_y;
    if data.bottom.y == 0 {
        bottom_y = 0;
    } else {
        bottom_y = ((x * 2 - 1) * data.bottom.y + data.bottom.x) / (data.bottom.x * 2);

        if data.bottom.greater_or_equal(bottom_y * 2 + 1, x * 2)
            && blocks_light(x, bottom_y, data)
            && !blocks_light(x, bottom_y + 1, data)
        {
            bottom_y += 1;
        }
    }
    IVec2::new(top_y, bottom_y)
}

fn compute_visiblity<T: VisibilityMap>(
    top_y: i32,
    bottom_y: i32,
    x: i32,
    data: &mut VisibilityData<T>,
) -> bool {
    let mut was_opaque = -1;

    for y in (bottom_y..=top_y).rev() {
        if data.range < 0 || data.map.distance(IVec2::ZERO, IVec2::new(x, y)) <= data.range as f32 {
            let is_opaque = blocks_light(x, y, data);

            // Less symmetrical
            // let is_visible = is_opaque ||
            // (
            //     (y != top_y || top.greater(y * 4 - 1, x * 4 + 1)) &&
            //     (y != bottom_y || bottom.less(y * 4 + 1, x * 4 - 1))
            // );

            // Better symmetry
            let is_visible = is_opaque || // Remove is_opaque check for full symmetry but more artifacts in hallways
          (
              (y != top_y || data.top.greater_or_equal(y, x)) &&
              (y != bottom_y || data.bottom.less_or_equal(y, x))
          );

            if is_visible {
                set_visible(x, y, data);
            }

            if x != data.range {
                if is_opaque {
                    if was_opaque == 0 {
                        let mut nx = x * 2;
                        let ny = y * 2 + 1;
                        if blocks_light(x, y + 1, data) {
                            nx -= 1;
                        }
                        if data.top.greater(ny, nx) {
                            if y == bottom_y {
                                *data.bottom = Slope { y: ny, x: nx };
                                break;
                            } else {
                                compute_octant(x + 1, data);
                            }
                        } else if y == bottom_y {
                            return false;
                        }
                    }
                    was_opaque = 1;
                } else {
                    if was_opaque > 0 {
                        let mut nx = x * 2;
                        let ny = y * 2 + 1;
                        if blocks_light(x + 1, y + 1, data) {
                            nx += 1;
                        }
                        if data.bottom.greater_or_equal(ny, nx) {
                            return false;
                        }
                        *data.top = Slope { y: ny, x: nx };
                    }
                    was_opaque = 0;
                }
            }
        }
    }

    was_opaque == 0
}

fn blocks_light<T: VisibilityMap>(x: i32, y: i32, data: &VisibilityData<T>) -> bool {
    let (mut nx, mut ny) = data.origin.into();
    match data.octant {
        0 => {
            nx += x;
            ny -= y;
        }
        1 => {
            nx += y;
            ny -= x;
        }
        2 => {
            nx -= y;
            ny -= x;
        }
        3 => {
            nx -= x;
            ny -= y;
        }
        4 => {
            nx -= x;
            ny += y;
        }
        5 => {
            nx -= y;
            ny += x;
        }
        6 => {
            nx += y;
            ny += x;
        }
        7 => {
            nx += x;
            ny += y;
        }
        _ => {}
    }

    let p = IVec2::new(nx, ny);
    if !data.map.is_in_bounds(p) {
        return true;
    }

    data.map.is_opaque(IVec2::new(nx, ny), data.vision_component)
}

fn set_visible<T: VisibilityMap>(x: i32, y: i32, data: &mut VisibilityData<T>) {
    let (mut nx, mut ny) = data.origin.into();
    match data.octant {
        0 => {
            nx += x;
            ny -= y;
        }
        1 => {
            nx += y;
            ny -= x;
        }
        2 => {
            nx -= y;
            ny -= x;
        }
        3 => {
            nx -= x;
            ny -= y;
        }
        4 => {
            nx -= x;
            ny += y;
        }
        5 => {
            nx -= y;
            ny += x;
        }
        6 => {
            nx += y;
            ny += x;
        }
        7 => {
            nx += x;
            ny += y;
        }
        _ => {}
    }

    let p = IVec2::new(nx, ny);
    if data.map.is_in_bounds(p) {
        data.map.set_visible(p);
    }
}
