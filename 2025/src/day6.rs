use aoc::grid;

fn part1(lines: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    let mut columns: Vec<Vec<usize>> = vec![];

    for (i, line) in lines[..lines.len() - 1].iter().enumerate() {
        if i == 0 {
            let count = line.split_whitespace().count();
            columns = vec![Vec::new(); count];
        }

        for (col, part) in line.split_whitespace().enumerate() {
            columns[col].push(part.parse::<usize>()?);
        }
    }

    let mut sum = 0;
    for (i, op) in lines.last().unwrap().split_whitespace().enumerate() {
        match op {
            "+" => sum += columns[i].iter().sum::<usize>(),
            "*" => sum += columns[i].iter().product::<usize>(),
            _ => unreachable!(),
        };
    }
    Ok(sum)
}

fn part2(lines: Vec<String>) -> Option<usize> {
    let mut grid = grid::Grid::from_vec(&lines);
    grid.transpose();

    let mut op = None;
    let mut nums = vec![];
    let mut sum = 0;
    for mut row in grid.rows() {
        if row.iter().all(u8::is_ascii_whitespace) {
            match op {
                Some(b'+') => sum += nums.iter().sum::<usize>(),
                Some(b'*') => sum += nums.iter().product::<usize>(),
                _ => (),
            }
            op = None;
            nums.clear();
            continue;
        }

        let last = row.last().unwrap();
        if matches!(last, b'+' | b'*') {
            op = Some(*last);
            row.pop();
        }

        let mut num: usize = 0;
        for digit in row {
            if digit.is_ascii_digit() {
                num = num * 10 + (digit - b'0') as usize
            }
        }
        nums.push(num);
    }

    if !nums.is_empty() {
        match op {
            Some(b'+') => sum += nums.iter().sum::<usize>(),
            Some(b'*') => sum += nums.iter().product::<usize>(),
            _ => (),
        }
    }

    Some(sum)
}

#[cfg(test)]
mod day6 {
    use crate::day6::{part1, part2};

    #[test]
    fn test_part1() {
        let expected = 4277556;
        let input = r"123 328  51 64
    45 64  387 23
    6 98  215 314
    *   +   *   +
    ";
        assert_eq!(part1(aoc::lines_str(input)).unwrap(), expected);
    }

    #[test]
    fn test_part1_real() {
        dbg!(part1(aoc::lines_file("./input/day6.txt")).unwrap());
    }

    #[test]
    fn test_part2() {
        let expected = 3263827;
        let input = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
        assert_eq!(part2(aoc::lines_str(input)).unwrap(), expected);
    }

    #[test]
    fn test_part2_real() {
        dbg!(part2(aoc::lines_file("./input/day6.txt")).unwrap());
    }
}
