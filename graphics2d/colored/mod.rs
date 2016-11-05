#[cfg(not(target_os = "emscripten"))]
pub const VERTEX_GLSL_120: &'static [u8] = include_bytes!("120.glslv");
#[cfg(not(target_os = "emscripten"))]
pub const VERTEX_GLSL_150_CORE: &'static [u8] = include_bytes!("150_core.glslv");

#[cfg(not(target_os = "emscripten"))]
pub const FRAGMENT_GLSL_120: &'static [u8] = include_bytes!("120.glslf");
#[cfg(not(target_os = "emscripten"))]
pub const FRAGMENT_GLSL_150_CORE: &'static [u8] = include_bytes!("150_core.glslf");

#[cfg(target_os = "emscripten")]
pub const VERTEX_GLSL_120: &'static [u8] = include_bytes!("120_webgl.glslv");
#[cfg(target_os = "emscripten")]
pub const VERTEX_GLSL_150_CORE: &'static [u8] = include_bytes!("150_core_webgl.glslv");

#[cfg(target_os = "emscripten")]
pub const FRAGMENT_GLSL_120: &'static [u8] = include_bytes!("120_webgl.glslf");
#[cfg(target_os = "emscripten")]
pub const FRAGMENT_GLSL_150_CORE: &'static [u8] = include_bytes!("150_core_webgl.glslf");
