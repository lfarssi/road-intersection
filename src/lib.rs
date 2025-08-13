use macroquad::{prelude::*, rand::ChooseRandom};

#[derive(Debug, PartialEq, Eq)]
pub enum Route {
    Left,
    Right,
    Straight,
}

pub trait Update {
    fn update(&mut self, dt: f32);
}
#[derive(Debug, PartialEq)]

pub struct Car {
    position: Vec2,
    speed: Vec2,
    route: Route,
    color: Color,
    turned: bool, 

}
#[derive(Debug, PartialEq)]

pub struct TrafficLight {
    state: LightState,
    green_duration: f32,
    red_duration: f32,
}
#[derive(Debug, PartialEq)]

pub enum LightState {
    Red,
    Green,
}
impl Car {
    fn maybe_turn(&mut self) {
        if self.turned {
            return; // Already turned
        }

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Coming from bottom (Up direction)
        if self.speed.y < 0.0 && self.position.y < center_y -40.0 {
            match self.color {
                YELLOW => { // turn left
                    self.speed = vec2(-100.0, 0.0);
                    self.route = Route::Left;
                }
                PURPLE => { // turn right
                    self.speed = vec2(100.0, 100.0);
                    self.route = Route::Right;
                }
                BLUE => { // straight, do nothing
                    self.route = Route::Straight;
                }
                _ => {}
            }
            self.turned = true;
        }

        // Coming from top (Down direction)
        if self.speed.y > 0.0 && self.position.y > center_y {
            match self.color {
                YELLOW => { self.speed = vec2(100.0, 0.0); self.route = Route::Right; }
                PURPLE => { self.speed = vec2(-100.0, 0.0); self.route = Route::Left; }
                BLUE => { self.route = Route::Straight; }
                _ => {}
            }
            self.turned = true;
        }

        // Coming from left (Right direction)
        if self.speed.x > 0.0 && self.position.x > center_x{
            match self.color {
                YELLOW => { self.speed = vec2(0.0, -100.0); self.route = Route::Left; }
                PURPLE => { self.speed = vec2(0.0, 100.0); self.route = Route::Right; }
                BLUE => { self.route = Route::Straight; }
                _ => {}
            }
            self.turned = true;
        }

        // Coming from right (Left direction)
        if self.speed.x < 0.0 && self.position.x < center_x -40.0 {
            match self.color {
                YELLOW => { self.speed = vec2(0.0, 100.0); self.route = Route::Left; }
                PURPLE => { self.speed = vec2(0.0, -100.0); self.route = Route::Right; }
                BLUE => { self.route = Route::Straight; }
                _ => {}
            }
            self.turned = true;
        }
    }
}

impl Update for Car {
    fn update(&mut self, dt: f32) {
        self.position += self.speed * dt;
        self.maybe_turn(); 
    }
}


pub struct RoadIntersection {
    pub cars: Vec<Car>,
    pub traffic_lights: Vec<TrafficLight>,
    // Sequential control fields
    current_light_index: usize,
    timer: f32,
}

