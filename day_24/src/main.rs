use itertools::Itertools;
use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug)]
struct Hailstone {
    position: (f64, f64, f64),
    velocity: (f64, f64, f64),
}

impl Hailstone {
    fn new(position: (f64, f64, f64), velocity: (f64, f64, f64)) -> Self {
        Self { position, velocity }
    }

    fn intersect_with_2d(&self, other: &Hailstone) -> Option<(f64, f64, f64)> {
        // Represent a line as y = mx + 2
        let m1 = self.velocity.1 / self.velocity.0;
        let m2 = other.velocity.1 / other.velocity.0;
        let q1 = self.position.1 - m1 * self.position.0;
        let q2 = other.position.1 - m2 * other.position.0;

        // Parallel
        if m1 == m2 {
            None
        }
        // Intersect at some point
        else {
            let y = (q1 * m2 - q2 * m1) / (m2 - m1);
            let x = (y - q2) / m2;
            Some((x, y, 0.0))
        }
    }

    fn is_point_in_future_2d(&self, point: &(f64, f64, f64)) -> bool {
        assert!(self.velocity.0 != 0.0);
        (self.velocity.0 > 0.0 && point.0 > self.position.0)
            || (self.velocity.0 < 0.0 && point.0 < self.position.0)
    }
}

fn line_intercepting_hailstones(h1: &Hailstone, h2: &Hailstone, h3: &Hailstone) -> (f64, f64, f64) {
    // Equation of a line ax + by + cz + d = 0
    // Each line tells us the a/

    todo!();
}

fn is_point_in_area_2d(
    point: &(f64, f64, f64),
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
) -> bool {
    point.0 >= min_x && point.0 <= max_x && point.1 >= min_y && point.1 <= max_y
}

fn parse_input() -> Vec<Hailstone> {
    let f = File::open("input/input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();
    lines
        .map(|line| {
            let line = line.unwrap();
            let mut line = line.split("@");

            let position = line.next().unwrap();
            let position = position.replace(" ", "");
            let position = position.split(",");
            let position = position
                .map(|coord| coord.parse::<f64>().unwrap())
                .collect_tuple()
                .unwrap();

            let velocity = line.next().unwrap();
            let velocity = velocity.replace(" ", "");
            let velocity = velocity.split(",");
            let velocity = velocity
                .map(|coord| coord.parse::<f64>().unwrap())
                .collect_tuple()
                .unwrap();

            Hailstone::new(position, velocity)
        })
        .collect_vec()
}

fn intersect_within_area_2d(
    hailstones: &[Hailstone],
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
) -> usize {
    hailstones
        .iter()
        .tuple_combinations()
        .filter_map(|(h1, h2)| {
            let point = h1.intersect_with_2d(h2);
            if let Some(point) = point {
                if h1.is_point_in_future_2d(&point)
                    && h2.is_point_in_future_2d(&point)
                    && is_point_in_area_2d(&point, min_x, max_x, min_y, max_y)
                {
                    Some(point)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .count()
}

fn main() {
    let hailstones = parse_input();

    // First part
    let res = intersect_within_area_2d(
        &hailstones,
        200000000000000.0,
        400000000000000.0,
        200000000000000.0,
        400000000000000.0,
    );
    println!("Number of intersections: {}", res);
}
