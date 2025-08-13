    use macroquad::prelude::*;
    use road_intersection::*;

    #[macroquad::main("Road Intersection")]

    async fn main() {
        loop {
            clear_background(BLACK);
            if is_key_pressed(KeyCode::Escape){
                break;
            }
            up();
            if is_key_pressed(KeyCode::Up) {
                draw_rectangle((screen_width()/2.0 )+5.0, screen_height(), 30.0, 20.0, RED);
            }
            // car_up();
            down();
            // center
            draw_circle_lines(screen_width()/2.0, screen_height()/2.0, 6.0 , 4.0, RED);
            left();
            right();      
            next_frame().await;
        }
    }