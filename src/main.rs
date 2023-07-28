use reqwest;
use rss;
use std::io::{self, Write};
use termion::{color, style};
use tokio::runtime::Runtime;

fn main() {
    // Specify the URL of the RSS feed
    let url = "https://www.nasa.gov/rss/dyn/hurricaneupdate.rss";

    // Create a Tokio runtime
    let rt = Runtime::new().expect("Failed to create Tokio runtime");

    // Run the program within the Tokio runtime
    rt.block_on(async {
        // Fetch the RSS feed
        let response = reqwest::get(url)
            .await
            .expect("Failed to fetch RSS feed");

        // Read the response body as bytes
        let body = response
            .bytes()
            .await
            .expect("Failed to read response body");

        // Parse the RSS feed
        let body_str = String::from_utf8_lossy(&body);
        let channel = rss::Channel::read_from(body_str.as_bytes())
            .expect("Failed to parse RSS feed");

        // Format and display the feed title and description
        println!("{}{}Title: {}{}",
            style::Bold,
            color::Fg(color::Yellow),
            channel.title(),
            style::Reset
        );
        println!("{}Description: {}{}", style::Bold, channel.description(), style::Reset);
        println!();

        // Display the feed items
        println!("{}Items:{}", style::Bold, style::Reset);

        for item in channel.items() {
            println!("{}{}Title: {}{}",
                color::Fg(color::Green),
                style::Bold,
                item.title().unwrap_or("Untitled"),
                style::Reset
            );
            println!("{}Link: {}{}", color::Fg(color::Blue), item.link().unwrap_or("No link"), style::Reset);
            println!("{}Description: {}{}", color::Fg(color::Cyan), item.description().unwrap_or("No description"), style::Reset);
            println!();
        }
    });

    // Flush stdout to ensure all content is displayed
    io::stdout().flush().unwrap();
}
