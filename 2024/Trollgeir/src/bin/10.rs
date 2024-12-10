advent_of_code::solution!(10);

struct TrailClimber {
    current_position: (usize, usize),
    current_height: u8,
}

impl TrailClimber {
    fn traverse_uphill(&mut self, map: &TrailMap) -> Vec<(usize, usize)> {
        if self.current_height == 9 {
            return vec![self.current_position];
        }

        let mut ascending_candidates: Vec<(usize, usize)> = Vec::with_capacity(3);

        let (x, y) = self.current_position;
        for direction in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (dx, dy) = direction;
            let new_position = ((x as i32 + dx) as usize, (y as i32 + dy) as usize);
            if map.get_height(new_position) == Some(self.current_height + 1) {
                ascending_candidates.push(new_position);
            }
        }
        match ascending_candidates.len() {
            0 => vec![],
            1 => {
                self.current_position = ascending_candidates[0];
                self.current_height = map.get_height(self.current_position).unwrap();
                self.traverse_uphill(map)
            }
            _ => ascending_candidates
                .iter()
                .flat_map(|candidate| {
                    let mut branched_climber = TrailClimber {
                        current_position: *candidate,
                        current_height: map.get_height(*candidate).unwrap(),
                    };
                    branched_climber.traverse_uphill(map)
                })
                .collect(),
        }
    }
}

struct TrailMap {
    map: Vec<Vec<u8>>,
}

impl TrailMap {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<u8>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();

        Self { map }
    }

    fn spawn_trailcrawlers(&self) -> Vec<TrailClimber> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row.iter().enumerate().filter_map(move |(y, &cell)| {
                    (cell == 0).then_some(TrailClimber {
                        current_position: (x, y),
                        current_height: 0,
                    })
                })
            })
            .collect()
    }

    fn get_height(&self, position: (usize, usize)) -> Option<u8> {
        self.map
            .get(position.0)
            .and_then(|row| row.get(position.1))
            .copied()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = TrailMap::new(input);
    let mut crawlers = map.spawn_trailcrawlers();
    let result = crawlers
        .iter_mut()
        .map(|crawler| {
            let mut peaks = crawler.traverse_uphill(&map);
            peaks.sort_unstable();
            peaks.dedup();
            peaks.len() as u32
        })
        .sum::<u32>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = TrailMap::new(input);
    let mut crawlers = map.spawn_trailcrawlers();
    let result = crawlers
        .iter_mut()
        .map(|crawler| crawler.traverse_uphill(&map).len())
        .sum::<usize>() as u32;
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
