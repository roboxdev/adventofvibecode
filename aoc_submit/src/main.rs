use reqwest::Client;
use std::env;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <day-spec> <level> <answer>", args[0]);
        eprintln!("Examples:");
        eprintln!("  {} 5 1 12345      # Day 5, level 1 of current year", args[0]);
        eprintln!("  {} 2024-15 2 6789 # Day 15 of 2024, level 2", args[0]);
        std::process::exit(1);
    }

    let (year, day) = parse_day_spec(&args[1])?;
    let level: u32 = args[2].parse()?;
    let answer = &args[3];

    // Load session token from .env
    dotenv::dotenv().ok();
    let session_token = env::var("SESSION_TOKEN")
        .expect("SESSION_TOKEN not found in .env file");

    // Submit answer
    submit_answer(&session_token, year, day, level, answer).await?;

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

async fn submit_answer(
    session_token: &str,
    year: u32,
    day: u32,
    level: u32,
    answer: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("https://adventofcode.com/{}/day/{}/answer", year, day);

    let params = [
        ("level", level.to_string()),
        ("answer", answer.to_string()),
    ];

    println!("Submitting answer: {} for {}/day {} level {}", answer, year, day, level);

    let response = client
        .post(&url)
        .header("Cookie", format!("session={}", session_token))
        .form(&params)
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;

    // Parse response to check if answer was correct
    if body.contains("That's the right answer!") {
        println!("✅ Correct answer!");
        save_answer(year, day, level, answer)?;
    } else if body.contains("That's not the right answer") {
        println!("❌ Wrong answer!");
        if body.contains("too high") {
            println!("   (answer is too high)");
        } else if body.contains("too low") {
            println!("   (answer is too low)");
        }
    } else if body.contains("You don't seem to be solving the right level.") {
        println!("⚠️  You've already solved this level!");
    } else if body.contains("Please wait") {
        println!("⏱️  Rate limited - please wait before submitting again");
    } else if status.as_u16() == 200 {
        println!("✅ Answer submitted successfully!");
        save_answer(year, day, level, answer)?;
    } else {
        println!("Status: {}", status);
    }

    Ok(())
}

fn save_answer(year: u32, day: u32, level: u32, answer: &str) -> Result<(), Box<dyn std::error::Error>> {
    let day_dir = format!("aoc{}/{}", year, day);
    fs::create_dir_all(&day_dir)?;

    let answer_file = format!("answer{}.md", level);
    let answer_path = Path::new(&day_dir).join(&answer_file);

    fs::write(&answer_path, format!("{}\n", answer))?;
    println!("✓ Answer saved to {}", answer_path.display());

    Ok(())
}
