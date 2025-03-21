use crate::{
    math::{cross_product, vector_len_2d},
    mathlut::PARTIAL_SINE_LUT,
    utils::fixed_array_binary_search,
};

use super::{
    boundingshapes::BoundingShape, BoundingBox, BoundingCylinder, Camera, EntityEnum, Fixed,
};

fn partition(
    entity_render_order: &mut [usize],
    entity_array: &[EntityEnum],
    low: usize,
    high: usize,
    camera: &Camera,
) -> usize {
    let pivot_distance = entity_array[entity_render_order[high]].distance_from_camera(camera);
    let pivot_y = entity_array[entity_render_order[high]].get_y();

    let mut i = low as isize - 1; // Use `isize` to allow `-1` for initialization

    //very jank method to avoid having to do actual per polygon zindexing
    //basically objects with largest y are drawn last, use distance to camera as fallback
    //this needs to be kept in mind when creating levels eventually
    for j in low..high {
        let y = entity_array[entity_render_order[j]].get_y();
        if y < pivot_y {
            i += 1;
            entity_render_order.swap(i as usize, j);
        } else if y > pivot_y {
            //already when we want it
        } else if entity_array[entity_render_order[j]].distance_from_camera(camera)
            >= pivot_distance
        {
            i += 1;
            entity_render_order.swap(i as usize, j);
        }
    }

    entity_render_order.swap((i + 1) as usize, high);
    (i + 1) as usize
}

#[allow(dead_code)]
pub fn quick_sort(
    entity_render_order: &mut [usize],
    entity_array: &[EntityEnum],
    low: usize,
    high: usize,
    camera: &Camera,
) {
    if low < high {
        let pi = partition(entity_render_order, entity_array, low, high, camera);

        if pi > 0 {
            quick_sort(entity_render_order, entity_array, low, pi - 1, camera);
        }
        quick_sort(entity_render_order, entity_array, pi + 1, high, camera);
    }
}

pub fn rect_simple_overlap_check(first: &BoundingBox, second: &BoundingBox) -> bool {
    let mut first_smallest_x = first.data[0][0];
    let mut first_largest_x = first.data[0][0];
    let mut first_smallest_y = first.data[0][1];
    let mut first_largest_y = first.data[0][1];

    let mut second_smallest_x = second.data[0][0];
    let mut second_largest_x = second.data[0][0];
    let mut second_smallest_y = second.data[0][1];
    let mut second_largest_y = second.data[0][1];

    for point in &first.data {
        if point[0] < first_smallest_x {
            first_smallest_x = point[0];
        }
        if point[0] > first_largest_x {
            first_largest_x = point[0];
        }
        if point[1] < first_smallest_y {
            first_smallest_y = point[1];
        }
        if point[1] > first_largest_y {
            first_largest_y = point[1];
        }
    }

    for point in &second.data {
        if point[0] < second_smallest_x {
            second_smallest_x = point[0];
        }
        if point[0] > second_largest_x {
            second_largest_x = point[0];
        }
        if point[1] < second_smallest_y {
            second_smallest_y = point[1];
        }
        if point[1] > second_largest_y {
            second_largest_y = point[1];
        }
    }

    if (first_largest_x < second_smallest_x || first_smallest_x > second_largest_x)
        || (first_largest_y < second_smallest_y || first_smallest_y > second_largest_y)
    {
        return false;
    } else {
        return true;
    }
}

pub fn rect_overlap(first: &BoundingBox, second: &BoundingBox) -> bool {
    for i in 0..4 {
        let cross1: Fixed = cross_product(second.data[0], second.data[1], first.data[i]);
        let cross2: Fixed = cross_product(second.data[1], second.data[2], first.data[i]);
        let cross3: Fixed = cross_product(second.data[2], second.data[3], first.data[i]);
        let cross4: Fixed = cross_product(second.data[3], second.data[0], first.data[i]);
        const Z: Fixed = Fixed::const_new(0);
        if (cross1 >= Z && cross2 >= Z && cross3 >= Z && cross4 >= Z)
            || (cross1 <= Z && cross2 <= Z && cross3 <= Z && cross4 <= Z)
        {
            return true;
        }
    }
    for i in 0..4 {
        let cross1: Fixed = cross_product(first.data[0], first.data[1], second.data[i]);
        let cross2: Fixed = cross_product(first.data[1], first.data[2], second.data[i]);
        let cross3: Fixed = cross_product(first.data[2], first.data[3], second.data[i]);
        let cross4: Fixed = cross_product(first.data[3], first.data[0], second.data[i]);

        if (cross1 >= 0 && cross2 >= 0 && cross3 >= 0 && cross4 >= 0)
            || (cross1 <= 0 && cross2 <= 0 && cross3 <= 0 && cross4 <= 0)
        {
            return true;
        }
    }
    return false;
}

