use std::{collections::HashMap, mem::swap};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_straight(self: &Self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }
}

impl IntoIterator for Line {
    type Item = Point;
    type IntoIter = LineIterator;
    fn into_iter(self) -> Self::IntoIter {
        if self.is_straight() {
            LineIterator {
                line: self,
                cur: self.start,
            }
        } else {
            LineIterator {
                line: self,
                cur: self.end,
            }
        }
    }
}
struct LineIterator {
    line: Line,
    cur: Point,
}

impl Iterator for LineIterator {
    type Item = Point;
    fn next(&mut self) -> Option<Point> {
        if self.cur <= self.line.end {
            let res = self.cur;
            if self.cur.x < self.line.end.x {
                self.cur.x += 1;
                Some(res)
            } else {
                self.cur.y += 1;
                Some(res)
            }
        } else {
            None
        }
    }
}

fn main() {
    let straight_lines: Vec<Line> = include_str!("input.txt")
        .lines()
        .map(parse_line)
        .filter(Line::is_straight)
        .collect();
    let mut cnt: HashMap<Point, usize> = HashMap::new();
    for line in straight_lines {
        for point in line {
            match cnt.get_mut(&point) {
                Some(x) => *x = *x + 1,
                None => {
                    let _ = cnt.insert(point, 1);
                }
            }
        }
    }
    let mut ans = 0;
    for (_, x) in cnt {
        if x > 1 {
            ans += 1;
        }
    }
    println!("ans: {}", ans);
}

fn parse_line(line: &str) -> Line {
    let mut split = line.split(" -> ");
    let mut start_split = split.next().unwrap().split(",");
    let start = Point {
        x: start_split.next().unwrap().parse().unwrap(),
        y: start_split.next().unwrap().parse().unwrap(),
    };
    let mut end_split = split.next().unwrap().split(",");
    let end = Point {
        x: end_split.next().unwrap().parse().unwrap(),
        y: end_split.next().unwrap().parse().unwrap(),
    };
    let mut res = Line { start, end };
    if res.is_straight() && res.start > res.end {
        swap(&mut res.start, &mut res.end);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_line() {
        let str = "0,9 -> 5,9";
        let line = parse_line(str);
        assert_eq!(line.start.x, 0);
        assert_eq!(line.start.y, 9);
        assert_eq!(line.end.x, 5);
        assert_eq!(line.end.y, 9);

        let str = "5,9 -> 0,9";
        let line = parse_line(str);
        assert_eq!(line.start.x, 0);
        assert_eq!(line.start.y, 9);
        assert_eq!(line.end.x, 5);
        assert_eq!(line.end.y, 9);
    }

    #[test]
    fn test_is_straight() {
        let line = Line {
            start: Point { x: 0, y: 9 },
            end: Point { x: 5, y: 9 },
        };
        assert_eq!(line.is_straight(), true);

        let line = Line {
            start: Point { x: 0, y: 8 },
            end: Point { x: 0, y: 9 },
        };
        assert_eq!(line.is_straight(), true);

        let line = Line {
            start: Point { x: 0, y: 9 },
            end: Point { x: 5, y: 2 },
        };
        assert_eq!(line.is_straight(), false);
    }

    #[test]
    fn test_line_iter() {
        let line = Line {
            start: Point { x: 0, y: 9 },
            end: Point { x: 3, y: 9 },
        };
        let mut iter = line.into_iter();
        assert_eq!(iter.next(), Some(Point { x: 0, y: 9 }));
        assert_eq!(iter.next(), Some(Point { x: 1, y: 9 }));
        assert_eq!(iter.next(), Some(Point { x: 2, y: 9 }));
        assert_eq!(iter.next(), Some(Point { x: 3, y: 9 }));
        assert_eq!(iter.next(), None);
    }
}
