fn part_1(lines: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    let mut separator = 0;
    let mut ranges: Vec<(usize, usize)> = vec![];

    for i in 0..lines.len() {
        let line = &lines[i];
        if line.is_empty() {
            separator = i;
            break;
        }

        let Some((lhs, rhs)) = line.split_once('-') else {
            break;
        };

        ranges.push((lhs.parse()?, rhs.parse()?));
    }

    Ok(lines[separator..lines.len()]
        .iter()
        .filter_map(|d| d.parse::<usize>().ok())
        .filter(|d| ranges.iter().any(|r| d >= &r.0 && d <= &r.1))
        .count())
}

fn part_2(lines: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    let mut ranges: Vec<(usize, usize)> = vec![];

    for i in 0..lines.len() {
        let line = &lines[i];
        if line.is_empty() {
            break;
        }

        let Some((lhs, rhs)) = line.split_once('-') else {
            break;
        };

        ranges.push((lhs.parse()?, rhs.parse()?));
    }

    ranges.sort_unstable_by_key(|x| x.0);

    let mut merged = Vec::new();
    let mut current = ranges[0];
    for &(start, end) in &ranges[1..] {
        if start <= current.1 + 1 {
            current.1 = current.1.max(end);
        } else {
            merged.push(current);
            current = (start, end);
        }
    }

    merged.push(current);

    Ok(merged.into_iter().map(|(l, r)| r - l + 1).sum())
}

// good idea, but input is too large:
// fn part_2(lines: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
//     let mut set: HashSet<usize> = HashSet::new();
//     for i in 0..lines.len() {
//         let line = &lines[i];
//         if line.is_empty() {
//             break;
//         }

//         let Some((lhs, rhs)) = line.split_once('-') else {
//             break;
//         };

//         let (lower, upper) = (lhs.parse::<usize>()?, rhs.parse::<usize>()?);
//         set.extend(lower..=upper)
//     }

//     Ok(set.len())
// }

#[cfg(test)]
mod day5 {
    use crate::day5::{part_1, part_2};
    #[test]
    fn test_part_1() {
        let expected = 3;
        let input = r"3-5
    10-14
    16-20
    12-18

    1
    5
    8
    11
    17
    32
    ";

        assert_eq!(part_1(aoc::lines_str(input)).unwrap(), expected);
    }

    #[test]
    fn test_part_1_real() {
        let expected = 3;
        assert_eq!(
            part_1(aoc::lines_file("./input/day5.txt")).unwrap(),
            expected
        );
    }

    #[test]
    fn test_part_2() {
        let expected = 14;
        let input = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

        assert_eq!(part_2(aoc::lines_str(input)).unwrap(), expected);
    }

    #[test]
    fn test_part_2_real() {
        let expected = 3;
        assert_eq!(
            part_2(aoc::lines_file("./input/day5.txt")).unwrap(),
            expected
        );
    }
}
