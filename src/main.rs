use anyhow::{Context, Result};
use clipboard::{ClipboardContext, ClipboardProvider};
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::env;

fn main() -> Result<()> {
    let args = parse_args()?;
    let content = fetch_website_content(&args.url)?;
    let selector = format!("span.lang-{}", args.language);
    let extracted_content = extract_content(&content, &selector)?;
    copy_to_clipboard(&extracted_content)?;
    println!("Copied!: {}", args.url);
    Ok(())
}

fn parse_args() -> Result<CommandLineArgs> {
    let args = env::args().skip(1).collect::<Vec<String>>(); // skip(1) to ignore the program name
    if args.is_empty() {
        anyhow::bail!("Usage: xatcoder <URL> [lang]");
    }
    let url = args[0].clone();
    let language = args.get(1).cloned().unwrap_or("ja".to_string()); // Use 'ja' as default if no language is provided
    if language != "ja" && language != "en" {
        anyhow::bail!("Language must be 'ja' or 'en'");
    }
    Ok(CommandLineArgs { url, language })
}

struct CommandLineArgs {
    url: String,
    language: String,
}

fn fetch_website_content(url: &str) -> Result<String> {
    let client = Client::new();
    let response = client
        .get(url)
        .send()
        .context("Failed to send request")?
        .text()
        .context("Failed to read response text")?;
    Ok(response)
}

fn extract_content(html: &str, selector_str: &str) -> Result<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse(selector_str)
        .map_err(|e| anyhow::anyhow!("Failed to parse selector: {:?}", e))?;

    document
        .select(&selector)
        .next()
        .map(|element| element.inner_html())
        .ok_or_else(|| anyhow::anyhow!("No elements found with the specified selector"))
}

fn copy_to_clipboard(content: &str) -> Result<()> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()
        .map_err(|e| anyhow::anyhow!("Failed to create clipboard context: {:?}", e))?;
    ctx.set_contents(content.to_string())
        .map_err(|e| anyhow::anyhow!("Failed to copy content to clipboard: {:?}", e))?;
    Ok(())
}
