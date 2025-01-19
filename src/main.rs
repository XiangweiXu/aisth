use aisth::rt::{camera::Camera, film::Film};
use aisth::*;

fn main() {
    let mut film = Film::new(80, 45);
    let camera = Camera::build(
        film.clone(),
        Vector3::new([-3.0, 1.0, 0.0]),
        Vector3::new([0.0, 1.0, 0.0]),
        60.0,
    );

    let p = Plane::new(Vector3::new([0.0, 0.0, 0.0]), Vector3::new([0.0, 1.0, 0.0]));

    for i in 0..film.size().0 {
        for j in 0..film.size().1 {
            if let Some(_) = p.intersect(
                &camera.cast_ray(i, j),
                &Interval::new([(1E-5, Float::INFINITY)]),
            ) {
                film.write(i, j, Vector::<3>::new([0.5, 0.7, 0.3]));
            } else {
                film.write(i, j, camera.cast_ray(i, j).d)
            }
        }
    }

    film.save("test.ppm").expect("Cannot create file.");
}
