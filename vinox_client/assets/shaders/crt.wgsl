struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
}

@group(1) @binding(0)
var t: texture_2d<f32>;

@group(1) @binding(1)
var s: sampler;

const RES = (vec2<f32>(120.0,160.0)); // Controls the crt 'resolution'

const HARD_SCAN  = -12.0; // How hard the scan lines are closer to 0.0 equals softer

const HARD_PIX = -10.0; // How hard pixels are similiar to above

const WRP = vec2<f32>(0.03125, 0.04166666666); // The amount of warping
const MASK_DARK = 1.0; // The mask darkness level
const MASK_LIGHT = 1.5; // Ditto but lightness
const OFFSET = 0.0005;

fn to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        return c / 12.92;
    } else {
        return pow((c+0.055)/1.055, 2.4);
    }
}

fn to_linear_vec(c: vec3<f32>) -> vec3<f32> {
    return vec3(to_linear(c.r),to_linear(c.g),to_linear(c.b));
}

fn to_srgb(c: f32) -> f32 {
    if c < 0.0031308 {
        return c * 12.92;
    } else {
        return 1.055 * pow(c, 0.41666) - 0.055;
    }
}

fn to_srgb_vec(c: vec3<f32>) -> vec3<f32> {
    return vec3(to_srgb(c.r),to_srgb(c.g),to_srgb(c.b));
}

fn fetch(tex: vec3<f32>, pos_init: vec2<f32>,off: vec2<f32>) -> vec3<f32> {
  let pos=floor(pos_init*RES+off)/RES;
  if(max(abs(pos.x - 0.5),abs(pos.y - 0.5))>0.5) {
        return vec3(0.0,0.0,0.0);
    } else {  
        return to_linear_vec(tex);
    }
}

// Distance in emulated pixels to nearest texel.
fn dist(pos_init: vec2<f32>) -> vec2<f32> {
    let pos=pos_init*RES;
    return -((pos-floor(pos))-vec2(0.5));
}
    
// 1D Gaussian.
fn gaus(pos: f32,scale: f32) -> f32 {
    return exp2(scale*pos*pos);
}

// 3-tap Gaussian filter along horz line.
fn horz_3(pos: vec2<f32>,off: f32, tex: vec3<f32>) -> vec3<f32>{
  let b=fetch(tex, pos,vec2(-1.0,off));
  let c=fetch(tex, pos,vec2( 0.0,off));
  let d=fetch(tex, pos,vec2( 1.0,off));
  let dst=dist(pos).x;
  // Convert distance to weight
  let scale=HARD_PIX;
  let wb=gaus(dst - 1.0,scale);
  let wc=gaus(dst + 0.0,scale);
  let wd=gaus(dst + 1.0,scale);
  // Return filtered sample.
  return (b*wb+c*wc+d*wd)/(wb+wc+wd);
}

// 5-tap Gaussian filter along horz line.
fn horz_5(pos: vec2<f32>, off: f32, tex: vec3<f32>) -> vec3<f32>{
  let a=fetch(tex, pos,vec2(-2.0,off));
  let b=fetch(tex, pos,vec2(-1.0,off));
  let c=fetch(tex, pos,vec2( 0.0,off));
  let d=fetch(tex, pos,vec2( 1.0,off));
  let e=fetch(tex, pos,vec2( 2.0,off));
  let dst=dist(pos).x;
  // Convert distance to weight.
  let scale=HARD_PIX;
  let wa=gaus(dst - 2.0,scale);
  let wb=gaus(dst - 1.0,scale);
  let wc=gaus(dst + 0.0,scale);
  let wd=gaus(dst + 1.0,scale);
  let we=gaus(dst + 2.0,scale);
  // Return filtered sample.
  return (a*wa+b*wb+c*wc+d*wd+e*we)/(wa+wb+wc+wd+we);}

// Return scanline weight.
fn scan(pos: vec2<f32>, off: f32) -> f32{
  let dst=dist(pos).y;
  return gaus(dst+off,HARD_SCAN);
}

// Allow nearest three lines to effect pixel.
fn tri(pos: vec2<f32>, tex: vec3<f32>) -> vec3<f32>{
  let a=horz_3(pos,-1.0, tex);
  let b=horz_5(pos, 0.0, tex);
  let c=horz_3(pos, 1.0, tex);
  let wa=scan(pos,-1.0);
  let wb=scan(pos, 0.0);
  let wc=scan(pos, 1.0);
  return a*wa+b*wb+c*wc;
}

// Distortion of scanlines, and end of screen alpha.
fn warp(pos_init: vec2<f32>) -> vec2<f32>{
  var pos = pos_init * 2.0 - 1.0;    
  pos *= vec2(1.0 + (pos.y * pos.y) * WRP.x,1.0 + (pos.x * pos.x) * WRP.y);
  return pos * 0.5 + 0.5;
}

// Shadow mask.
fn mask(pos_init: vec2<f32>) -> vec3<f32>{
  var pos = pos_init;
  pos.x+=pos.y*3.0;
  var mask=vec3(MASK_DARK,MASK_DARK,MASK_DARK);
  pos.x=fract(pos.x/6.0);
  mask.b=MASK_LIGHT;
  return mask;
}    


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var frag_color = vec4<f32>(
        textureSample(t, s, in.uv + vec2<f32>(OFFSET, -OFFSET)).r,
        textureSample(t, s, in.uv + vec2<f32>(-OFFSET, 0.0)).g,
        textureSample(t, s, in.uv + vec2<f32>(0.0, OFFSET)).b,
        1.0
    );
    let pos=warp(in.uv.xy);
    frag_color = vec4<f32>(tri(pos, frag_color.rgb)*mask(in.uv.xy), 1.0);
    frag_color = vec4<f32>(to_srgb_vec(frag_color.rgb), 1.0);

    // Sample each color channel with an arbitrary shift
    return frag_color;
}
