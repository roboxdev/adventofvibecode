use reqwest::Client;
use scraper::{Html, Selector};
use std::env;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <day-spec>", args[0]);
        eprintln!("Examples:");
        eprintln!("  {} 5          # Day 5 of current year", args[0]);
        eprintln!("  {} 2024-15    # Day 15 of 2024", args[0]);
        std::process::exit(1);
    }

    let (year, day) = parse_day_spec(&args[1])?;

    // Load session token from .env
    dotenv::dotenv().ok();
    let session_token = env::var("SESSION_TOKEN")
        .expect("SESSION_TOKEN not found in .env file");

    let client = Client::new();

    // Determine if we're fetching puzzle1 or puzzle2
    let puzzle_dir = format!("aoc{}/{}", year, day);
    fs::create_dir_all(&puzzle_dir)?;

    let answer1_path = Path::new(&puzzle_dir).join("answer1.md");
    let puzzle_num = if answer1_path.exists() { 2 } else { 1 };

    // Fetch puzzle page
    println!("Fetching puzzle {} for {}/day {}...", puzzle_num, year, day);
    let puzzle_html = fetch_puzzle(&client, year, day, &session_token).await?;
    let main_content = extract_main_content(&puzzle_html)?;

    // Save puzzle
    let puzzle_path = Path::new(&puzzle_dir).join(format!("puzzle{}.html", puzzle_num));
    fs::write(&puzzle_path, main_content)?;
    println!("✓ Puzzle {} saved to {}", puzzle_num, puzzle_path.display());

    // Fetch and save input only if it's puzzle 1
    if puzzle_num == 1 {
        println!("Fetching input for {}/day {}...", year, day);
        let input = fetch_input(&client, year, day, &session_token).await?;
        let input_path = Path::new(&puzzle_dir).join("input.md");
        fs::write(&input_path, input)?;
        println!("✓ Input saved to {}", input_path.display());
    }

    Ok(())
}

fn parse_day_spec(spec: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    if spec.contains('-') {
        let parts: Vec<&str> = spec.split('-').collect();
        if parts.len() != 2 {
            return Err("Invalid format. Use YYYY-DD (e.g., 2024-15)".into());
        }
        let year: u32 = parts[0].parse()?;
        let day: u32 = parts[1].parse()?;
        if day < 1 || day > 25 {
            return Err("Day must be between 1 and 25".into());
        }
        Ok((year, day))
    } else {
        let day: u32 = spec.parse()?;
        if day < 1 || day > 25 {
            return Err("Day must be between 1 and 25".into());
        }
        // Get current year using std::time
        let current_year = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| {
                let seconds = d.as_secs();
                // Rough calculation: 2024 started at ~1704067200 seconds
                let years_since_1970 = (seconds - 1704067200) / (365 * 24 * 3600) + 2024;
                if years_since_1970 < 2024 {
                    2024
                } else {
                    years_since_1970 as u32
                }
            })
            .unwrap_or(2025);
        Ok((current_year, day))
    }
}

async fn fetch_puzzle(
    client: &Client,
    year: u32,
    day: u32,
    session_token: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://adventofcode.com/{}/day/{}", year, day);
    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", session_token))
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}

async fn fetch_input(
    client: &Client,
    year: u32,
    day: u32,
    session_token: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", session_token))
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}

fn extract_main_content(html: &str) -> Result<String, Box<dyn std::error::Error>> {
    let document = Html::parse_document(html);

    // Find the main tag
    let main_selector = Selector::parse("main").unwrap();

    if let Some(main_element) = document.select(&mut main_selector.clone()).next() {
        Ok(main_element.inner_html())
    } else {
        Err("Could not find <main> element".into())
    }
}
