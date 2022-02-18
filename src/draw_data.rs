use std::borrow::Borrow;
use serde::{Serialize, Deserialize};

/// All draw data to render a Dear ImGui frame.
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct DrawData {
    /// For convenience, sum of all draw list index buffer sizes.
    pub total_idx_count: i32,
    /// For convenience, sum of all draw list vertex buffer sizes.
    pub total_vtx_count: i32,
    // Array of DrawList.
    pub cmd_lists: Vec<DrawList>,
    /// Upper-left position of the viewport to render.
    ///
    /// (= upper-left corner of the orthogonal projection matrix to use)
    pub display_pos: [f32; 2],
    /// Size of the viewport to render.
    ///
    /// (= display_pos + display_size == lower-right corner of the orthogonal matrix to use)
    pub display_size: [f32; 2],
    /// Amount of pixels for each unit of display_size.
    ///
    /// Based on io.display_frame_buffer_scale. Typically [1.0, 1.0] on normal displays, and
    /// [2.0, 2.0] on Retina displays, but fractional values are also possible.
    pub framebuffer_scale: [f32; 2],
}

#[cfg(feature = "imgui")]
impl From<&imgui::DrawData> for DrawData {
    fn from(d: &imgui::DrawData) -> Self {
        Self {
            total_idx_count: d.total_idx_count,
            total_vtx_count: d.total_vtx_count,
            cmd_lists: d.draw_lists().map(|n| n.into()).collect(),
            display_pos: d.display_pos,
            display_size: d.display_size,
            framebuffer_scale: d.framebuffer_scale,
        }
    }
}

impl DrawData {
    pub fn draw_lists(&self) -> impl Iterator<Item=&DrawList> {
        self.cmd_lists.iter()
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug)]
pub struct DrawList {
    pub commands: Vec<DrawCmd>,
    pub idx_buffer: Vec<DrawIdx>,
    pub vtx_buffer: Vec<DrawVert>,
}

#[cfg(feature = "imgui")]
impl From<&imgui::DrawList> for DrawList {
    fn from(d: &imgui::DrawList) -> Self {
        Self {
            commands: d.commands().map(|n| n.into()).collect(),
            idx_buffer: d.idx_buffer().to_vec(),
            vtx_buffer: d.vtx_buffer().iter().map(|n| n.into()).collect(),
        }
    }
}

impl DrawList {
    #[inline]
    pub fn idx_buffer(&self) -> &[DrawIdx] {
        &self.idx_buffer
    }

    #[inline]
    pub fn vtx_buffer(&self) -> &[DrawVert] {
        &self.vtx_buffer
    }

    /// # Safety
    /// This is equivalent to `transmute(self.vtx_buffer())` with a little more
    /// checking, and thus inherits the safety considerations of `transmute`ing
    /// slices.
    pub unsafe fn transmute_vtx_buffer<VTy: Copy>(&self) -> &[VTy] {
        // these checks are constant and thus are removed from release builds
        assert_eq!(
            core::mem::size_of::<VTy>(),
            core::mem::size_of::<DrawVert>(),
        );
        assert!(core::mem::align_of::<VTy>() <= core::mem::align_of::<DrawVert>());
        core::slice::from_raw_parts(self.vtx_buffer.as_ptr() as _, self.vtx_buffer.len())
    }

    #[inline]
    pub fn commands(&self) -> impl Iterator<Item=DrawCmd> {
        self.commands.clone().into_iter()
    }
}

/// A draw command
#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq)]
pub enum DrawCmd {
    Elements {
        /// The number of indices used for this draw command
        count: usize,
        cmd_params: DrawCmdParams,
    },
    ResetRenderState,
    // RawCallback {
    //     callback: unsafe extern "C" fn(*const sys::ImDrawList, cmd: *const sys::ImDrawCmd),
    //     raw_cmd: *const sys::ImDrawCmd,
    // },
}

#[cfg(feature = "imgui")]
impl From<imgui::DrawCmd> for DrawCmd {
    fn from(c: imgui::DrawCmd) -> Self {
        match c {
            imgui::DrawCmd::Elements { cmd_params, count } => Self::Elements { cmd_params: cmd_params.borrow().into(), count: count },
            imgui::DrawCmd::ResetRenderState => Self::ResetRenderState,
            imgui::DrawCmd::RawCallback { .. } => panic!("DrawCmd::RawCallback not supported")
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq)]
pub struct DrawCmdParams {
    /// left, up, right, down
    pub clip_rect: [f32; 4],
    pub texture_id: TextureId,
    pub vtx_offset: usize,
    pub idx_offset: usize,
}

#[cfg(feature = "imgui")]
impl From<&imgui::DrawCmdParams> for DrawCmdParams {
    fn from(p: &imgui::DrawCmdParams) -> Self {
        Self {
            clip_rect: p.clip_rect,
            texture_id: p.texture_id.into(),
            vtx_offset: p.vtx_offset,
            idx_offset: p.idx_offset,
        }
    }
}

/// A vertex index
pub type DrawIdx = u16;

/// A single vertex
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct DrawVert {
    pub pos: [f32; 2],
    pub uv: [f32; 2],
    pub col: [u8; 4],
}

#[cfg(feature = "imgui")]
impl From<&imgui::DrawVert> for DrawVert {
    fn from(v: &imgui::DrawVert) -> Self {
        Self {
            pos: v.pos,
            uv: v.uv,
            col: v.col,
        }
    }
}

/// An opaque texture identifier
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct TextureId(pub(crate) usize);

impl TextureId {
    /// Creates a new texture id with the given identifier.
    #[inline]
    pub const fn new(id: usize) -> Self {
        Self(id)
    }

    /// Returns the id of the TextureId.
    #[inline]
    pub const fn id(self) -> usize {
        self.0
    }
}

#[cfg(feature = "imgui")]
impl From<imgui::TextureId> for TextureId {
    fn from(i: imgui::TextureId) -> Self {
        Self(unsafe { core::mem::transmute(i) })
    }
}

impl From<usize> for TextureId {
    fn from(n: usize) -> Self {
        Self(n)
    }
}
