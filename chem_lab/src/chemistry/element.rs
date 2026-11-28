use macroquad::prelude::Vec3;

pub struct Element {
    pub protons: u8,
    pub symbol: String,
    pub name: String,
    pub electronegativity: f32
}

pub struct Atom {
    pub element: Element,
    pub position: Vec3
}