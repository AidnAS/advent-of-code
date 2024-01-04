#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Clone, Copy)]
struct Vector {
    point: Point,
    velocity: Point,
}

fn parse(input: &str) -> Vec<Vector> {
    input
        .lines()
        .map(|line| {
            let line = line
                .split([',', ' ', '@'])
                .filter(|c| !c.is_empty())
                .collect::<Vec<_>>();
            let point = Point {
                x: line[0].parse::<f64>().unwrap(),
                y: line[1].parse::<f64>().unwrap(),
                z: line[2].parse::<f64>().unwrap(),
            };
            let velocity = Point {
                x: line[3].parse::<f64>().unwrap(),
                y: line[4].parse::<f64>().unwrap(),
                z: line[5].parse::<f64>().unwrap(),
            };
            Vector { point, velocity }
        })
        .collect::<Vec<_>>()
}

fn inside(point: &Point, min: f64, max: f64) -> bool {
    if point.x > min && point.x < max && point.y > min && point.y < max {
        return true;
    }
    false
}

fn intersect2d_xz(vector1: &Vector, vector2: &Vector) -> Option<Point> {
    let m1 = vector1.velocity.z / vector1.velocity.x;
    let m2 = vector2.velocity.z / vector2.velocity.x;
    if m1 == m2 {
        return None;
    }
    let b1 = vector1.point.z - m1 * vector1.point.x;
    let b2 = vector2.point.z - m2 * vector2.point.x;
    let x = (b2 - b1) / (m1 - m2);
    let z = m1 * x + b1;
    let t1 = (x - vector1.point.x) / vector1.velocity.x;
    let t2 = (x - vector2.point.x) / vector2.velocity.x;
    if t1 > 0.0 && t2 > 0.0 {
        return Some(Point { x, y: 0.0, z });
    }
    None
}
fn intersect2d_xy(vector1: &Vector, vector2: &Vector) -> Option<Point> {
    let m1 = vector1.velocity.y / vector1.velocity.x;
    let m2 = vector2.velocity.y / vector2.velocity.x;
    if m1 == m2 {
        return None;
    }
    let b1 = vector1.point.y - m1 * vector1.point.x;
    let b2 = vector2.point.y - m2 * vector2.point.x;
    let x = (b2 - b1) / (m1 - m2);
    let y = m1 * x + b1;
    let t1 = (x - vector1.point.x) / vector1.velocity.x;
    let t2 = (x - vector2.point.x) / vector2.velocity.x;
    if t1 > 0.0 && t2 > 0.0 {
        return Some(Point { x, y, z: 0.0 });
    }
    None
}

fn part1(input: &str, min: f64, max: f64) -> usize {
    let vectors = parse(input);
    vectors
        .iter()
        .enumerate()
        .flat_map(|(i, vector1)| {
            vectors[i + 1..].iter().filter_map(|vector2| {
                intersect2d_xy(vector1, vector2).filter(|intersect| inside(intersect, min, max))
            })
        })
        .count()
}

fn part2(input: &str) -> usize {
    let vectors = parse(input);
    for vx in -500..=500 {
        for vy in -500..=500 {
            let mut vector1 = vectors[0];
            let mut vector2 = vectors[1];
            vector1.velocity.x += vx as f64;
            vector1.velocity.y += vy as f64;
            vector2.velocity.x += vx as f64;
            vector2.velocity.y += vy as f64;
            if let Some(intersect_xy) = intersect2d_xy(&vector1, &vector2) {
                let found_xy = vectors.iter().all(|vector| {
                    let mut vector = *vector;
                    vector.velocity.x += vx as f64;
                    vector.velocity.y += vy as f64;
                    let diff = (intersect_xy.x - vector.point.x) * vector.velocity.y
                        - (intersect_xy.y - vector.point.y) * vector.velocity.x;
                    diff == 0.0
                });
                if found_xy {
                    for vz in -500..=500 {
                        let mut vector1 = vectors[0];
                        let mut vector2 = vectors[1];
                        vector1.velocity.x += vx as f64;
                        vector2.velocity.x += vx as f64;
                        vector1.velocity.z += vz as f64;
                        vector2.velocity.z += vz as f64;
                        if let Some(intersect_xz) = intersect2d_xz(&vector1, &vector2) {
                            let found_xz = vectors.iter().all(|vector| {
                                let mut vector = *vector;
                                vector.velocity.x += vx as f64;
                                vector.velocity.z += vz as f64;
                                let diff = (intersect_xz.x - vector.point.x) * vector.velocity.z
                                    - (intersect_xz.z - vector.point.z) * vector.velocity.x;
                                diff == 0.0
                            });
                            if found_xz {
                                return (intersect_xy.x + intersect_xy.y + intersect_xz.z) as usize;
                            }
                        }
                    }
                }
            }
        }
    }
    1
}

fn main() {
    let input = include_str!("input.txt");
    println!(
        "part1: {:?}",
        part1(input, 200000000000000.0, 400000000000000.0)
    );
    println!("part2: {:?}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("test_input.txt");
        assert_eq!(part1(input, 7.0, 27.0), 2);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("test_input.txt");
        assert_eq!(part2(input), 47);
    }
}
