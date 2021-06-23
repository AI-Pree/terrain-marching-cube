use macroquad::prelude::*;

// importing module lib
mod marching_cube;

fn window_conf() -> Conf {

    Conf {

        window_title  : "Marching cube Terrain generator".to_owned(),
        window_height : 400,
        window_width  : 400,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    // just rendering a cube
    marching_cube::run().await;

    
}
