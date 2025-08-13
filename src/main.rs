    use macroquad::{prelude::*, rand::ChooseRandom};
    use road_intersection::*;

    #[macroquad::main("Road Intersection")]

    async fn main() {
        let mut road = RoadIntersection::new();
        let mut timer = get_time();
        loop {
            clear_background(BLACK);
            draw_text("Press Arrows to add a car", 10.0, 20.0, 20.0, GREEN);
            draw_text("Press R for random positions", 10.0, 40.0, 20.0, GREEN);
            draw_text("Escape to quit", 10.0, 60.0, 20.0, GREEN);
            let dt  = get_frame_time();
            if get_time()-timer > 0.5 {
                if is_key_pressed(KeyCode::Up){
                    road.add_car(KeyCode::Up);
                    timer = get_time();
                }
                 if is_key_pressed(KeyCode::Down){
                    road.add_car(KeyCode::Down);
                    timer = get_time();
                }
                 if is_key_pressed(KeyCode::Left){
                    road.add_car(KeyCode::Left);
                    timer = get_time();
                }
                 if is_key_pressed(KeyCode::Right){
                    road.add_car(KeyCode::Right);
                    timer = get_time();
                }
                 if is_key_pressed(KeyCode::R){

                    road.add_car(*[KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right].choose().unwrap());
                    timer = get_time();
                }
            }
            if is_key_pressed(KeyCode::Escape){
                break;
            }
            road.update(dt);
            road.draw();
            next_frame().await;
        }
    }