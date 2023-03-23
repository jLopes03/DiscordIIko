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

pub async fn get_novel_data(novel_name: &str) -> (String, String, String) {
    let client = reqwest::Client::new();

    let mut wayback_availabilty_url =
        "http://archive.org/wayback/available?url=https://www.novelupdates.com/series/".to_string();

    wayback_availabilty_url.push_str(&novel_name.trim().replace(" ", "-").replace("'","").to_lowercase());

    let wayback_response = client
        .get(wayback_availabilty_url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let wayback_data: ArchiveInfo = serde_json::from_str(&wayback_response).unwrap();

    let html_response = client
        .get(wayback_data.archived_snapshots.closest.url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    parse_html(html_response)

    //print!("Image url = {}\n\n\nSynopsis = {}", image_url, synopsys);
}

fn parse_html(response: String) -> (String, String, String) {
    let parsed_html = Html::parse_document(&response);

    // Finding the title

    let title_div_selector = Selector::parse("div.seriestitlenu").unwrap();
    let title = parsed_html
        .select(&title_div_selector)
        .next()
        .unwrap()
        .text()
        .next()
        .unwrap();

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

    (String::from(title), String::from(image_url), synopsys)
}
