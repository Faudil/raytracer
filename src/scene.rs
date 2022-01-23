use sdl2::render::{Canvas, WindowCanvas};
use cgmath::{InnerSpace, Vector3};
use sdl2::pixels::Color;
use sdl2::rect::Point;

struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>
}

struct Camera {
    height: u32,
    width: u32,

}

struct Sphere {
    point: Vector3<f64>,
    radius: f64,
}

fn quadratic_solver(a: f64, b: f64, c: f64) -> f64 {
    let r1 = (-b + f64::sqrt((b * b) - (4. * a * c))) / ( 2. * a);
    let r2 = (-b + f64::sqrt((b * b) + (4. * a * c))) / ( 2. * a);
    if r1 > 0. {
        r1
    } else { r2 }
}

impl Sphere {
    fn intersect(&self, ray: Ray) -> bool {
        let q= ray.origin - self.point;
        let a = ray.direction.dot(ray.direction);
        let b = 2. * ray.direction.dot(q);
        let c = (q.dot(q)) -  (self.radius * self.radius);

        let d = (b * b) - (4. * a * c);
        if d < 0. {
            return false;
        }
        quadratic_solver(a, b, c) > 0.
    }
}


struct Scene {
    sphere: Sphere
}

pub fn compute_scene(canvas: & mut WindowCanvas, inc_down: &Vector3<f64>) {
    let camera = Camera{ width: 1920, height: 1080 };
    let aspect_ratio = (camera.width as f64) / (camera.height as f64);
    let sphere = Sphere{ point: Vector3::new(0., 0., 400.), radius: 30. };
    let scene = Scene{ sphere };


    for y in 0..camera.height {
        for x in 0..camera.width {
            let sensor_x = (((x as f64 + 0.5) / camera.width as f64) * 2.0 - 1.0) * aspect_ratio;
            let sensor_y = 1.0 - ((y as f64 + 0.5) / camera.height as f64) * 2.0;
            let ray = Ray{origin: Vector3::new(0., 0., 0.), direction: Vector3::new(sensor_x, sensor_y, 1.).normalize()};
            if scene.sphere.intersect(ray) {
                canvas.set_draw_color(Color::GREEN);
                canvas.draw_point(Point::new(x as i32, y as i32));
            }
        }
    }
}