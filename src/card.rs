use crate::renderer::Vertex;
use crate::texture::Texture;

pub struct Card {
    pos: [f32; 3],
    flipped: bool,
    tapped: bool,
    front_texture: Option<Texture>,
    back_texture: Option<Texture>,
    label: String,
}

impl Card {
    pub fn new(
        position: [f32; 3],
        front_image: &str,
        back_image: Option<&str>,
        label: &str,
    ) -> Card {
        Card {
            pos: position,
            flipped: false,
            tapped: false,
            front_texture: None,
            back_texture: None,
            label: label.to_string(),
        }
    }

    pub fn create_vertices(aspect_ratio: f32) -> [Vertex; 4] {
        [
            Vertex {
                position: [-0.10, -0.1, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [0.10, -0.1, 0.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [0.10, 0.28, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [-0.10, 0.28, 0.0],
                tex_coords: [0.0, 0.0],
            },
        ]
    }

    pub fn create_indices() -> [u16; 6] {
        [0, 1, 2, 0, 2, 3]
    }
}