impl RoadIntersection {
    pub fn new() -> Self {
        Self {
            cars: Vec::new(),
            traffic_lights: vec![
                TrafficLight {
                    state: LightState::Green,
                    green_duration: 3.0,
                    red_duration: 3.0,
                },
                TrafficLight {
                    state: LightState::Red,
                    green_duration: 3.0,
                    red_duration: 3.0,
                },
                TrafficLight {
                    state: LightState::Red,
                    green_duration: 3.0,
                    red_duration: 3.0,
                },
                TrafficLight {
                    state: LightState::Red,
                    green_duration: 3.0,
                    red_duration: 3.0,
                },
            ],
            current_light_index: 0,
            timer: 3.0, // Start with green duration
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Update cars
        for car in &mut self.cars {
            car.update(dt);
        }

        // Update traffic light sequence
        self.timer -= dt;
        
        if self.timer <= 0.0 {
            // Current light turns red
            self.traffic_lights[self.current_light_index].state = LightState::Red;
            
            // Move to next light
            self.current_light_index = (self.current_light_index + 1) % self.traffic_lights.len();
            
            // Set next light to green and reset timer
            self.traffic_lights[self.current_light_index].state = LightState::Green;
            self.timer = self.traffic_lights[self.current_light_index].green_duration;
        }
    }

    pub fn add_car(&mut self, direction: KeyCode) {
        let color = *[PURPLE, YELLOW, BLUE].choose().unwrap();
        let (position, speed) = match direction {
            KeyCode::Up => (vec2(screen_width() / 2.0 + 10.0, screen_height()), vec2(0.0, -100.0)),
            KeyCode::Down => (vec2(screen_width() / 2.0 - 40.0, 0.0), vec2(0.0, 100.0)),
            KeyCode::Left => (vec2(screen_width(), screen_height() / 2.0 - 40.0), vec2(-100.0, 0.0)),
            KeyCode::Right => (vec2(0.0, screen_height() / 2.0 + 10.0), vec2(100.0, 0.0)),
            _ => return,
        };
        self.cars.push(Car {
            position,
            speed,
            route: Route::Straight,
            color,
            turned: false, 

        });
    }

    pub fn draw(&self) {
        // center

        let feux_rouge = [
            vec2((screen_width() / 2.0) + 40.0, (screen_height() / 2.0) - 60.0),
            vec2((screen_width() / 2.0) + 40.0, (screen_height() / 2.0) + 40.0),
            vec2((screen_width() / 2.0) - 60.0, (screen_height() / 2.0) - 60.0),
            vec2((screen_width() / 2.0) - 60.0, (screen_height() / 2.0) + 40.0),
        ];

        for (i, traffic_light) in self.traffic_lights.iter().enumerate() {
            let color = match traffic_light.state {
                LightState::Green => GREEN,
                LightState::Red => RED,
            };
            draw_rectangle(feux_rouge[i].x, feux_rouge[i].y, 20.0, 20.0, color);
        }
        
        up();
        down();
        left();
        right();
         draw_circle_lines(screen_width() / 2.0, screen_height() / 2.0, 6.0, 2.0, WHITE);

        for car in &self.cars {
            draw_rectangle(car.position.x, car.position.y, 30.0, 30.0, car.color);
        }
    }
}

pub fn up() {
    draw_rectangle(screen_width() / 2.0 - 40.0, screen_height() / 2.0, 80.0, screen_height(), GRAY);

    draw_line((screen_width() / 2.0) - 40.0, screen_height(), (screen_width() / 2.0) - 40.0, (screen_height() / 2.0) + 40.0, 1.0, WHITE);
    draw_dashed_lines(screen_width() / 2.0, screen_height() / 2.0 + 40.0, screen_width() / 2.0, screen_height() + 40.0, 10.0, 5.0, 1.0, WHITE);
    draw_line((screen_width() / 2.0) + 40.0, screen_height(), (screen_width() / 2.0) + 40.0, (screen_height() / 2.0) + 40.0, 1.0, WHITE);
}

pub fn right() {
    draw_rectangle(screen_width() / 2.0 + 40.0, screen_height() / 2.0 - 40.0, screen_width() / 2.0 - 40.0, 80.0, GRAY);

    draw_line((screen_width() / 2.0) + 40.0, (screen_height() / 2.0) - 40.0, screen_width(), (screen_height() / 2.0) - 40.0, 1.0, WHITE);
    draw_dashed_lines((screen_width() / 2.0) + 40.0, screen_height() / 2.0, screen_width(), screen_height() / 2.0, 10.0, 5.0, 1.0, WHITE);
    draw_line((screen_width() / 2.0) + 40.0, (screen_height() / 2.0) + 40.0, screen_width(), (screen_height() / 2.0) + 40.0, 1.0, WHITE);
}

pub fn down() {
    draw_rectangle(screen_width() / 2.0 - 40.0, 0.0, 80.0, screen_height() / 2.0, GRAY);

    draw_line((screen_width() / 2.0) - 40.0, 0.0, (screen_width() / 2.0) - 40.0, (screen_height() / 2.0) - 40.0, 1.0, WHITE);
    draw_dashed_lines(screen_width() / 2.0, 0.0, screen_width() / 2.0, (screen_height() / 2.0) - 50.0, 10.0, 5.0, 1.0, WHITE);
    draw_line((screen_width() / 2.0) + 40.0, 0.0, (screen_width() / 2.0) + 40.0, (screen_height() / 2.0) - 40.0, 1.0, WHITE);
}

pub fn left() {
    draw_rectangle(0.0, screen_height() / 2.0 - 40.0, screen_width() / 2.0 - 40.0, 80.0, GRAY);

    draw_line(0.0, (screen_height() / 2.0) - 40.0, (screen_width() / 2.0) - 40.0, (screen_height() / 2.0) - 40.0, 1.0, WHITE);
    draw_dashed_lines(0.0, screen_height() / 2.0, (screen_width() / 2.0) - 40.0, screen_height() / 2.0, 10.0, 5.0, 1.0, WHITE);
    draw_line(0.0, (screen_height() / 2.0) + 40.0, (screen_width() / 2.0) - 40.0, (screen_height() / 2.0) + 40.0, 1.0, WHITE);
}

pub fn draw_dashed_lines(
    start_x: f32,
    start_y: f32,
    end_x: f32,
    end_y: f32,
    dash_length: f32,
    gap_length: f32,
    thickness: f32,
    color: Color,
) {
    let dx = end_x - start_x;
    let dy = end_y - start_y;

    // Horizontal
    if dy.abs() < f32::EPSILON {
        let mut x = start_x;
        while x < end_x {
            let next_x = (x + dash_length).min(end_x);
            draw_line(x, start_y, next_x, start_y, thickness, color);
            x += dash_length + gap_length;
        }
    }
    // Vertical
    else if dx.abs() < f32::EPSILON {
        let mut y = start_y;
        while y < end_y {
            let next_y = (y + dash_length).min(end_y);
            draw_line(start_x, y, start_x, next_y, thickness, color);
            y += dash_length + gap_length;
        }
    }
}