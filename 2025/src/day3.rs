fn chars_to_digit(chars: &[u8]) -> usize {
    chars.iter().fold(0, |r, &c| r * 10 + (c - b'0') as usize)
}

fn part_1(lines: &[String]) -> usize {
    let mut sum = 0;
    for l in lines.iter().filter(|l| l.len() != 0) {
        let chars = l.as_bytes();
        let mut max = 0;
        for i in 0..chars.len() {
            for j in (i + 1)..chars.len() {
                let jolt = chars_to_digit(&[chars[i], chars[j]]);
                if jolt > max {
                    max = jolt;
                }
            }
        }

        sum += max;
    }
    sum
}

fn max_joltage(digits: &[u8], k: usize) -> usize {
    let mut result = Vec::with_capacity(k);
    let mut start = 0;
    let n = digits.len();

    for remaining in (0..k).rev() {
        let end = n - remaining;
        let mut max_idx = start;
        for i in start..end {
            if digits[i] > digits[max_idx] {
                max_idx = i;
            }
        }
        result.push(digits[max_idx]);
        start = max_idx + 1;
    }

    result.iter().fold(0, |r, &c| r * 10 + (c - b'0') as usize)
}

fn part_2(lines: &[String]) -> usize {
    lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| max_joltage(l.as_bytes(), 12))
        .sum()
}

#[cfg(test)]
mod day3 {
    use crate::day3::{part_1, part_2};

    #[test]
    fn test_part_1() {
        let expected = 357;
        let input = r"
987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(part_1(&aoc::lines_str(input)), expected);
    }

    #[test]
    fn test_part_1_real() {
        let expected = 357;
        assert_eq!(part_1(&aoc::lines_file("./input/day3.txt")), expected);
    }

    #[test]
    fn test_part_2() {
        let expected = 3121910778619;
        let input = r"
987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(part_2(&aoc::lines_str(input)), expected);
    }

    #[test]
    fn test_part_2_real() {
        let expected = 3121910778619;
        assert_eq!(part_2(&aoc::lines_file("./input/day3.txt")), expected);
    }
}
