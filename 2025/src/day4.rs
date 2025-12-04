use aoc::point::{self, Point};

fn part_1(lines: Vec<String>) -> usize {
    let grid = aoc::grid::Grid::from_vec(lines);
    let mut count = 0;

    for y in 0..grid.height {
        for x in 0..grid.width {
            let p = Point::new(x, y);

            if grid.get(p.x, p.y) != Some(b'@') {
                continue;
            }

            let neighbors = point::DIAGONAL
                .iter()
                .filter(|&&dir| grid.get((p + dir).x, (p + dir).y) == Some(b'@'))
                .count();

            if neighbors < 4 {
                count += 1;
            }
        }
    }

    count
}

fn part_2(lines: Vec<String>) -> usize {
    let mut grid = aoc::grid::Grid::from_vec(lines);
    let mut removed = 0;

    loop {
        let prev = removed;
        for y in 0..grid.height {
            for x in 0..grid.width {
                let p = Point::new(x, y);

                if grid.get(p.x, p.y) != Some(b'@') {
                    continue;
                }

                let neighbors = point::DIAGONAL
                    .iter()
                    .filter(|&&dir| grid.get((p + dir).x, (p + dir).y) == Some(b'@'))
                    .count();

                if neighbors < 4 {
                    grid[p] = b'.';
                    removed += 1;
                }
            }
        }
        if prev == removed {
            break;
        }
    }

    removed
}

#[cfg(test)]
mod day3 {
    use crate::day4::{part_1, part_2};

    #[test]
    fn test_part_1() {
        let expected = 13;
        let input = r"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

        assert_eq!(part_1(aoc::lines_str(input)), expected);
    }

    #[test]
    fn test_part_1_real() {
        let expected = 13;
        assert_eq!(part_1(aoc::lines_file("./input/day4.txt")), expected);
    }

    #[test]
    fn test_part_2() {
        let expected = 43;
        let input = r"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

        assert_eq!(part_2(aoc::lines_str(input)), expected);
    }

    #[test]
    fn test_part_2_real() {
        let expected = 43;
        assert_eq!(part_2(aoc::lines_file("./input/day4.txt")), expected);
    }
}
