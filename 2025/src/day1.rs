const START: i64 = 50;
const SIZE: i64 = 100; // 0-99

#[derive(Debug)]
struct Knob {
    pos: i64,
    inner_zero_passes: usize,
}

impl Knob {
    /// python style modulus, returns absolute value for things like -18 % 100, which results in 82
    /// and not in -18, as rust defines mod
    fn m(a: i64, b: i64) -> i64 {
        ((a % b) + b) % b
    }

    fn left(&mut self, step: usize) {
        let step = step as i64;
        self.inner_zero_passes += if self.pos == 0 {
            (step / SIZE) as usize
        } else if step < self.pos {
            0
        } else {
            1 + ((step - self.pos) / SIZE) as usize
        };

        self.pos = Knob::m(self.pos - step as i64, SIZE);
    }

    fn right(&mut self, step: usize) {
        let step = step as i64;
        let dist = (SIZE - self.pos) % SIZE;
        self.inner_zero_passes += if dist == 0 {
            (step / SIZE) as usize
        } else if step < dist {
            0
        } else {
            1 + ((step - dist) / SIZE) as usize
        };
        self.pos = Knob::m(self.pos + step as i64, SIZE);
    }
}

fn part_1(lines: &[String]) -> Result<usize, Box<dyn std::error::Error>> {
    let mut k = Knob {
        pos: START,
        inner_zero_passes: 0,
    };
    let mut is0 = 0;
    for l in lines {
        match l.split_at(1) {
            ("L", digit) => k.left(digit.parse::<usize>()?),
            ("R", digit) => k.right(digit.parse::<usize>()?),
            _ => unreachable!(),
        }

        if k.pos == 0 {
            is0 += 1;
        }
    }

    Ok(is0)
}

fn part_2(lines: &[String]) -> Result<usize, Box<dyn std::error::Error>> {
    let mut k = Knob {
        pos: START,
        inner_zero_passes: 0,
    };

    for l in lines {
        match l.split_at(1) {
            ("L", digit) => k.left(digit.parse::<usize>()?),
            ("R", digit) => k.right(digit.parse::<usize>()?),
            _ => unreachable!(),
        }
    }

    Ok(k.inner_zero_passes)
}

#[cfg(test)]
mod day1 {
    use crate::day1::{part_1, part_2};

    #[test]
    fn test_part_2() {
        let expected = 6;
        let input = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(part_2(&aoc::lines_str(input)).unwrap(), expected);
    }

    #[test]
    fn test_part_2_real() {
        let expected = 6;
        assert_eq!(
            part_2(&aoc::lines_file("./input/day1.txt")).unwrap(),
            expected
        );
    }

    #[test]
    fn test_part_1() {
        let expected = 3;
        let input = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(part_1(&aoc::lines_str(input)).unwrap(), expected);
    }

    #[test]
    fn test_part_1_real() {
        let expected = 1120;
        assert_eq!(
            part_1(&aoc::lines_file("./input/day1.txt")).unwrap(),
            expected
        );
    }
}
