fn main() {
    let input = include_str!("input.txt");
    part1(input);
    part2(input);
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct SlotInfo {
    num: u64,
    marked: bool,
}

type Board = [[SlotInfo; 5]; 5];

impl SlotInfo {
    fn new(num: u64) -> Self {
        SlotInfo {
            num: num,
            marked: false,
        }
    }
}

fn part1(input: &str) {
    let mut blocks = input.split("\n\n");
    let order: Vec<u64> = blocks
        .next()
        .unwrap()
        .split(',')
        .map(|str| str.parse().unwrap())
        .collect();

    println!("{:?}", order);
    let mut boards: Vec<Board> = blocks.map(parse_block).collect();
    for num in order {
        for board in &mut boards {
            (0..5).for_each(|row| {
                (0..5).for_each(|col| {
                    if board[row][col].num == num {
                        board[row][col].marked = true;
                    }
                });
            });
            if check_board(board) {
                let sum = board.iter().fold(0, |accu, arr| {
                    arr.iter().fold(accu, |accu, cur| {
                        accu + if !cur.marked { cur.num } else { 0 }
                    })
                });
                println!("sum: {}, last: {}, ans: {}", sum, num, sum * num);
                return;
            }
        }
    }
    return;
}

fn part2(input: &str) {
    let mut blocks = input.split("\n\n");
    let order: Vec<u64> = blocks
        .next()
        .unwrap()
        .split(',')
        .map(|str| str.parse().unwrap())
        .collect();

    println!("{:?}", order);
    let mut boards: Vec<Board> = blocks.map(parse_block).collect();
    for num in order {
        // println!("round: {}", num);
        for board in &mut boards {
            if check_board(board) {
                continue;
            }
            (0..5).for_each(|row| {
                (0..5).for_each(|col| {
                    if board[row][col].num == num {
                        board[row][col].marked = true;
                    }
                });
            });
            if check_board(board) {
                let sum = board.iter().fold(0, |accu, arr| {
                    arr.iter().fold(accu, |accu, cur| {
                        accu + if !cur.marked { cur.num } else { 0 }
                    })
                });
                println!("sum: {}, last: {}, ans: {}", sum, num, sum * num);
            }
        }
    }
    return;
}

fn parse_block(block: &str) -> Board {
    let mut board: Board = [[SlotInfo::new(0); 5]; 5];

    block.lines().enumerate().for_each(|(i, line)| {
        line.split_whitespace().enumerate().for_each(|(j, num)| {
            // println!("i: {}, j: {}, num: {}", i, j, num);
            board[i][j].num = num.parse().unwrap();
        });
    });

    board
}

fn check_board(board: &Board) -> bool {
    (0..5).any(|row| (0..5).all(|col| board[row][col].marked))
        || (0..5).any(|col| (0..5).all(|row| board[row][col].marked))
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_parse_block() {
        let block_str = "1  2  3  4  5
2  3  4  5  6
3  4  5  6  7
10 11 12 12 1
88 77 99 22 33";
        println!("{}", block_str);
        let board = parse_block(block_str);
        assert_eq!(board[0][0], SlotInfo::new(1));
        assert_eq!(board[0][1], SlotInfo::new(2));
        assert_eq!(board[0][2], SlotInfo::new(3));
        assert_eq!(board[0][3], SlotInfo::new(4));
        assert_eq!(board[0][4], SlotInfo::new(5));
        assert_eq!(board[4][0], SlotInfo::new(88));
        assert_eq!(board[4][1], SlotInfo::new(77));
        assert_eq!(board[4][2], SlotInfo::new(99));
        assert_eq!(board[4][3], SlotInfo::new(22));
        assert_eq!(board[4][4], SlotInfo::new(33));
    }

    #[test]
    fn test_check_board_row() {
        let mut board = [[SlotInfo::new(0); 5]; 5];
        assert_eq!(check_board(&board), false);

        (0..5).for_each(|col| {
            board[0][col].marked = true;
        });
        assert_eq!(check_board(&board), true);
    }

    #[test]
    fn test_check_board_col() {
        let mut board = [[SlotInfo::new(0); 5]; 5];
        assert_eq!(check_board(&board), false);

        (0..5).for_each(|row| {
            board[row][2].marked = true;
        });
        assert_eq!(check_board(&board), true);
    }
}
