use aoc::grid::Grid;

fn check(grid: &Grid) -> usize {
    0
}

fn part1(lines: Vec<String>) -> usize {
    let grid = Grid::from_vec(&lines);
    dbg!(grid
        .rows()
        .into_iter()
        .map(|x| String::from_utf8(x).unwrap())
        .collect::<Vec<String>>());
    check(&grid)
}

fn part2(lines: Vec<String>) -> usize {
    0
}

#[cfg(test)]
mod day4 {
    use crate::day4::{part1, part2};
    #[test]
    fn test_part1() {
        let t = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
        let r = part1(aoc::lines_str(t));
        assert_eq!(r, 18);
    }

    // #[test]
    // fn test_part1_real() {
    //     dbg!(part1(aoc::lines_file("./input/day4.txt")));
    // }

    // #[test]
    // fn test_part2() {
    //     let t = r"";
    //     let r = part2(aoc::lines_str(t));
    //     assert_eq!(r, 0);
    // }

    // #[test]
    // fn test_part2_real() {
    //     dbg!(part2(aoc::lines_file("./input/day4.txt")));
    // }
}
