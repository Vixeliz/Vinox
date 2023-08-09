use base64::Engine;
use glam::*;
use gltf::scene::Transform;
use image::{EncodableLayout, ImageBuffer, RgbaImage};
use std::{fs::File, io::BufReader, path::Path};

fn transform_to_matrix(transform: Transform) -> Mat4 {
    let tr = transform.matrix();
    Mat4::from_cols_array_2d(&tr)
}

// This is needed to handle ascii gltf files
struct DataUri<'a> {
    mime_type: &'a str,
    base64: bool,
    data: &'a str,
}

fn split_once(input: &str, delimiter: char) -> Option<(&str, &str)> {
    let mut iter = input.splitn(2, delimiter);
    Some((iter.next()?, iter.next()?))
}

impl<'a> DataUri<'a> {
    fn parse(uri: &'a str) -> Result<DataUri<'a>, ()> {
        let uri = uri.strip_prefix("data:").ok_or(())?;
        let (mime_type, data) = split_once(uri, ',').ok_or(())?;

        let (mime_type, base64) = match mime_type.strip_suffix(";base64") {
            Some(mime_type) => (mime_type, true),
            None => (mime_type, false),
        };

        Ok(DataUri {
            mime_type,
            base64,
            data,
        })
    }

    fn decode(&self) -> Result<Vec<u8>, String> {
        if self.base64 {
            if let Ok(vec) = base64::engine::general_purpose::STANDARD_NO_PAD.decode(self.data) {
                Ok(vec)
            } else {
                Err("Failed to decode base64".to_string())
            }
        } else {
            Ok(self.data.as_bytes().to_owned())
        }
    }
}

// Implementation tooken from bevy
/// An aabb stands for axis aligned bounding box. This is basically a cube that can't rotate.
#[derive(Debug, Copy, Clone)]
pub struct Aabb {
    /// The center of this `Aabb`
    pub center: Vec3,
    /// The half_extents or half the size of this `Aabb` for each axis
    pub half_extents: Vec3,
}

impl Default for Aabb {
    fn default() -> Self {
        Self {
            center: Vec3::ZERO.into(),
            half_extents: Vec3::ZERO.into(),
        }
    }
}

impl Aabb {
    /// Create an `Aabb` from a minimum point and a maximum point
    #[inline]
    pub fn from_min_max(minimum: Vec3, maximum: Vec3) -> Self {
        let minimum = minimum;
        let maximum = maximum;
        let center = 0.5 * (maximum + minimum);
        let half_extents = 0.5 * (maximum - minimum);
        Self {
            center: center.into(),
            half_extents: half_extents.into(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vertex {
    /// The position of this vertex
    pub pos: [f32; 3],
    /// The texture uv of this vertex
    pub tex_coord: [f32; 2],
    /// The color of this vertex
    pub color: [f32; 4],
    /// Normal of this vertex (the direction it faces)
    pub normals: [f32; 3],
}

impl Vertex {
    /// Create a new vertex from a position, uv, normals, and color
    pub fn new(position: Vec3, uv: Vec2, color: Option<[f32; 4]>, normals: Vec3) -> Vertex {
        // let position: Vector3<f32> = position.into();
        // let normals: Vector3<f32> = normals.into();
        // let uv: Vector2<f32> = uv.into();
        // let color: Option<graphics::Color> = color.into();
        // let color = color
        //     .unwrap_or(graphics::Color::new(1.0, 1.0, 1.0, 1.0))
        //     .into();
        let color = color.unwrap_or([1.0, 1.0, 1.0, 1.0]);
        Vertex {
            pos: position.into(),
            tex_coord: uv.into(),
            color,
            normals: normals.into(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Mesh {
    pub texture: Option<ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub aabb: Option<Aabb>,
    // pub transform: Transform,
}

/// These models are for entities
#[derive(Debug, Default)]
pub struct Model {
    // pub id: u64,
    /// The meshes that make up the model
    pub meshes: Vec<Mesh>,
    /// The bounding box of the model
    pub aabb: Option<Aabb>,
    pub animations: Vec<Animation>,
}

#[derive(Debug, Default)]
pub struct Animation {}

// impl std::hash::Hash for Model {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.id.hash(state)
//     }
// }

impl Model {
    fn read_node(
        meshes: &mut Vec<Mesh>,
        node: &gltf::Node,
        parent_transform: Mat4,
        buffer_data: &mut Vec<Vec<u8>>,
        // gfx: &mut GraphicsContext,
    ) -> Result<(), String> {
        let transform = parent_transform * transform_to_matrix(node.transform());
        for child in node.children() {
            Model::read_node(meshes, &child, transform, buffer_data)?;
        }
        if let Some(mesh) = node.mesh() {
            for primitive in mesh.primitives() {
                let reader =
                    primitive.reader(|buffer| Some(buffer_data[buffer.index()].as_slice()));
                let texture_source = &primitive
                    .material()
                    .pbr_metallic_roughness()
                    .base_color_texture()
                    .map(|tex| tex.texture().source().source());
                let image = if let Some(source) = texture_source {
                    match source {
                        gltf::image::Source::View { view, mime_type } => {
                            let parent_buffer_data = &buffer_data[view.buffer().index()];
                            let data =
                                &parent_buffer_data[view.offset()..view.offset() + view.length()];
                            let mime_type = mime_type.replace('/', ".");
                            let dynamic_img = image::load_from_memory_with_format(
                                data,
                                image::ImageFormat::from_path(mime_type)
                                    .unwrap_or(image::ImageFormat::Png),
                            )
                            .unwrap_or_default()
                            .into_rgba8();
                            dynamic_img
                            // Image::from_pixels(
                            //     gfx,
                            //     dynamic_img.as_bytes(),
                            //     wgpu::TextureFormat::Rgba8UnormSrgb,
                            //     dynamic_img.width(),
                            //     dynamic_img.height(),
                            // )
                        }
                        gltf::image::Source::Uri { uri, mime_type } => {
                            let uri = percent_encoding::percent_decode_str(uri)
                                .decode_utf8()
                                .unwrap();
                            let uri = uri.as_ref();
                            let bytes = match DataUri::parse(uri) {
                                Ok(data_uri) => data_uri.decode()?,
                                Err(()) => return Err("Failed to decode".to_string()),
                            };
                            let dynamic_img = image::load_from_memory_with_format(
                                bytes.as_bytes(),
                                image::ImageFormat::from_path(mime_type.unwrap_or_default())
                                    .unwrap_or(image::ImageFormat::Png),
                            )
                            .unwrap_or_default()
                            .into_rgba8();
                            dynamic_img
                            // Image::from_pixels(
                            //     gfx,
                            //     dynamic_img.as_bytes(),
                            //     wgpu::TextureFormat::Rgba8UnormSrgb,
                            //     dynamic_img.width(),
                            //     dynamic_img.height(),
                            // )
                        }
                    }
                } else {
                    ImageBuffer::default()
                    // Image::from_color(gfx, 1, 1, Some(graphics::Color::WHITE))
                };
                let mut vertices = Vec::default();
                if let Some(vertices_read) = reader.read_positions() {
                    vertices = vertices_read
                        .map(|x| {
                            let pos = Vec4::new(x[0], x[1], x[2], 1.);
                            let res = transform * pos;
                            let pos = Vec3::new(res.x / res.w, res.y / res.w, res.z / res.w);
                            Vertex::new(
                                pos,
                                glam::Vec2::ZERO,
                                Some([1.0, 1.0, 1.0, 0.0]),
                                Vec3::new(0.0, 0.0, 0.0),
                            )
                        })
                        .collect();
                }

                if let Some(tex_coords) = reader.read_tex_coords(0).map(|v| v.into_f32()) {
                    let mut idx = 0;
                    tex_coords.for_each(|tex_coord| {
                        vertices[idx].tex_coord = tex_coord;

                        idx += 1;
                    });
                }

                if let Some(normals) = reader.read_normals() {
                    let mut idx = 0;
                    normals.for_each(|normals| {
                        vertices[idx].normals = normals;

                        idx += 1;
                    });
                }

                let mut indices = Vec::new();
                if let Some(indices_raw) = reader.read_indices() {
                    indices.append(&mut indices_raw.into_u32().collect::<Vec<u32>>());
                }

                // let mesh = Mesh3dBuilder::new()
                //     .from_data(vertices, indices, Some(image))
                //     .build(gfx);
                let mesh = Mesh {
                    vertices,
                    indices,
                    texture: Some(image),
                    aabb: None,
                };

                meshes.push(mesh);
            }
        }

        Ok(())
    }

    /// Load gltf from path
    pub fn from_gltf(
        // gfx: &mut impl HasMut<GraphicsContext>,
        path: impl AsRef<Path>,
    ) -> Result<Self, String> {
        // let gfx = gfx.retrieve_mut();
        // let file = gfx.fs.open(path)?;
        let file = BufReader::new(File::open(&path).unwrap());
        if let Ok(gltf) = gltf::Gltf::from_reader(file) {
            return Model::from_raw_gltf(gltf);
        }
        Err("Failed to load gltf file".to_string())
    }

    /// Load gltf from bytes
    pub fn from_gltf_bytes(
        // gfx: &mut impl HasMut<GraphicsContext>,
        bytes: &[u8],
    ) -> Result<Self, String> {
        if let Ok(gltf) = gltf::Gltf::from_slice(bytes) {
            return Model::from_raw_gltf(gltf);
        }
        Err("Invalid gltf bytes".to_string())
    }

    /// Load gltf file from a GLTF. Keep in mind right now the whole gltf will be loaded as one model. So multiple models won't be made in more complex scenes. This either has to be implemneted yourself or possibly will come later.
    pub fn from_raw_gltf(
        // gfx: &mut impl HasMut<GraphicsContext>,
        gltf: gltf::Gltf,
    ) -> Result<Self, String> {
        // let gfx = gfx.retrieve_mut();
        const VALID_MIME_TYPES: &[&str] = &["application/octet-stream", "application/gltf-buffer"];
        let mut meshes = Vec::default();
        let mut buffer_data = Vec::new();
        for buffer in gltf.buffers() {
            match buffer.source() {
                gltf::buffer::Source::Uri(uri) => {
                    let uri = percent_encoding::percent_decode_str(uri)
                        .decode_utf8()
                        .unwrap();
                    let uri = uri.as_ref();
                    let buffer_bytes = match DataUri::parse(uri) {
                        Ok(data_uri) if VALID_MIME_TYPES.contains(&data_uri.mime_type) => {
                            data_uri.decode()?
                        }
                        Ok(_) => return Err("Buffer Format Unsupported".to_string()),
                        Err(()) => return Err("Failed to decode".to_string()),
                    };
                    buffer_data.push(buffer_bytes);
                }
                gltf::buffer::Source::Bin => {
                    if let Some(blob) = gltf.blob.as_deref() {
                        buffer_data.push(blob.into());
                    } else {
                        return Err("MissingBlob".to_string());
                    }
                }
            }
        }
        for scene in gltf.scenes() {
            for node in scene.nodes() {
                Model::read_node(&mut meshes, &node, Mat4::IDENTITY, &mut buffer_data)?;
            }
        }
        let mut model = Model {
            meshes,
            aabb: None,
            animations: vec![],
        };
        model.calculate_aabb();

        Ok(model)
    }
    /// Generate an aabb for this Model
    pub fn calculate_aabb(&mut self) {
        let mut minimum = Vec3::MAX;
        let mut maximum = Vec3::MIN;
        for mesh in self.meshes.iter() {
            for p in mesh.vertices.iter() {
                minimum = minimum.min(Vec3::from_array(p.pos));
                maximum = maximum.max(Vec3::from_array(p.pos));
            }
        }
        if minimum.x != std::f32::MAX
            && minimum.y != std::f32::MAX
            && minimum.z != std::f32::MAX
            && maximum.x != std::f32::MIN
            && maximum.y != std::f32::MIN
            && maximum.z != std::f32::MIN
        {
            self.aabb = Some(Aabb::from_min_max(minimum, maximum))
        } else {
            self.aabb = None
        }
    }
}
