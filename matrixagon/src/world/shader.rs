pub trait VertexType {}
pub trait IndexType {}

impl IndexType for u32 {}  // DEPRECATED: will be removing the index type in favor of vulkano's index


// using less # of bytes per vertices generally yields higher performance because
// the data can be sent with less clock cycles

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct UIVert {
    pub pos: [f32; 2],  // 2D position
    pub col: [f32; 4],  // RGBA colors
}

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct CubeVert {
    pub pos: [f32; 3],  // 3D position
    pub txtr: u32,  // texture info: 0b00000000_00000000-_00000000_00-0000-00
    //                 texture arr ind (31-15) | other unplanned (14-6) | 4-bit block lighting values (5-2) | texture loc (1-0)
}

// this includes both FloraX and FloraH
#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct FloraVert {
    pub pos: [f32; 3],  // 3D position
    pub txtr: u32,  // texture info: 0b00000000_00000000-_00000000_00-0000-00
    //                 texture arr ind (31-15) | other unplanned (14-2) | texture loc (1-0)
}

vulkano::impl_vertex!(UIVert, pos, col);
vulkano::impl_vertex!(CubeVert, pos, txtr);
vulkano::impl_vertex!(FloraVert, pos, txtr);

impl VertexType for UIVert {}
impl VertexType for CubeVert {}
impl VertexType for FloraVert {}


pub mod ui_simpl_vs { vulkano_shaders::shader!{ty: "vertex", path: "resource/shaders/ui.vert",} }
pub mod ui_simpl_fs { vulkano_shaders::shader!{ty: "fragment", path: "resource/shaders/ui.frag",} }

// texture array should be static relative to program

pub mod cube_vs { vulkano_shaders::shader!{ty: "vertex", path: "resource/shaders/cube.vert",} }
pub mod cube_fs { vulkano_shaders::shader!{ty: "fragment", path: "resource/shaders/cube.frag",} }

pub mod flora_vs { vulkano_shaders::shader!{ty: "vertex", path: "resource/shaders/flora.vert",} }
pub mod flora_fs { vulkano_shaders::shader!{ty: "fragment", path: "resource/shaders/flora.frag",} }

