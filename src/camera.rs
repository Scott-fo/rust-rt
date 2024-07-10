use crate::{
    colour::Colour,
    hittable_list::HittableList,
    ray::{Point3, Ray},
    utils::sample_square,
    vec3::Vec3,
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i64,
    pub samples_per_pixel: i64,
    pixel_samples_scale: f64,
    image_height: i64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i64, samples_per_pixel: i64) -> Self {
        let mut image_height = (image_width as f64 / aspect_ratio) as i64;

        if image_height < 1 {
            image_height = 1
        }

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        let center = Point3::new(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            pixel_samples_scale,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &HittableList) {
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = Camera::get_ray(&self, i, j);
                    pixel_colour += r.colour(world);
                }
                pixel_colour *= self.pixel_samples_scale;
                pixel_colour.display();
            }
        }

        eprintln!();
        eprintln!("\rDone. \n");
    }

    fn get_ray(&self, i: i64, j: i64) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let origin = self.center;
        let direction = pixel_sample - origin;

        Ray::new(origin, direction)
    }
}
