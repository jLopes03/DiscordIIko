//doing a get image and synopsis from user input first (a valid full web novel name)

use scraper::{Html, Selector};
use serde::Deserialize;
use serde_json;

// I clearly didn't need these structs but I wanted to try out serde_json and just maybe I may also want to do some stuff with this data later
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ArchiveInfo {
    url: String,
    archived_snapshots: ArchivedSnapshots,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ArchivedSnapshots {
    closest: ClosestSnapshot,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ClosestSnapshot {
    status: String,
    available: bool,
    url: String,
    timestamp: String,
}

// might switch blocking for Client
pub fn get_novel_data(novel_name: &str) -> String {
    let mut wayback_availabilty_url =
        "http://archive.org/wayback/available?url=https://www.novelupdates.com/series/".to_string();

    wayback_availabilty_url.push_str(
        &novel_name
            .replace(" ", "-")
            .replace("\"", "")
            .to_lowercase(),
    );

    let wayback_response = reqwest::blocking::get(wayback_availabilty_url)
        .unwrap()
        .text()
        .unwrap();

    let wayback_data: ArchiveInfo = serde_json::from_str(&wayback_response).unwrap();

    let html_response = reqwest::blocking::get(wayback_data.archived_snapshots.closest.url)
        .unwrap()
        .text()
        .unwrap();
    let (image_url, synopsys) = parse_html(html_response);

    let message = format!("{}\n\n{}\n", image_url, synopsys);

    message

    //print!("Image url = {}\n\n\nSynopsis = {}", image_url, synopsys);
}

fn parse_html(response: String) -> (String, String) {
    let parsed_html = Html::parse_document(&response);

    // Finding the image

    let img_div_selector = Selector::parse("div.seriesimg").unwrap();
    let image_container = parsed_html.select(&img_div_selector).next().unwrap();

    let image_selector = Selector::parse("img").unwrap();
    let image = image_container.select(&image_selector).next().unwrap();

    let image_url = image.value().attr("src").unwrap();

    // Finding the synopsis

    let synopsis_div_selector = Selector::parse("#editdescription").unwrap();
    let synopsis_container = parsed_html.select(&synopsis_div_selector).next().unwrap();

    let synopsys_selector = Selector::parse("p").unwrap();
    let synopsys = synopsis_container
        .select(&synopsys_selector)
        .map(|chunk| chunk.text().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n\n");

    (String::from(image_url), synopsys)
}
