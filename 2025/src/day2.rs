fn prep(line: &str) -> Result<Vec<(usize, usize)>, Box<dyn std::error::Error>> {
    line.split(",")
        .map(|r| {
            let (start, end) = r.split_once('-').ok_or("Invalid ID range input")?;
            Ok((start.parse::<usize>()?, end.parse::<usize>()?))
        })
        .collect()
}

fn part_1(lines: &[String]) -> Result<usize, Box<dyn std::error::Error>> {
    let mut sum = 0;
    for (lower, upper) in prep(lines.first().unwrap())? {
        for x in lower..=upper {
            let x_str = x.to_string();
            if x_str.len() % 2 != 0 {
                continue;
            }
            let (lhs, rhs) = x_str.split_at(x_str.len() / 2);
            if lhs == rhs {
                sum += x;
            }
        }
    }
    Ok(sum)
}

fn is_invalid(s: &str) -> bool {
    let bytes = s.as_bytes();
    let n = bytes.len();

    for k in 1..=n / 2 {
        if n % k != 0 {
            continue;
        }

        if bytes.chunks(k).all(|chunk| chunk == &bytes[0..k]) {
            return true;
        }
    }

    false
}

fn part_2(lines: &[String]) -> Result<usize, Box<dyn std::error::Error>> {
    let mut sum = 0;
    for (lower, upper) in prep(lines.first().unwrap())? {
        for x in lower..=upper {
            if is_invalid(&x.to_string()) {
                sum += x;
            }
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod day1 {
    use crate::day2::{part_1, part_2};

    #[test]
    fn test_part_2() {
        let expected = 4174379265;
        let input = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part_2(&aoc::lines_str(input)).unwrap(), expected);
    }

    #[test]
    fn test_part_2_real() {
        let expected = 6;
        assert_eq!(
            part_2(&aoc::lines_file("./input/day2.txt")).unwrap(),
            expected
        );
    }

    #[test]
    fn test_part_1() {
        let expected = 1227775554;
        let input = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part_1(&aoc::lines_str(input)).unwrap(), expected);
    }

    #[test]
    fn test_part_1_real() {
        let expected = 1227775554;
        assert_eq!(
            part_1(&aoc::lines_file("./input/day2.txt")).unwrap(),
            expected
        );
    }
}
