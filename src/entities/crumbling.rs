use serde::Deserialize;

use super::math;
use super::BoundingBox;
use super::BoundingCylinder;
use super::Camera;
use super::Entity;
use crate::effects;
use crate::renderer;
use math::*;

use crate::fixed;
use crate::utils;
use fixed::*;

#[derive(Copy, Clone, Deserialize, Debug)]
pub struct Crumbling {
    #[serde(default = "default_i16")]
    id: i16,
    #[serde(default = "default_fixed")]
    x: Fixed,
    #[serde(default = "default_fixed")]
    y: Fixed,
    #[serde(default = "default_fixed")]
    z: Fixed,

    #[serde(default = "default_fixed")]
    xsize: Fixed,
    #[serde(default = "default_fixed")]
    ysize: Fixed,
    #[serde(default = "default_fixed")]
    zsize: Fixed,

    #[serde(default = "default_fixed")]
    x_rotation: Fixed,
    #[serde(rename = "rotation", default = "default_fixed")]
    y_rotation: Fixed,
    #[serde(default = "default_fixed")]
    z_rotation: Fixed,

    #[serde(default = "default_fixed_3_8")]
    points: [[Fixed; 3]; 8],
    #[serde(default = "default_fixed_3_8")]
    model_rotated_points: [[Fixed; 3]; 8],

    #[serde(default = "default_fixed_3_3")]
    x_rotation_matrix: [[Fixed; 3]; 3],
    #[serde(default = "default_fixed_3_3")]
    y_rotation_matrix: [[Fixed; 3]; 3],
    #[serde(default = "default_fixed_3_3")]
    z_rotation_matrix: [[Fixed; 3]; 3],

    #[serde(default = "default_u16")]
    color: u16,

    #[serde(default = "default_i16")]
    lifetime: i16,

	#[serde(default = "default_i16")]
	player_standing_on_rect: i16,

	#[serde(default = "positive_i16")]
	shake_direction: i16,
}

impl Crumbling {
    #[allow(dead_code)]
    pub fn default() -> Self {
        Self {
            id: 0,
            x: Fixed::const_new(0),
            y: Fixed::const_new(0),
            z: Fixed::const_new(0),
            xsize: Fixed::const_new(0),
            ysize: Fixed::const_new(0),
            zsize: Fixed::const_new(0),
            x_rotation: Fixed::const_new(0),
            y_rotation: Fixed::const_new(0),
            z_rotation: Fixed::const_new(0),
            points: [[Fixed::const_new(0); 3]; 8],
            model_rotated_points: [[Fixed::const_new(0); 3]; 8],
            x_rotation_matrix: [[Fixed::const_new(0); 3]; 3],
            y_rotation_matrix: [[Fixed::const_new(0); 3]; 3],
            z_rotation_matrix: [[Fixed::const_new(0); 3]; 3],
            color: 0,
            lifetime: 0,
            player_standing_on_rect: 0,
			shake_direction: 1,
        }
    }
}

impl Entity for Crumbling {
    fn set_x_offset(&mut self, x_offset: Fixed) {
        self.x = x_offset;
    }

    fn set_y_offset(&mut self, y_offset: Fixed) {
        self.y = y_offset;
    }

    fn set_z_offset(&mut self, z_offset: Fixed) {
        self.z = z_offset;
    }

    fn set_size(&mut self, size: Fixed) {
        self.xsize = size;
        self.ysize = size;
        self.zsize = size;
    }

    fn recalculate_points(&mut self) {
        let halfx: Fixed = self.xsize / 2;
        let halfy: Fixed = self.ysize / 2;
        let halfz: Fixed = self.zsize / 2;
        self.points = [
            [(halfx), (halfy), (halfz)],
            [(-halfx), (halfy), (halfz)],
            [(-halfx), (-halfy), (halfz)],
            [(halfx), (-halfy), (halfz)],
            [(halfx), (halfy), (-halfz)],
            [(-halfx), (halfy), (-halfz)],
            [(-halfx), (-halfy), (-halfz)],
            [(halfx), (-halfy), (-halfz)],
        ];
    }

    fn set_x_rotation(&mut self, x_rotation: Fixed) {
        self.x_rotation = x_rotation;
        self.x_rotation_matrix = [
            [
                Fixed::const_new(1),
                Fixed::const_new(0),
                Fixed::const_new(0),
            ],
            [
                Fixed::const_new(0),
                self.x_rotation.cos(),
                -self.x_rotation.sin(),
            ],
            [
                Fixed::const_new(0),
                self.x_rotation.sin(),
                self.x_rotation.cos(),
            ],
        ];
    }

    fn set_y_rotation(&mut self, y_rotation: Fixed) {
        self.y_rotation = y_rotation;
        self.y_rotation_matrix = [
            [
                self.y_rotation.cos(),
                Fixed::const_new(0),
                self.y_rotation.sin(),
            ],
            [
                Fixed::const_new(0),
                Fixed::const_new(1),
                Fixed::const_new(0),
            ],
            [
                -self.y_rotation.sin(),
                Fixed::const_new(0),
                self.y_rotation.cos(),
            ],
        ];
    }

