#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}
