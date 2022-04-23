use crate::*;
use fermium::{
    prelude::*,
    video::*,
    *,
};
use glam::UVec2;

pub fn render_3d(scene: Scene) {

    // Front
    let vAf = Vec3::new(-2.0, -0.5, 5.);
    let vBf = Vec3::new(-2.0, 0.5, 5.);
    let vCf = Vec3::new(-1.0, 0.5, 5.);
    let vDf = Vec3::new(-1.0, -0.5, 5.);
    // Back
    let vAb = Vec3::new(-2.0, -0.5, 6.);
    let vBb = Vec3::new(-2.0, 0.5, 6.);
    let vCb = Vec3::new(-1.0, 0.5, 6.);
    let vDb = Vec3::new(-1.0, -0.5, 6.);

    // Front
    // let vAf = Vec3::new(2.0, 0.5, 5.);
    // let vBf = Vec3::new(4.0, 0.5, 5.);
    // let vCf = Vec3::new(4.0, 2.5, 5.);
    // let vDf = Vec3::new(2.0, 2.5, 5.);
    // Back
    // let vAb = Vec3::new(2.0, 0.5, 6.);
    // let vBb = Vec3::new(4.0, 0.5, 6.);
    // let vCb = Vec3::new(4.0, 2.5, 6.);
    // let vDb = Vec3::new(2.0, 2.5, 6.);

    // Frontface
    draw_line(scene.project_vertex(vAf), scene.project_vertex(vBf), &scene);
    draw_line(scene.project_vertex(vBf), scene.project_vertex(vCf), &scene);
    draw_line(scene.project_vertex(vCf), scene.project_vertex(vDf), &scene);
    draw_line(scene.project_vertex(vDf), scene.project_vertex(vAf), &scene);
    // Backface
    draw_line(scene.project_vertex(vAb), scene.project_vertex(vBb), &scene);
    draw_line(scene.project_vertex(vBb), scene.project_vertex(vCb), &scene);
    draw_line(scene.project_vertex(vCb), scene.project_vertex(vDb), &scene);
    draw_line(scene.project_vertex(vDb), scene.project_vertex(vAb), &scene);
    //Connection
    draw_line(scene.project_vertex(vAb), scene.project_vertex(vAf), &scene);
    draw_line(scene.project_vertex(vBb), scene.project_vertex(vBf), &scene);
    draw_line(scene.project_vertex(vCb), scene.project_vertex(vCf), &scene);
    draw_line(scene.project_vertex(vDb), scene.project_vertex(vDf), &scene);

    let mut vertices = Vec::new();
    vertices.push(Vertex {point: Vec3::new(1.,1.,1.), color: Color{ r: 255, g: 0, b: 0, a: 1}});
    vertices.push(Vertex {point: Vec3::new(-1.,1.,1.), color: Color{ r: 255, g: 0, b: 0, a: 1}});
    vertices.push(Vertex {point: Vec3::new(-1.,-1.,1.), color: Color{ r: 255, g: 0, b: 0, a: 1}});
    vertices.push(Vertex {point: Vec3::new(1.,-1.,1.), color: Color{ r: 255, g: 0, b: 0, a: 1}});
    vertices.push(Vertex {point: Vec3::new(1.,1.,-1.), color: Color{ r: 255, g: 0, b: 0, a: 1}});
    vertices.push(Vertex {point: Vec3::new(-1.,1.,-1.), color: Color{ r: 255, g: 0, b: 0, a: 1}});
    vertices.push(Vertex {point: Vec3::new(-1.,-1.,-1.), color: Color{ r: 255, g: 0, b: 0, a: 1}});
    vertices.push(Vertex {point: Vec3::new(1.,-1.,-1.), color: Color{ r: 255, g: 0, b: 0, a: 1}});


}

fn render_object(vertices: Vec<Vertex>, triangles: Vec<[usize; 3]>, scene: &Scene) {
    let mut projected: Vec<UVec2> = Vec::new();
    for vertex in vertices {
        projected.push(scene.project_vertex(vertex.point));
    }
    for t in triangles {
        render_triangle(t, &projected, scene);
    }
}

fn render_triangle(triangle: [usize; 3], projected: &Vec<UVec2>, scene: &Scene) {
    draw_wireframe_triangle(projected[triangle[0]],projected[triangle[1]], projected[triangle[2]], scene);
}