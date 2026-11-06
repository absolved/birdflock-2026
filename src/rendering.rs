use birdflock::bird::Bird;
use birdflock::flock::FlockParameters;

use macroquad::prelude::*;

pub fn draw_birds(birds: &[Bird], params: &FlockParameters) {
    let screen_x_coeff = screen_width() / (params.sim_width as f32);
    let screen_y_coeff = screen_height() / (params.sim_height as f32);
    for bird in birds.iter() { // draw a triangle representing each bird
        let x_draw = (bird.position().x as f32)*screen_x_coeff;
        let y_draw = (bird.position().y as f32)*screen_y_coeff;

        let front = Vec2::new(params.bird_size, 0.0);
        let back_left = Vec2::new(-params.bird_size/2.0,-params.bird_size/2.0);
        let back_right = Vec2::new(-params.bird_size/2.0,params.bird_size/2.0);
        let angle = bird.velocity().y.atan2(bird.velocity().x) as f32;
        let center = Vec2::new(x_draw,y_draw);
        let front_vertex = rotate_point(front,angle) + center;
        let back_left_vertex = rotate_point(back_left,angle) + center;
        let back_right_vertex = rotate_point(back_right,angle) + center;
        draw_triangle(front_vertex, back_left_vertex, back_right_vertex, WHITE);
    }
}

pub fn draw_hud(params: &FlockParameters) {
    let sep_message = format!("Q/A Separation: {:.2}", params.separation_gain);
    draw_text(&sep_message, 20.0, screen_height()-50.0, 20.0, WHITE);
    let coh_message = format!("W/S Cohesion: {:.2}", params.cohesion_gain);
    draw_text(&coh_message, 20.0, screen_height()-35.0, 20.0, WHITE);
    let ali_message = format!("E/D Alignment: {:.2}", params.alignment_gain);
    draw_text(&ali_message, 20.0, screen_height()-20.0, 20.0, WHITE);
    let destination_message = format!("R/F Destination: {:.2}", params.global_cohesion_gain);
    draw_text(&destination_message, 20.0, screen_height()-5.0, 20.0, WHITE);        
}

fn rotate_point(point: Vec2, angle: f32) -> Vec2 {
    Vec2 {x: point.x*(angle.cos())-point.y*(angle.sin()),
          y: point.x*(angle.sin())+point.y*(angle.cos())}
}