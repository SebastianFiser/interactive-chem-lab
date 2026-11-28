mod chemistry;
use macroquad::prelude::*;
use chemistry::element::Element;
use chemistry::element::Atom;


#[macroquad::main("Chemlab")]
async fn main() {
    let atoms: Vec<Atom> = vec![
        Atom {
            element: Element {
                protons: 1,
                symbol: "H".to_string(),
                name: "Hydrogen".to_string(),
                electronegativity: 2.20
            },
            position: vec3(0.0, 0.0, 0.0)
        },
        Atom {
            element: Element {
                protons: 8,
                symbol: "O".to_string(),
                name: "Oxygen".to_string(),
                electronegativity: 3.44
            },
            position: vec3(1.0, 0.0, 0.0)
        },
        Atom {
            element: Element {
                protons: 1,
                symbol: "H".to_string(),
                name: "Hydrogen".to_string(),
                electronegativity: 2.20
            },
            position: vec3(0.0, 1.0, 0.0)
        }
    ];

    
    
    
    loop {
        let mut r = 5.0;
        let mut speed = 0.5;
        let time = get_time();
        let angle = time as f32 * speed;
    
        let camera = Camera3D {
            position: vec3(r * angle.cos(), 3.0, r * angle.sin()),
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            fovy: 45.0_f32.to_radians(),
            projection: Projection::Perspective,
            ..Default::default()
        };
        clear_background(LIGHTGRAY);

        set_camera(&camera);
        let time = get_time();

        for atom in &atoms {
            match atom.element.symbol.as_str() {
                "H" => draw_sphere(atom.position, 0.5, None, GRAY),
                "O" => draw_sphere(atom.position, 0.7, None, RED),
                _ => draw_sphere(atom.position, 0.5, None, GRAY)
            }
        }
        draw_grid(10, 1.0, BLACK, GRAY);

        set_default_camera();

        next_frame().await
    }
}