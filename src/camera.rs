use std::{
    sync::{Arc, Mutex},
    thread::{spawn, JoinHandle},
    vec,
};

use crate::{
    colour::Colour,
    hittable_list::HittableList,
    ray::{Point3, Ray},
    utils::sample_square,
    vec3::Vec3,
};

#[derive(Clone)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i64,
    pub samples_per_pixel: i64,
    pub max_depth: i64,
    pixel_samples_scale: f64,
    image_height: i64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i64,
        samples_per_pixel: i64,
        max_depth: i64,
    ) -> Self {
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
            max_depth,
            pixel_samples_scale,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: Arc<HittableList>) {
        let num_threads = std::thread::available_parallelism().unwrap().get();
        let chunk_size = self.image_height as usize / num_threads;

        let results = Arc::new(Mutex::new(vec![String::new(); self.image_height as usize]));

        (0..num_threads)
            .map(|t| {
                let start = t * chunk_size;
                let end = match t == num_threads - 1 {
                    true => self.image_height as usize,
                    false => (t + 1) * chunk_size,
                };

                let results_clone = Arc::clone(&results);
                let world_clone = Arc::clone(&world);
                let self_clone = self.clone();
                spawn(move || {
                    Camera::render_chunk(
                        t,
                        self_clone,
                        start,
                        end,
                        world_clone,
                        results_clone,
                        t == num_threads - 1,
                    );
                })
            })
            .collect::<Vec<JoinHandle<()>>>()
            .into_iter()
            .for_each(|handle| handle.join().unwrap());

        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        let results = Arc::try_unwrap(results).unwrap().into_inner().unwrap();
        for line in results {
            print!("{}", line);
        }

        eprintln!("");
    }

    fn render_chunk(
        chunk: usize,
        camera: Camera,
        start: usize,
        end: usize,
        world: Arc<HittableList>,
        results: Arc<Mutex<Vec<String>>>,
        report: bool,
    ) {
        for j in start..end {
            if report {
                eprint!(
                    "\rChunk {} working on line {} of {}",
                    { chunk },
                    { j - start },
                    { end - start }
                );
            }
            let mut line_result = String::new();
            for i in 0..camera.image_width {
                let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);
                for _ in 0..camera.samples_per_pixel {
                    let r = Camera::get_ray(&camera, i, j as i64);
                    pixel_colour += r.colour(&world, camera.max_depth);
                }
                pixel_colour *= camera.pixel_samples_scale;
                line_result.push_str(&pixel_colour.write());
            }
            results.lock().unwrap()[j] = line_result;
        }
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