pub fn vertical_room_for_box(
    first: &BoundingBox,
    second: &BoundingBox,
    second_fallback: &BoundingCylinder,
    limit: Fixed,
) -> Fixed {
    if (limit < 0 && first.y_top > second.y_bottom) || (limit > 0 && first.y_bottom < second.y_top)
    {
        return limit;
    }

    // // //fast check to see if we are even close to each other
    // if !rect_simple_overlap_check(first, second) {
    //     return limit;
    // }

    
    //more detailed check
    if rect_overlap(first, second) {
        if limit < 0 {
            return first.y_top;
        } else {
            return first.y_bottom;
        }
    }

    //if the more detailed check doesn't produce an answer, check with the cylinder to account for a
    //special edge case, where the player is standing on a super narrow platform
    if cylinder_and_rotated_rect_collision(second_fallback, first).1 {
        if limit < 0 {
            return first.y_top;
        } else {
            return first.y_bottom;
        }
    }

    return limit;
}

fn vertical_room_for_cylinder(
    first: &BoundingCylinder,  //platform
    second: &BoundingCylinder, //player
    limit: Fixed,
) -> Fixed {
    if (limit < 0 && first.y_top > second.y_bottom) || (limit > 0 && first.y_bottom < second.y_top)
    {
        return limit;
    }

    let dx = first.x - second.x;
    let dz = first.z - second.z;
    let distance_squared = dx * dx + dz * dz;
    let sum_radius = first.radius + second.radius;
    if sum_radius * sum_radius > distance_squared {
        if limit < 0 {
            return first.y_top;
        } else {
            return first.y_bottom;
        }
    }
    return limit;

    // if cylinder_and_rotated_rect_h_overlap(first, second) {
    //     if limit < 0 {
    //         return first.y_top;
    //     } else {
    //         return first.y_bottom;
    //     }
    // } else {
    //     return limit;
    // }
}

//determine if the element in the entiry array is below us and how far
pub fn check_support_below(
    entity_array: &[EntityEnum],
    bottom: &BoundingBox,
    fallback: &BoundingCylinder,
) -> (Fixed, i16) {
    let mut height: Fixed = Fixed::const_new(-999);
    let mut collider_id: i16 = -1;
    //a hack to detect player on a platform even if they aren't right on it
    //TODO: instead of a static nudge, take player yspeed as a param, and check if 
    //      bottom.y_bottom + yspeed <= height, then consider it to collide
    const DISTANCE_NUDGE_FACTOR: Fixed = Fixed::const_new(1);
    for (i, e) in entity_array.iter().enumerate() {
        if let EntityEnum::Empty(_) = e {
            break;
        }
        if i != 0 && i != 1 {
            //let top: BoundingBox = e.bounding_box();
            let top_shape = e.bounding_shape();
            if let Some(top_shape) = top_shape {
                if let BoundingShape::BoundingBox(top) = top_shape {
                    let d: Fixed =
                        vertical_room_for_box(&top, &bottom, &fallback, Fixed::const_new(-999));
                    if d > height {
                        height = d;
                        if (height - bottom.y_bottom).abs() < DISTANCE_NUDGE_FACTOR {
                            collider_id = e.get_id();
                        }
                    }
                } else if let BoundingShape::BoundingCylinder(top) = top_shape {
                    //using player cylinder as fallback, as the box-cylinder check seems borked if it's small as the player is
                    let d: Fixed =
                        vertical_room_for_cylinder(&top, &fallback, Fixed::const_new(-999));
                    if d > height {
                        height = d;
                        if (height - bottom.y_bottom).abs() < DISTANCE_NUDGE_FACTOR  {
                            collider_id = e.get_id();
                        }
                    }
                }
            }
        }
    }
    return (height, collider_id);
}

pub fn check_block_above(
    entity_array: &[EntityEnum],
    top: &BoundingBox,
    fallback: &BoundingCylinder,
) -> Fixed {
    let mut max_height: Fixed = Fixed::const_new(999);
    for (i, e) in entity_array.iter().enumerate() {
        if let EntityEnum::Empty(_) = e {
            break;
        }
        if i != 0 && i != 1 {
            let bottom_shape = e.bounding_shape();

            if let Some(bottom_shape) = bottom_shape {
                if let BoundingShape::BoundingBox(bottom) = bottom_shape {
                    let d: Fixed =
                        vertical_room_for_box(&bottom, &top, &fallback, Fixed::const_new(999));
                    if d < max_height {
                        max_height = d;
                    }
                } else if let BoundingShape::BoundingCylinder(bottom) = bottom_shape {
                    let d: Fixed =
                        vertical_room_for_cylinder(&bottom, &fallback, Fixed::const_new(999));
                    if d < max_height {
                        max_height = d;
                    }
                }
            }
        }
    }
    return max_height;
}

