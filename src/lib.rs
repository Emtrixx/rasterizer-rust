use fermium::{
    prelude::*,
    video::*,
    *,
};
use glam::UVec2;

pub fn render(renderer: *mut SDL_Renderer, width: u32, height: u32) {
    draw_line(UVec2::new(10, 90), UVec2::new(90, 30), renderer);
}

fn draw_line(mut p0: UVec2,mut p1: UVec2, renderer: *mut SDL_Renderer) {
    let dx: i32 = p1.x as i32 - p0.x as i32;
    let dy: i32 = p1.y as i32 - p0.y as i32;
    if dx > dy {
        // Line is horizontal-ish
        if p0.x > p1.x {
            (p0, p1) = (p1, p0);
        }
        let ys = interpolate(p0.x,p0.y as f32, p1.x, p1.y as f32);
        for x in p0.x..p1.x+1 {
            unsafe {
                SDL_SetRenderDrawColor(renderer, 255, 0, 0, 255);
                SDL_RenderDrawPoint(renderer, x as i32, ys[x as usize - p0.x as usize] as i32);
            }
        }
    } else {
        // Line is vertical-ish
        if p0.y > p1.y {
            (p0, p1) = (p1, p0);
        }
        let a = (dx as f32) / (dy as f32);
        let mut x = p0.x as f32;
        for y in p0.y..p1.y+1 {
            unsafe {
                SDL_SetRenderDrawColor(renderer, 255, 0, 0, 255);
                SDL_RenderDrawPoint(renderer, x as i32, y as i32);
            }
            x = x + a;
        }
    }
}

fn interpolate(i0: u32, d0: f32, i1: u32, d1: f32)-> Vec<f32> {
    let mut values: Vec<f32> = Vec::new();
    let a = (d1 - d0) / (i1 - i0) as f32;
    let mut d = d0;
    for _ in i0..i1+1 {
        values.push(d);
        d = d + a;
    }
    values
}