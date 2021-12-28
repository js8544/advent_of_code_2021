use itertools::Itertools;

struct HeightMap {
    data: Vec<Vec<u8>>,
}

impl HeightMap {
    fn get_low_points(self: &Self) -> Vec<(usize, usize, u8)> {
        let directions = [[1, 0], [0, 1], [-1, 0], [0, -1]];
        if self.data.len() == 0 {
            vec![]
        } else {
            let n = self.data.len();
            let m = self.data[0].len();
            (0..n)
                .cartesian_product(0..m)
                .filter(|&(i, j)| {
                    directions.iter().all(|dir| {
                        let new_i = i as i64 + dir[0];
                        let new_j = j as i64 + dir[1];
                        new_i < 0
                            || new_i >= n as i64
                            || new_j < 0
                            || new_j >= m as i64
                            || self.data[i][j] < self.data[new_i as usize][new_j as usize]
                    })
                })
                .map(|(i, j)| (i, j, self.data[i][j]))
                .collect()
        }
    }
}
fn main() {
    let data: Vec<Vec<u8>> = include_str!("input.txt")
        .lines()
        .map(|line| line.bytes().map(|num| num - b'0').collect())
        .collect();
    let height_map = HeightMap { data };
    let low_points = height_map.get_low_points();
    let ans = low_points.iter().fold(0, |acc, cur| acc + 1 + cur.2 as u64);
    println!("ans: {}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_low_points() {
        let height_map = HeightMap {
            data: vec![vec![1, 2, 3], vec![3, 4, 5], vec![5, 6, 7]],
        };
        assert_eq!(height_map.get_low_points(), vec![(0, 0, 1)]);

        let height_map = HeightMap {
            data: vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 0]],
        };
        assert_eq!(
            height_map.get_low_points(),
            vec![(0, 0, 0), (0, 2, 0), (1, 1, 0), (2, 0, 0), (2, 2, 0)]
        );
    }
}
