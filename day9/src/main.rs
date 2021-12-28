use std::cmp::Ordering;

use itertools::Itertools;

struct HeightMap {
    data: Vec<Vec<u8>>,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct LowPoint {
    i: usize,
    j: usize,
    height: u8,
}

impl HeightMap {
    fn get_low_points(self: &Self) -> Vec<LowPoint> {
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
                .map(|(i, j)| LowPoint {
                    i,
                    j,
                    height: self.data[i][j],
                })
                .collect()
        }
    }

    fn dfs(self: &Self, i: usize, j: usize, visited: &mut Vec<Vec<bool>>) -> u64 {
        let directions = [[1, 0], [0, 1], [-1, 0], [0, -1]];
        let n = self.data.len();
        let m = self.data[0].len();
        if visited[i][j] {
            return 0;
        }
        visited[i][j] = true;
        directions
            .iter()
            .map(|dir| (i as i64 + dir[0], j as i64 + dir[1]))
            .filter(|&(new_i, new_j)| {
                new_i >= 0
                    && new_i < n as i64
                    && new_j >= 0
                    && new_j < m as i64
                    && self.data[new_i as usize][new_j as usize] != 9
            })
            .map(|(new_i, new_j)| {
                let new_i = new_i as usize;
                let new_j = new_j as usize;
                self.dfs(new_i, new_j, visited)
            })
            .sum::<u64>()
            + 1
    }
}

fn main() {
    let data: Vec<Vec<u8>> = include_str!("input.txt")
        .lines()
        .map(|line| line.bytes().map(|num| num - b'0').collect())
        .collect();
    let n = data.len();
    let m = data[0].len();
    let height_map = HeightMap { data };
    let low_points = height_map.get_low_points();
    let mut visited = vec![vec![false; n]; m];
    let ans: u64 = low_points
        .iter()
        .map(|p| height_map.dfs(p.i, p.j, &mut visited))
        .sorted()
        .map(|e| {
            println!("{}", e);
            e
        })
        .rev()
        .take(3)
        .product();
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
        assert_eq!(
            height_map.get_low_points(),
            vec![LowPoint {
                i: 0,
                j: 0,
                height: 1
            }]
        );

        let height_map = HeightMap {
            data: vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 0]],
        };
        assert_eq!(
            height_map.get_low_points(),
            vec![
                LowPoint {
                    i: 0,
                    j: 0,
                    height: 0
                },
                LowPoint {
                    i: 0,
                    j: 2,
                    height: 0
                },
                LowPoint {
                    i: 1,
                    j: 1,
                    height: 0
                },
                LowPoint {
                    i: 2,
                    j: 0,
                    height: 0
                },
                LowPoint {
                    i: 2,
                    j: 2,
                    height: 0
                }
            ]
        );
    }

    #[test]
    fn test_dfs() {
        let height_map = HeightMap {
            data: vec![vec![1, 2, 3], vec![3, 4, 5], vec![5, 6, 7]],
        };
        let mut visited = vec![vec![false; 3]; 3];
        let dfs_res: Vec<u64> = height_map
            .get_low_points()
            .iter()
            .map(|p| height_map.dfs(p.i, p.j, &mut visited))
            .collect();
        assert_eq!(dfs_res, vec![9]);
    }
}
