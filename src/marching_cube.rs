use macroquad::prelude::*;


pub async fn run() {
    let cube_pos  = vec3(0., 0., 0.);
    let cube_size = vec3(20., 20., 20.);
    

    // addding mouse position
    let mut position = vec3(0.0, 1.0, 0.0);
    let mut last_mouse_pos:Vec2 = mouse_position().into();
    let main_up:Vec3 = vec3(0.0, 0.0, 0.0);


    // look up speed
    let LOOK_SPEED = 0.12;

    let mut horz_rot:f32 = 1.10;
    let mut vert_rot:f32 = 0.0;
    
    let mut front = vec3(
            horz_rot.cos() * vert_rot.cos(),
            vert_rot.sin(),
            horz_rot.sin() * vert_rot.cos(),
        ).normalize(); // normalize the vector to only change its magnitude and not the direction
    let mut up;
    let mut right  = front.cross(main_up).normalize();

    //setting cursor grab in the window
    set_cursor_grab(true);

    loop {  
    
        clear_background(DARKGRAY);

        let mut delta = get_frame_time();

        let mut mouse_pos:Vec2 = mouse_position().into();
        let mut mouse_delta = mouse_pos - last_mouse_pos; // change in the mouse position
        last_mouse_pos = mouse_pos;

        // getting the horizontal rotation
        horz_rot += mouse_delta.x * delta * LOOK_SPEED;
        vert_rot += mouse_delta.y * delta * -LOOK_SPEED; // get the neg value at it lies in the y axis
        
        vert_rot = if vert_rot > 1.5 {1.5} else {vert_rot};
        vert_rot = if vert_rot < -1.5 {-1.5} else {vert_rot};

        front = vec3(
            horz_rot.cos() * vert_rot.cos(),
            vert_rot.sin(),
            horz_rot.sin() * vert_rot.cos()
            ).normalize();

        right = front.cross(main_up).normalize();
        up = right.cross(front).normalize();

        // setting the camera based on the mouse movement
        set_camera(&Camera3D {
            position,
            up,
            target: front + position,
            ..Default::default()
         });
        
        draw_cube_wires(cube_pos, cube_size, RED);
        set_default_camera();
        next_frame().await;
    }
    
}
