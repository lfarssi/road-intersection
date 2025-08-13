use macroquad::prelude::*;
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
        if self.timer <=0 {
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

pub fn  up() {   
        draw_line((screen_width()/2.0)-40.0, screen_height() ,(screen_width()/2.0)-40.0,(screen_height()/2.0)+40.0,1.0, WHITE);
        draw_line(screen_width()/2.0, screen_height() ,screen_width()/2.0,(screen_height()/2.0)+40.0,1.0, WHITE);
        draw_line((screen_width()/2.0)+40.0, screen_height() ,(screen_width()/2.0)+40.0,(screen_height()/2.0)+40.0,1.0, WHITE);
        draw_rectangle((screen_width()/2.0)+40.0, (screen_height()/2.0)-60.0 ,20.0, 20.0, RED);
    }
    
pub fn car_up(){
    let mut timer = get_time();
    let mut i=0;
    while   i < screen_height() as i32{
        if get_time()-timer > 0.4{
            draw_rectangle((screen_width()/2.0 )+5.0, screen_height() - i as f32 ,30.0, 20.0, RED);
            timer = get_time();
            println!("{}", timer); 
            i+=1;
        }
    }
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