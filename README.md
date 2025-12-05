# advent-of-code

collection of my advent of code solutions and trys

## New Day:

> requires the value of the `session=` cookie in the `.cookie` file at the root of this project
> the `aoc` crate will automatically determine the next day and download

```shell
cargo run aoc
# aoc: found day 2, creating new day 3
# aoc: reading cookie from .cookie
# aoc: downloading puzzle input from "https://adventofcode.com/2024/day/3/input" into "./2024/input/day3.txt"
```

## Testing

```
cd <year>
cargo test <day>

cd 2024
cargo test day3
```