    fn set_z_rotation(&mut self, z_rotation: Fixed) {
        self.z_rotation = z_rotation;
        self.z_rotation_matrix = [
            [
                self.z_rotation.cos(),
                -self.z_rotation.sin(),
                Fixed::const_new(0),
            ],
            [
                self.z_rotation.sin(),
                self.z_rotation.cos(),
                Fixed::const_new(0),
            ],
            [
                Fixed::const_new(0),
                Fixed::const_new(0),
                Fixed::const_new(1),
            ],
        ];
    }

    fn reload_rotation_matrices(&mut self) {
        self.set_x_rotation(self.x_rotation);
        self.set_y_rotation(self.y_rotation);
        self.set_z_rotation(self.z_rotation);
    }
    fn refresh_model_matrix(&mut self) {
        for i in 0..self.points.len() {
            let point: &[Fixed; 3] = &self.points[i];

            let mut rotated_point: [Fixed; 3] = matmul(self.x_rotation_matrix, *point);
            rotated_point = matmul(self.y_rotation_matrix, rotated_point);
            rotated_point = matmul(self.z_rotation_matrix, rotated_point);

            self.model_rotated_points[i] = rotated_point;
        }
    }

    fn set_vertex(&mut self, _point: [Fixed; 3], _index: i32) {
        //not implemented
    }

    fn render(&mut self, camera: &Camera, page: u16) {
        if self.lifetime > 0 {
            let shaking_points: [[Fixed; 3]; 8];

            let mut shake: i16 = 60 - self.lifetime;
			if shake < 0 {
				shake = 0;
			}
			shake *= self.shake_direction;
			self.shake_direction *= -1;
			let offset = Fixed::const_new(shake as i32) / 200;

            if shake > 0 && self.player_standing_on_rect == 1 {
                shaking_points = self.model_rotated_points.map(|point| {
                    let mut new_point = point;
                    new_point[0] += offset;
                    new_point
                });
            } else {
                shaking_points = self.model_rotated_points;
            }

            renderer::draw_rect(
                &shaking_points,
                self.x,
                self.y,
                self.z,
                self.y_rotation,
                camera,
                self.color,
                page,
            );
        }
    }

    fn distance_from_camera(&self, camera: &Camera) -> Fixed {
        return (self.x - camera.x).abs() + (self.y - camera.y).abs() + (self.z - camera.z).abs();
    }

    fn bounding_box(&self) -> BoundingBox {
        if self.lifetime == 0 {
            BoundingBox {
                data: [[Fixed::const_new(0); 2]; 4],
                center: [Fixed::const_new(0); 2],
                width: Fixed::const_new(0),
                height: Fixed::const_new(0),
                y_top: Fixed::const_new(-999),
                y_bottom: Fixed::const_new(-999),
                rotation: Fixed::const_new(0),
            }
        } else {
            let points: [[Fixed; 2]; 4] = [
                [
                    self.model_rotated_points[0][0] + self.x,
                    self.model_rotated_points[0][2] + self.z,
                ],
                [
                    self.model_rotated_points[1][0] + self.x,
                    self.model_rotated_points[1][2] + self.z,
                ],
                [
                    self.model_rotated_points[5][0] + self.x,
                    self.model_rotated_points[5][2] + self.z,
                ],
                [
                    self.model_rotated_points[4][0] + self.x,
                    self.model_rotated_points[4][2] + self.z,
                ],
            ];

            BoundingBox {
                data: points,
                center: utils::calculate_center(&points),
                width: (self.model_rotated_points[0][0] + self.x
                    - (self.model_rotated_points[1][0] + self.x))
                    .abs(),
                height: (self.model_rotated_points[1][2] + self.z
                    - (self.model_rotated_points[5][2] + self.z))
                    .abs(),
                y_top: self.model_rotated_points[0][1] + self.y,
                y_bottom: self.model_rotated_points[2][1] + self.y,
                rotation: -self.y_rotation,
            }
        }
    }

    fn bounding_cylinder(&self) -> BoundingCylinder {
        BoundingCylinder {
            x: self.x,
            z: self.z,
            radius: self.xsize / 2,
            y_top: self.model_rotated_points[0][1] + self.y,
            y_bottom: self.model_rotated_points[2][1] + self.y,
        }
    }
    fn get_y(&self) -> Fixed {
        return self.y;
    }
    fn set_color(&mut self, color: u16) {
        self.color = color;
    }
    fn tick(
        &mut self,
        effects: &effects::InputPlayerEffects,
    ) -> Option<effects::OutputPlayerEffects> {
        if self.lifetime > 0 && effects.support_below_id == self.id {
            self.lifetime -= 1;
			self.player_standing_on_rect = 1;
        }
		else {
			self.player_standing_on_rect = 0;
		}
        return None;
    }

    fn get_id(&self) -> i16 {
        return self.id;
    }

    fn set_id(&mut self, id: i16) {
        self.id = id
    }
}
