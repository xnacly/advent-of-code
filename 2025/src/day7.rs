use aoc::{
    grid,
    point::{self, Point},
};

fn part1(lines: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    let mut grid = grid::Grid::from_vec(&lines);
    let mut count = 0;
    for i in 0..grid.height {
        for j in 0..grid.width {
            let p = Point::new(j, i);
            let cur = grid[p];
            if let Some(prev) = grid.get_point(p + point::UP) {
                match prev {
                    b'S' => grid[p] = b'|',
                    b'|' if b'^' == cur => {
                        grid[p + point::LEFT] = b'|';
                        grid[p + point::RIGHT] = b'|';
                        count += 1;
                    }
                    b'|' => grid[p] = b'|',
                    _ => (),
                }
            }
        }
    }
    dbg!(grid);
    Ok(count)
}

fn part2(lines: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    Ok(0)
}

#[cfg(test)]
mod day7 {
    use crate::day7::{part1, part2};

    #[test]
    fn test_part1() {
        let expected = 21;
        let input = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(part1(aoc::lines_str(input)).unwrap(), expected);
    }

    // #[test]
    // fn test_part1_real() {
    //     dbg!(part1(aoc::lines_file("./input/day7.txt")).unwrap());
    // }

    //     #[test]
    //     fn test_part2() {
    //      let expected = 0;
    //      let input = "";
    //      assert_eq!(part1(aoc::lines_str(input)).unwrap(), expected);
    //     }

    //     #[test]
    //     fn test_part2_real() {
    //         dbg!(part2(aoc::lines_file("./input/day7.txt")).unwrap());
    //     }
}
