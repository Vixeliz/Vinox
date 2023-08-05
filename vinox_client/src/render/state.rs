use glam::*;
use image::ImageBuffer;

use super::model::Model;

pub trait ConvertModel {
    fn to_mesh(model: Model) -> Self;
}

#[derive(Debug)]
pub struct AssetRegistry<M: ConvertModel> {
    pub models: Vec<M>,
    pub block_atlas: Option<ImageBuffer<image::Rgba<u8>, Vec<u8>>>, // Animated voxels will be done by passing in a framecount and time value to the shader to offset the uvs. This means all animations are the same speed could pass a speed attribute as well
    pub entity_atlas: Option<ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
}

impl<M: ConvertModel> Default for AssetRegistry<M> {
    fn default() -> Self {
        Self {
            models: Vec::with_capacity(100),
            block_atlas: None,
            entity_atlas: None,
        }
    }
}

impl<M: ConvertModel> AssetRegistry<M> {}

#[derive(Default, Debug)]
pub struct Draw {
    pub model_id: u64,
}

#[derive(Debug)]
pub struct RenderState<M: ConvertModel> {
    pub camera: Camera,
    pub draws: Vec<Draw>,
    pub asset_registry: AssetRegistry<M>,
}

impl<M: ConvertModel> Default for RenderState<M> {
    fn default() -> Self {
        Self {
            camera: Camera::default(),
            draws: Vec::with_capacity(100),
            asset_registry: AssetRegistry::default(),
        }
    }
}

impl<M: ConvertModel> RenderState<M> {
    pub fn clear(&mut self) {}
}

/// Camera3d bundle that holds both the `Projection` and `Camera`
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    /// Position of camera
    pub position: Vec3,
    /// Rotation of camera
    pub rotation: Quat,
    /// The aspect ratio of the projection
    pub aspect: f32,
    /// The field of view for the projection
    pub fovy: f32,
    /// The near clipping plane for the projection
    pub znear: f32,
    /// The far clipping plane for the projection
    pub zfar: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            aspect: 0.0,
            fovy: 70.0,
            znear: 0.1,
            zfar: 250.0,
        }
    }
}

impl Camera {
    /// Calculate the matrix for your camera
    pub fn to_matrix(self) -> Mat4 {
        let (yaw, pitch, _) = self.rotation.to_euler(EulerRot::XYZ);
        let (sin_pitch, cos_pitch) = pitch.sin_cos();
        let (sin_yaw, cos_yaw) = yaw.sin_cos();

        let transform_mat = glam::Mat4::look_to_rh(
            self.position.into(),
            Vec3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            Vec3::Y,
        );
        let projection_mat = Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar);
        (projection_mat * transform_mat).into()
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }
}
