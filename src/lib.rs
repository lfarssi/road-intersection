use macroquad::{prelude::*, rand::ChooseRandom};
#[derive(Debug, PartialEq, Eq)]
pub enum  Route{
    Left,
    Right,
    Straight,
}
pub trait Update {
    fn update(&mut self, dt:f32);
}

pub struct Car {
    position : Vec2,
    speed : Vec2,
    route : Route,
    color: Color,
}

pub struct TrafficLight{
    state: LightState,
    timer:f32,
    red_duration:f32,
    green_duration:f32,
}

pub enum LightState {
    Red,
    Green,
}

impl Update for Car {
    fn update(&mut self, dt: f32) {
        self.position += self.speed * dt;
    }
}

impl Update for TrafficLight{
    fn update(&mut self, dt:f32) {
        self.timer-=dt;
        if self.timer <=0.0 {
            self.state =match self.state {
                LightState::Green => {
                    self.timer=self.red_duration;
                    LightState::Red
                }
                LightState::Red => {
                    self.timer = self.green_duration;
                    LightState::Green
                }
            }
        }
    }
}

pub struct RoadIntersection {
    pub cars: Vec<Car>,
    pub traffic_lights :Vec<TrafficLight>,
}

impl RoadIntersection {
    pub fn new()->Self{
        Self {
            cars:Vec::new(),
            traffic_lights: vec![
                TrafficLight {
                    state: LightState::Green,
                    timer: 3.0,
                    green_duration: 3.0,
                    red_duration: 3.0,
                },
                TrafficLight {
                    state: LightState::Red,
                    timer: 3.0,
                    green_duration: 3.0,
                    red_duration: 3.0,
                },
                TrafficLight {
                    state: LightState::Red,
                    timer: 3.0,
                    green_duration: 3.0,
                    red_duration: 3.0,
                },
                TrafficLight {
                    state: LightState::Red,
                    timer: 3.0,
                    green_duration: 3.0,
                    red_duration: 3.0,
                },
            ]
        }
    }
    pub fn update(&mut self,dt:f32 ){
        for light in &mut self.traffic_lights{
            light.update(dt);
        } 
        for car in &mut self.cars{
            car.update(dt);
        }
    }
    pub fn add_car(&mut self, direction : KeyCode){
        let color = * [PURPLE, YELLOW, BLUE].choose().unwrap();
        let (position, speed)= match direction {
            KeyCode::Up =>(vec2(screen_width() / 2.0+40.0, screen_height()), vec2(0.0, -100.0)),
            KeyCode::Down =>(vec2(screen_width() / 2.0+40.0, screen_height()), vec2(0.0, 100.0)),
            KeyCode::Left =>(vec2(screen_width() / 2.0+40.0, screen_height()), vec2(-100.0, 0.0)),
            KeyCode::Right =>(vec2(screen_width() / 2.0+40.0, screen_height()), vec2(100.0, 0.0)),
            _ =>return ,
        };
        self.cars.push(Car{
            position,
            speed,
            route: Route::Straight,
            color,
        });
    }

    pub fn draw(&self){
        up();
        down();
        // center
        draw_circle_lines(screen_width()/2.0, screen_height()/2.0, 6.0 , 4.0, RED);
        left();
        right();  
    }
}

pub fn  up() {   
        draw_line((screen_width()/2.0)-40.0, screen_height() ,(screen_width()/2.0)-40.0,(screen_height()/2.0)+40.0,1.0, WHITE);
        draw_line(screen_width()/2.0, screen_height() ,screen_width()/2.0,(screen_height()/2.0)+40.0,1.0, WHITE);
        draw_line((screen_width()/2.0)+40.0, screen_height() ,(screen_width()/2.0)+40.0,(screen_height()/2.0)+40.0,1.0, WHITE);
        draw_rectangle((screen_width()/2.0)+40.0, (screen_height()/2.0)-60.0 ,20.0, 20.0, RED);
    }
    

pub fn right(){
        draw_line((screen_width()/2.0) +40.0, (screen_height()/2.0)-40.0 ,screen_width(),(screen_height()/2.0)-40.0,1.0, WHITE);
        draw_line((screen_width()/2.0) +40.0, screen_height()/2.0 ,screen_width(),screen_height()/2.0,1.0, WHITE);
        draw_line((screen_width()/2.0) +40.0, (screen_height()/2.0)+40.0 ,screen_width(),(screen_height()/2.0)+40.0,1.0, WHITE);
        draw_rectangle((screen_width()/2.0)+40.0, (screen_height()/2.0)+40.0 ,20.0, 20.0, GREEN);
}
pub fn down(){
        draw_line((screen_width()/2.0)-40.0, 0.0 ,(screen_width()/2.0)-40.0,(screen_height()/2.0)-40.0,1.0, WHITE);
        draw_line(screen_width()/2.0, 0.0 ,screen_width()/2.0,(screen_height()/2.0)-40.0,1.0, WHITE);
        draw_line((screen_width()/2.0)+40.0, 0.0 ,(screen_width()/2.0)+40.0,(screen_height()/2.0)-40.0,1.0, WHITE);
        draw_rectangle((screen_width()/2.0)-60.0, (screen_height()/2.0)-60.0 ,20.0, 20.0, RED);
}

pub fn left(){
        draw_line(0.0, (screen_height()/2.0)-40.0 ,(screen_width()/2.0)-40.0,(screen_height()/2.0)-40.0,1.0, WHITE);
        draw_line(0.0, screen_height()/2.0 ,(screen_width()/2.0)-40.0,screen_height()/2.0,1.0, WHITE);
        draw_line(0.0, (screen_height()/2.0)+40.0 ,(screen_width()/2.0)-40.0,(screen_height()/2.0)+40.0,1.0, WHITE);
        draw_rectangle((screen_width()/2.0)-60.0, (screen_height()/2.0)+40.0 ,20.0, 20.0, RED);
}