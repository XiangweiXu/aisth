use crate::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle>) -> Mesh {
        Mesh { triangles }
    }

    pub fn from_obj(path: &str) -> Mesh {
        let mut vs: Vec<Vector3> = Vec::new();
        let mut ns: Vec<Vector3> = Vec::new();
        let mut triangles = Vec::<Triangle>::new();

        let file = File::open(path).expect("Cannot open file.");
        for line in BufReader::new(file).lines() {
            let ln = line.expect("Cannot read file.");
            if let Some(s) = ln.split_once(' ') {
                match s.0 {
                    "v" => {
                        let v: Vec<&str> = s.1.split(' ').collect();
                        let vertice = Vector3::new([
                            v[0].parse::<Float>().unwrap(),
                            v[1].parse::<Float>().unwrap(),
                            v[2].parse::<Float>().unwrap(),
                        ]);
                        vs.push(vertice);
                    }
                    "vn" => {
                        let n = s.1.split(' ').collect::<Vec<&str>>();
                        let normal = Vector3::new([
                            n[0].parse::<Float>().unwrap(),
                            n[1].parse::<Float>().unwrap(),
                            n[2].parse::<Float>().unwrap(),
                        ]);
                        ns.push(normal);
                    }
                    "f" => {
                        let p = s.1.replacen("//", " ", 3);
                        let i: Vec<&str> = p.split(' ').collect();
                        let v = (
                            i[0].parse::<usize>().unwrap(),
                            i[2].parse::<usize>().unwrap(),
                            i[4].parse::<usize>().unwrap(),
                        );
                        let vn = (
                            i[1].parse::<usize>().unwrap(),
                            i[3].parse::<usize>().unwrap(),
                            i[5].parse::<usize>().unwrap(),
                        );
                        let vertices = [
                            vs[v.0.checked_sub(1).unwrap_or(0)],
                            vs[v.1.checked_sub(1).unwrap_or(0)],
                            vs[v.2.checked_sub(1).unwrap_or(0)],
                        ];
                        let normals = [
                            ns[vn.0.checked_sub(1).unwrap_or(0)],
                            ns[vn.1.checked_sub(1).unwrap_or(0)],
                            ns[vn.2.checked_sub(1).unwrap_or(0)],
                        ];
                        triangles.push(Triangle::new(vertices, normals));
                    }
                    _ => (),
                }
            }
        }

        Mesh { triangles }
    }
}

impl Shape for Mesh {
    fn intersect(&self, ray: &Ray, bound: &Interval) -> Option<Intersection> {
        let mut intersection = None;
        let mut b = bound.clone();
        for t in self.triangles.iter() {
            if let Some(candidate) = t.intersect(ray, &b) {
                b[0].1 = candidate.distance;
                intersection = Some(candidate);
            }
        }

        intersection
    }
}