#[allow(dead_code)]
pub fn cylinder_and_rect_collision(cyl1: &BoundingCylinder, box2: &BoundingBox) -> bool {
    //can't overlap, if not sharing y coordinates (z here)
    if cyl1.y_top <= box2.y_bottom || box2.y_top <= cyl1.y_bottom {
        return false;
    }
    // Step 1: Deduce rectangle bounds
    let mut min_x: Fixed = box2.data[0][0];
    let mut max_x: Fixed = box2.data[0][0];
    let mut min_z: Fixed = box2.data[0][1];
    let mut max_z: Fixed = box2.data[0][1];

    for point in box2.data.iter() {
        let x: Fixed = point[0];
        let z: Fixed = point[1];
        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if z < min_z {
            min_z = z;
        }
        if z > max_z {
            max_z = z;
        }
    }

    let clamped_x: Fixed = if cyl1.x < min_x {
        min_x
    } else if cyl1.x > max_x {
        max_x
    } else {
        cyl1.x
    };
    let clamped_y: Fixed = if cyl1.z < min_z {
        min_z
    } else if cyl1.z > max_z {
        max_z
    } else {
        cyl1.z
    };

    let dx: Fixed = clamped_x - cyl1.x;
    let dy: Fixed = clamped_y - cyl1.z;
    let distance_squared: Fixed = dx * dx + dy * dy;

    //true if there is a collision
    distance_squared <= cyl1.radius * cyl1.radius
}

fn line_circle_intersect(a: [Fixed; 2], b: [Fixed; 2], x: Fixed, z: Fixed, radius: Fixed) -> bool {
    let ab = ( b[0] - a[0], b[1] - a[1] );
    let ac = (x - a[0], z - a[1]);
    let proj = (ac.0 * ab.0 + ac.1 * ab.1) / (ab.0 * ab.0 + ab.1*ab.1);
    let closest = if proj < Fixed::const_new(0) {
        a
    } else if proj > Fixed::const_new(1) {
        b
    } else {
        [a[0] + proj*ab.0, a[1] + proj*ab.1]

    };

    let dx = closest[0] - x;
    let dz = closest[1] - z;
    let distance_squared = dx * dx + dz * dz;

    if distance_squared <= radius * radius {
        return true;
    }

    return false;
}

pub fn cylinder_and_rotated_rect_collision(
    cyl1: &BoundingCylinder,
    box2: &BoundingBox,
) -> (Fixed, bool) {

    // Check if the circle overlaps any of the corners of the box2 in xz space
    for point in &box2.data {
        let dx = cyl1.x - point[0];
        let dz = cyl1.z - point[1];
        let distance_squared = dx * dx + dz * dz;
        if distance_squared <= cyl1.radius * cyl1.radius {
            return (box2.rotation, true);
        }
    }

    //check if the circle rim and any of the rectangles lines overlap
    for i in 0..4 {
        let a: [Fixed; 2] = box2.data[i];
        let b: [Fixed; 2] = box2.data[(i + 1) % 4];

        if line_circle_intersect(a, b, cyl1.x, cyl1.z, cyl1.radius) {
            return (box2.rotation, true);
        }
    }

    

    return (Fixed::default(), false);


}

//TODO: do a check for the angle of the cylinder
//don't have
pub fn horizontal_collision_check(
    entity_array: &[EntityEnum],
    cyl1: BoundingCylinder,
) -> (Fixed, bool) {
    for (i, e) in entity_array.iter().enumerate() {
        if i != 0 && i != 1 {
            let shape2: Option<BoundingShape> = e.bounding_shape();

            if let Some(BoundingShape::BoundingBox(box2)) = shape2 {
                if box2.rotation == Fixed::const_new(0) {
                    if cylinder_and_rect_collision(&cyl1, &box2) {
                        return (Fixed::const_new(0), true);
                    }
                } else {
                    if cyl1.y_top <= box2.y_bottom || box2.y_top <= cyl1.y_bottom {
                        continue;
                    }
                    let (wallangle, ok) = cylinder_and_rotated_rect_collision(&cyl1, &box2);
                    if ok {
                        return (wallangle, true);
                    }
                }
            } else if let Some(BoundingShape::BoundingCylinder(cyl2)) = shape2 {
                if cyl1.y_top < cyl2.y_bottom || cyl2.y_top < cyl1.y_bottom {
                    continue;
                }

                let dx = cyl1.x - cyl2.x;
                let dz = cyl1.z - cyl2.z;
                let distance_squared = dx * dx + dz * dz;
                let sum_radius = cyl1.radius + cyl2.radius;
                if distance_squared < sum_radius * sum_radius {
                    //TODO: estimate the angle for the vector here
                    return (vector_angle(dx, dz), true);
                }
            }
        }
    }
    return (Fixed::default(), false);
}

fn vector_angle(dx: Fixed, dz: Fixed) -> Fixed {
    let hyp = vector_len_2d([dx, dz]);
    if hyp == Fixed::const_new(0) {
        return Fixed::const_new(0);
    }
    let mut target = dx / hyp;
    if target < 0 {
        target = target + Fixed::from_raw(64);
    }
    if target == Fixed::const_new(0) {
        return Fixed::const_new(0);
    }
    else if target > Fixed::const_new(255) {
        return Fixed::const_new(64);
    }
    let angle = fixed_array_binary_search(target, &PARTIAL_SINE_LUT, 90);
    //normalize to fixed between 0 and 1
    let angle: Fixed = Fixed::new(angle) / Fixed::const_new(360);

    angle
}
