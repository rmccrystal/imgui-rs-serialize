use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut};
use serde::{Serialize, Deserialize};
use crate::TextureId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    pub font_atlas: FontAtlas,
}

#[cfg(feature = "imgui")]
impl From<&mut imgui::Context> for Context {
    fn from(c: &mut imgui::Context) -> Self {
        Self {
            font_atlas: c.fonts().deref_mut().into()
        }
    }
}

impl Context {
    pub fn fonts(&mut self) -> &mut FontAtlas {
        &mut self.font_atlas
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FontAtlas {
    pub texture: FontAtlasTexture,
    pub tex_id: TextureId,
}

impl FontAtlas {
    pub fn build_rgba32_texture(&mut self) -> &FontAtlasTexture {
        &self.texture
    }
}

#[cfg(feature = "imgui")]
impl From<&mut imgui::FontAtlas> for FontAtlas {
    fn from(a: &mut imgui::FontAtlas) -> Self {
        Self {
            texture: a.build_rgba32_texture().borrow().into(),
            tex_id: a.tex_id.into()
        }
    }
}

#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct FontAtlasTexture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

#[cfg(feature = "imgui")]
impl From<&imgui::FontAtlasTexture<'_>> for FontAtlasTexture {
    fn from(t: &imgui::FontAtlasTexture<'_>) -> Self {
        Self {
            width: t.width,
            height: t.height,
            data: t.data.to_vec(),
        }
    }
}
