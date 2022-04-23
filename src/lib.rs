mod perspective;

use fermium::{
    prelude::*,
    video::*,
    *,
};
use glam::UVec2;
use glam::Vec3;
use crate::perspective::render_3d;

pub fn render(renderer: *mut SDL_Renderer, width: u32, height: u32, mut counter: u32) {
    // let p0 = UVec2::new(800, (10 + counter) % height);
    // let p1 = UVec2::new(1900, (970) + counter) % height;
    // let p2 = UVec2::new(10, (1070 + counter) % height);
    //
    // let scene = Scene {
    //     renderer,
    //     width,
    //     height,
    //     d: 10.0
    // };
    //
    // let v0 = Vertex {
    //     point: p0,
    //     color: Color {
    //         r: 255,
    //         g: 0,
    //         b: 0,
    //         a: 255
    //     }
    // };
    //
    // let v1 = Vertex {
    //     point: p1,
    //     color: Color {
    //         r: 255,
    //         g: 0,
    //         b: 0,
    //         a: 180
    //     }
    // };
    //
    // let v2 = Vertex {
    //     point: p2,
    //     color: Color {
    //         r: 255,
    //         g: 0,
    //         b: 0,
    //         a: 100
    //     }
    // };

    // draw_wireframe_triangle(p0, p1, p2, &scene);
    // draw_shaded_triangle(v0, v1, v2, &scene);
    render_3d(scene);
}

pub fn draw_wireframe_triangle(p0: UVec2,p1: UVec2, p2: UVec2, scene: &Scene) {
    draw_line(p0, p1, &scene);
    draw_line(p1, p2, &scene);
    draw_line(p2, p0, &scene);
}

pub fn draw_line(mut p0: UVec2,mut p1: UVec2, scene: &Scene) {
    let renderer  = scene.renderer;
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

pub fn interpolate(i0: u32, d0: f32, i1: u32, d1: f32)-> Vec<f32> {
    let mut values: Vec<f32> = Vec::new();
    let a = ((d1 as f32) - (d0 as f32)) / ((i1 as f32) - (i0 as f32));
    let mut d = d0;
    for _ in i0..i1+1 {
        values.push(d);
        d = d + a;
    }
    values
}

pub fn draw_filled_triangle(mut p0: UVec2,mut p1: UVec2, mut p2: UVec2, renderer: *mut SDL_Renderer) {
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
    let left: &Vec<f32>;
    let right: &Vec<f32>;

    if x02[m] < x012[m] {
        left = &x02;
        right = &x012;
    } else {
        left = &x012;
        right = &x02;
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

pub fn draw_shaded_triangle(v0: Vertex, v1: Vertex, v2: Vertex, scene: &Scene) {
    let renderer = scene.renderer;
    let (mut p0,mut p1,mut p2) = (v0.point, v1.point, v2.point);
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
    let mut a01 = interpolate(p0.y, v0.color.a as f32, p1.y, v1.color.a as f32);

    let x02 = interpolate(p0.y, p0.x as f32, p2.y, p2.x as f32);
    let a02 = interpolate(p0.y, v0.color.a as f32, p2.y, v2.color.a as f32);

    let mut x12 = interpolate(p1.y, p1.x as f32, p2.y, p2.x as f32);
    let mut a12 = interpolate(p1.y, v1.color.a as f32, p2.y, v2.color.a as f32);

    x01.remove(x01.len()-1);
    x01.append(&mut x12);
    let x012 = x01.clone();

    a01.remove(a01.len()-1);
    a01.append(&mut a12);
    let a012 = a01.clone();

    let m = (x02.len() / 2) as  usize;
    let x_left: &Vec<f32>;
    let x_right: &Vec<f32>;
    let a_left: &Vec<f32>;
    let a_right: &Vec<f32>;


    if x02[m] < x012[m] {
        x_left = &x02;
        a_left = &a02;
        x_right = &x012;
        a_right = &a012;
    } else {
        x_left = &x012;
        a_left = &a012;
        x_right = &x02;
        a_right = &a02;
    }

    for y in p0.y..p2.y {
        let p = (y-(p0.y)) as usize;
        let x_l = x_left[p];
        let x_r = x_right[p];
        let a_segment = interpolate(x_l as u32, a_left[p], x_r as u32, a_right[p]);
        for x in x_left[p] as u32..x_right[p] as u32 {
            unsafe {
                SDL_SetRenderDrawColor(renderer, 255, 0, 0, a_segment[x as usize -(x_l as usize)] as u8);
                SDL_RenderDrawPoint(renderer, x as i32, y as i32);
            }
        }
    }
}

pub struct Vertex {
    point: Vec3,
    color: Color
}

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

pub struct Scene {
    pub renderer: *mut SDL_Renderer,
    pub width: u32,
    pub height: u32,
    pub d: f32
}

impl Scene {
    pub fn viewport_to_canvas(&self, x: f32, y: f32)  -> UVec2 {
        UVec2::new(((x + 8.) * (self.width as f32 / 16.)) as u32, ((y  + 5.) * (self.height / 10) as f32) as u32)
    }
    pub fn project_vertex(&self, v: Vec3) -> UVec2 {
        self.viewport_to_canvas(v.x * self.d / v.z, (v.y * self.d / v.z))
    }
}