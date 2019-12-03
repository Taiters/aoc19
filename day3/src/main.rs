use std::env;
use std::fs;
use std::cmp;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Line {
    Vertical(Point, i32),
    Horizontal(Point, i32),
}

impl Line {
    fn contains(&self, p: &Point) -> bool {
        match self {
            Line::Vertical(o, d) => p.x == o.x && between(p.y, o.y, o.y + d),
            Line::Horizontal(o, d) => p.y == o.y && between(p.x, o.x, o.x + d),
        }
    }

    fn distance(&self) -> i32 {
        match self {
            Line::Vertical(_, d) | Line::Horizontal(_, d) => d.abs()
        }
    }

    fn distance_to_point(&self, point: &Point) -> i32 {
        match self {
            Line::Vertical(o, _) => (point.y - o.y).abs(),
            Line::Horizontal(o, _) => (point.x - o.x).abs(),
        }
    }

    fn intersection(&self, other: &Line) -> Option<Point> {
        match (self, other) {
            (Line::Vertical(origin, _), Line::Horizontal(other_origin, _)) => {
                let point = Point {
                    x: origin.x,
                    y: other_origin.y,
                };
                
                if self.contains(&point) && other.contains(&point) {
                    Some(point)
                } else {
                    None
                }
            },
            (Line::Horizontal(origin, _), Line::Vertical(other_origin, _)) => {
                let point = Point {
                    x: other_origin.x,
                    y: origin.y,
                };

                if self.contains(&point) && other.contains(&point) {
                    Some(point)
                } else {
                    None
                }
            },
            _ => None
        }
    }
}

fn between(value: i32, a: i32, b: i32) -> bool {
    let low = cmp::min(a, b);
    let high = cmp::max(a, b);

    low < value && value < high
}

fn convert_to_lines(input: &str) -> Vec<Vec<Line>> {
    input.lines()
        .map(|line| {
            line.split(',')
            .fold(Vec::new(), |mut acc, i| {
                let mut i = i.chars();
                let direction = i.next().unwrap();
                let distance: i32 = i.as_str().parse().unwrap();
                let origin = if let Some(previous) = acc.last() {
                    match previous {
                        Line::Vertical(p, p_distance) => Point {
                            x: p.x,
                            y: p.y + p_distance,
                        },
                        Line::Horizontal(p, p_distance) => Point {
                            x: p.x + p_distance,
                            y: p.y,
                        },
                    }
                } else {
                    Point {
                        x: 0,
                        y: 0,
                    }
                };

                acc.push(match direction {
                    'U' => Line::Vertical(origin, distance),
                    'R' => Line::Horizontal(origin, distance),
                    'D' => Line::Vertical(origin, -distance),
                    'L' => Line::Horizontal(origin, -distance),
                    _ => panic!("Unexpected direction"),
                });

                acc
            })
        })
        .collect()
}

fn find_intersections(a: &Vec<Line>, b: &Vec<Line>) -> Vec<(Point, i32)> {
    let mut intersections = Vec::new();
    let mut a_total_distance = 0;
    for line_a in a {
        let mut b_total_distance = 0;

        for line_b in b {
            match line_a.intersection(line_b) {
                Some(point) => {
                    let a_intersect_distance = a_total_distance + line_a.distance_to_point(&point);
                    let b_intersect_distance = b_total_distance + line_b.distance_to_point(&point);
                    intersections.push((point, a_intersect_distance + b_intersect_distance));
                }
                _ => (),
            }

            b_total_distance += line_b.distance();
        }

        a_total_distance += line_a.distance();
    }

    intersections
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = args.get(1).expect("Input file required");
    let input = fs::read_to_string(input_file).expect("Unable to read file");

    let lines = convert_to_lines(&input);
    let intersections = find_intersections(&lines[0], &lines[1]);
    let nearest = intersections.iter()
        // .map(|(p, _)| p.x.abs() + p.y.abs())
        .map(|(_, d)| d)
        .min()
        .expect("Couldn't find an intersection");

    println!("{}", nearest);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        let a = Line::Horizontal(Point { x: 100, y: -100 }, -200);
        let b = Line::Vertical(Point { x: -50, y: 100 }, -600);

        assert_eq!(a.intersection(&b), Some(Point {
            x: -50,
            y: -100
        }));
    }

    #[test]
    fn test_intersection_no_intersect() {
        let a = Line::Horizontal(Point{x: -32, y: 20}, -98);
        let b = Line::Vertical(Point{x: 25, y: 10}, 100);

        assert_eq!(a.intersection(&b), None);
    }

    #[test]
    fn test_intersection_same_direction() {
        let a = Line::Horizontal(Point{x: -32, y: 20}, -98);
        let b = Line::Horizontal(Point{x: 25, y: 10}, 100);

        assert_eq!(a.intersection(&b), None);
    }
}