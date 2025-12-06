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

fn part2(lines: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    Err("i give up".into())
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
        let input = r"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
        assert_eq!(part2(aoc::lines_str(input)).unwrap(), expected);
    }

    // #[test]
    // fn test_part2_real() {
    //     dbg!(part2(aoc::lines_file("./input/day6.txt")).unwrap());
    // }
}
