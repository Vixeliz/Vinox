struct DrawUniforms {
    color: vec4<f32>,
    model_transform: mat4x4<f32>,
    camera_transform: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: DrawUniforms;

@group(1) @binding(0)
var t: texture_2d<f32>;

@group(1) @binding(1)
var s: sampler;


struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) color: vec4<f32>,
}


struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) vertex_color: vec4<f32>,
    @location(3) fog: f32
}


@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    let in_clip = uniforms.camera_transform * uniforms.model_transform * vec4<f32>(model.position, 1.0);
    let snap_scale = 15.0;
    var position = vec4(
        in_clip.x  / in_clip.w,
        in_clip.y  / in_clip.w,
        in_clip.z  / in_clip.w,
        in_clip.w
    );
    position = vec4(
        floor(in_clip.x * snap_scale) / snap_scale,
        floor(in_clip.y * snap_scale) / snap_scale,
        in_clip.z,
        in_clip.w
    );

    // let fog_distance = vec2<f32>(25.0, 75.0);
    // let depth_vert = uniforms.camera_transform * vec4(position);
    // let depth = abs(depth_vert.z / depth_vert.w);
    out.clip_position = position;
    out.tex_coord = model.tex_coords;
    // out.fog = 1.0 - clamp((fog_distance.y - depth) / (fog_distance.y - fog_distance.x), 0.0, 1.0);
    out.fog = 0.0;
    out.color = uniforms.color;
    out.vertex_color = model.color;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex = textureSample(t, s, in.tex_coord);
    if tex.a < 0.1 {
        discard;
    }
    let fir_col = in.vertex_color * tex;
    let fog_color = vec3<f32>(1.0, 1.0, 1.0);
    let col = vec4(mix(fir_col.rgb, fog_color, in.fog), tex.a);
    return col;
}
