use crate::*;
use std::{fs::File, io::Write};

#[derive(Clone, Debug)]
pub struct Film {
    width: usize,
    height: usize,
    data: Vec<Vector<3>>,
}

impl Film {
    pub fn new(width: usize, height: usize) -> Film {
        let data = vec![Vector::<3>::new([0.0, 0.0, 0.0]); width * height];
        Film {
            width,
            height,
            data,
        }
    }

    pub fn write(&mut self, x: usize, y: usize, value: Vector<3>) {
        self.data[x + y * self.width] = value;
    }

    pub fn read(&self, x: usize, y: usize) -> Vector<3> {
        self.data[x + y * self.width]
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn aspect(&self) -> Float {
        self.width as Float / self.height as Float
    }

    pub fn save(&self, path: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(path).expect("Cannot create file.");
        file.write_all(format!("P6 {} {} 255\n", self.width, self.height).as_bytes())
            .expect("Cannot write data to file.");

        let mut content = Vec::with_capacity(self.width * self.height * 3);
        self.data
            .iter()
            .map(|v| {
                let b = Bound::<3>::new([(0.0, 1.0), (0.0, 1.0), (0.0, 1.0)]);
                let v = &b.clamp(v);
                let r = (v[0] * 255.999) as u8;
                let g = (v[1] * 255.999) as u8;
                let b = (v[2] * 255.999) as u8;
                content.extend(vec![r, g, b]);
            })
            .for_each(drop);
        file.write_all(&content)
            .expect("Cannot write data to file.");
        Ok(())
    }
}
