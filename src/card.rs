use crate::renderer::Vertex;
use crate::texture::Texture;

pub struct Card {
    pos: [f32; 3],
    flipped: bool,
    tapped: bool,
    front_texture: Texture,
    back_texture: Texture,
    label: String,
}

impl Card {
    pub fn new(
        position: [f32; 3],
        front_image: &str,
        back_image: Option<&str>,
        label: &str
    ) -> Card {
        todo!();
    }

    pub fn create_vertices() -> [Vertex; 4] {
        todo!("create vertices");
    }

    pub fn create_indices() -> [u16; 6] {
        todo!("create indices array");
    }
}
