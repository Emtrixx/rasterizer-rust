use fermium::{
    prelude::*,
    video::*,
    *,
};
use glam::UVec2;

pub fn render(renderer: *mut SDL_Renderer, width: u32, height: u32, mut counter: u32) {
    let p0 = UVec2::new(800, (10 + counter) % height);
    let p1 = UVec2::new(1900, (970) + counter) % height;
    let p2 = UVec2::new(10, (1070 + counter) % height);
    draw_wireframe_triangle(p0, p1, p2, renderer);
    draw_filled_triangle(p0, p1, p2, renderer);
}

fn draw_wireframe_triangle(p0: UVec2,p1: UVec2, p2: UVec2, renderer: *mut SDL_Renderer) {
    draw_line(p0, p1, renderer);
    draw_line(p1, p2, renderer);
    draw_line(p2, p0, renderer);
}

fn draw_line(mut p0: UVec2,mut p1: UVec2, renderer: *mut SDL_Renderer) {
    let dx: i32 = p1.x as i32 - p0.x as i32;
    let dy: i32 = p1.y as i32 - p0.y as i32;
    if dx.abs() > dy.abs() {
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
        let xs = interpolate(p0.y,p0.x as f32, p1.y, p1.x as f32);
        for y in p0.y..p1.y+1 {
            unsafe {
                SDL_SetRenderDrawColor(renderer, 255, 0, 0, 255);
                SDL_RenderDrawPoint(renderer, xs[y as usize - p0.y as usize] as i32, y as i32);
            }
        }
    }
}

fn interpolate(i0: u32, d0: f32, i1: u32, d1: f32)-> Vec<f32> {
    let mut values: Vec<f32> = Vec::new();
    let a = ((d1 as f32) - (d0 as f32)) / ((i1 as f32) - (i0 as f32));
    let mut d = d0;
    for _ in i0..i1+1 {
        values.push(d);
        d = d + a;
    }
    values
}

fn draw_filled_triangle(mut p0: UVec2,mut p1: UVec2, mut p2: UVec2, renderer: *mut SDL_Renderer) {
    if p0.y > p1.y {
        (p0, p1) = (p1, p0);
    }
    if p0.y > p2.y {
        (p0, p2) = (p2, p0);
    } 
    if p1.y > p2.y {
        (p1, p2) = (p2, p1);
    }

    let mut x01 = interpolate(p0.y, p0.x as f32, p1.y, p1.x as f32);
    let x02 = interpolate(p0.y, p0.x as f32, p2.y, p2.x as f32);
    let mut x12 = interpolate(p1.y, p1.x as f32, p2.y, p2.x as f32);

    x01.remove(x01.len()-1);
    x01.append(&mut x12);
    let x012 = x01.clone();

    let m = (x02.len() / 2) as  usize;
    let left: Vec<f32>;
    let right: Vec<f32>;

    if x02[m] < x012[m] {
        left = x02.clone();
        right = x012.clone();
    } else {
        left = x012.clone();
        right = x02.clone();
    }

    for y in p0.y..p2.y {
        let p = (y-(p0.y)) as usize;
        for x in left[p] as u32..right[p] as u32 {
            unsafe {
                SDL_SetRenderDrawColor(renderer, 255, 0, 0, 255);
                SDL_RenderDrawPoint(renderer, x as i32, y as i32);
            }
        }
    }
}