use clap::Parser;
use std::{fs, io::Write, path::PathBuf};

const TEMPLATE: &str = r#"
fn part1(lines: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    Ok(0)
}

fn part2(lines: Vec<String>) -> Result<usize, Box<dyn std::error::Error>> {
    Ok(0)
}

#[cfg(test)]
mod day$DAYHERE$ {
    use crate::day$DAYHERE$::{part1, part2};

    #[test]
    fn test_part1() {
        let expected = 0;
        let input = "";
        assert_eq!(part1(aoc::lines_str(input)).unwrap(), expected);
    }

    #[test]
    fn test_part1_real() {
        dbg!(part1(aoc::lines_file("./input/day$DAYHERE$.txt")).unwrap());
    }

//     #[test]
//     fn test_part2() {
//      let expected = 0;
//      let input = "";
//      assert_eq!(part1(aoc::lines_str(input)).unwrap(), expected);
//     }

//     #[test]
//     fn test_part2_real() {
//         dbg!(part2(aoc::lines_file("./input/day$DAYHERE$.txt")).unwrap());
//     }
}
"#;

/// CLI for AoC day generation
#[derive(Parser)]
struct Config {
    #[arg(short, long, default_value = "2025")]
    year: String,
    #[arg(short, long, default_value = ".cookie")]
    cookie_file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Config::parse();

    let dir: PathBuf = [&args.year].iter().collect();
    let src_dir = dir.join("src");
    let input_dir = dir.join("input");

    println!("Ensuring directories exist...");
    fs::create_dir_all(&src_dir)?;
    fs::create_dir_all(&input_dir)?;
    println!("Source directory: {:?}", src_dir);
    println!("Input directory: {:?}", input_dir);

    let day = new_day(&src_dir).unwrap_or(1);
    println!("Next day to create: {}", day);

    println!("Reading session cookie from '{}'", &args.cookie_file);
    let cookie = fs::read_to_string(&args.cookie_file)?
        .lines()
        .next()
        .ok_or("failed to read cookie")?
        .to_string();

    let url = format!("https://adventofcode.com/{}/day/{}/input", &args.year, day);
    let dest_file = input_dir.join(format!("day{}.txt", day));
    println!("Downloading input from {} to {:?}", url, dest_file);
    download_input(&cookie, &url, &dest_file)?;
    println!("Input downloaded successfully.");

    println!("Creating Rust source file for day {}...", day);
    create_day(day, &src_dir)?;
    println!("Day {} source file created successfully.", day);

    Ok(())
}

fn new_day(dir: &PathBuf) -> Option<usize> {
    println!("Scanning {:?} for existing day files...", dir);
    let mut days = dir
        .read_dir()
        .ok()?
        .filter_map(|x| {
            x.ok()?
                .file_name()
                .to_str()?
                .split('.')
                .next()?
                .chars()
                .skip(3)
                .collect::<String>()
                .parse::<usize>()
                .ok()
        })
        .collect::<Vec<_>>();
    days.sort();
    println!("Found existing days: {:?}", days);
    days.last().map(|x| x + 1)
}

fn create_day(day: usize, dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = dir.join(format!("day{}.rs", day));
    println!("Writing template to {:?}", file_path);
    let mut f = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&file_path)?;
    f.write_all(TEMPLATE.replace("$DAYHERE$", &day.to_string()).as_bytes())?;

    let lib_path = dir.join("lib.rs");
    println!("Updating {:?}", lib_path);
    let mut lib_content = fs::read_to_string(&lib_path).unwrap_or_default();
    let mod_line = format!("mod day{};", day);
    if !lib_content.contains(&mod_line) {
        lib_content.push_str(&format!("{}{}", mod_line, "\n"));
        fs::write(&lib_path, lib_content)?;
        println!("Added '{}' to lib.rs", mod_line);
    } else {
        println!("lib.rs already contains '{}'", mod_line);
    }

    Ok(())
}

fn download_input(
    cookie: &str,
    url: &str,
    destination: &PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Sending request to {}", url);
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .header("cookie", format!("session={}", cookie))
        .send()?;
    fs::write(destination, res.bytes()?)?;
    println!("Saved input to {:?}", destination);
    Ok(())
}
