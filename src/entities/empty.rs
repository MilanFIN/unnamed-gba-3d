use serde::Deserialize;

use super::{entity, BoundingBox, BoundingCylinder};
use entity::*;

use super::Camera;

use crate::{effects, fixed};
use fixed::*;

#[derive(Copy, Clone, Deserialize, Debug)]
pub struct Empty {}
impl Entity for Empty {
    fn render(&mut self, _camera: &Camera, _page: u16) {}
    fn set_x_offset(&mut self, _x_offset: Fixed) {}
    fn set_y_offset(&mut self, _y_offset: Fixed) {}
    fn set_z_offset(&mut self, _z_offset: Fixed) {}
    fn set_size(&mut self, _size: Fixed) {}
    fn recalculate_points(&mut self) {}
    fn set_x_rotation(&mut self, _x_rotation: Fixed) {}
    fn set_y_rotation(&mut self, _y_rotation: Fixed) {}
    fn set_z_rotation(&mut self, _z_rotation: Fixed) {}
    fn reload_rotation_matrices(&mut self) {}
    fn refresh_model_matrix(&mut self) {}
    fn set_vertex(&mut self, _point: [Fixed; 3], _index: i32) {}
    fn distance_from_camera(&self, _camera: &Camera) -> Fixed {
        return Fixed::const_new(999);
    }
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox::default()
    }
    fn bounding_cylinder(&self) -> BoundingCylinder {
        BoundingCylinder::default()
    }
    fn get_y(&self) -> Fixed {
        return Fixed::const_new(-999);
    }

    fn set_color(&mut self, _color: u16) {}

    fn tick(&mut self, _effects: &effects::InputPlayerEffects)-> Option<effects::OutputPlayerEffects> {None}

    fn get_id(&self) -> i16 {-1}

    fn set_id(&mut self, _id: i16) {}
}

impl Empty {
    pub fn default() -> Self {
        Self {}
    }
}
