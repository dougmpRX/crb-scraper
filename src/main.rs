use std::fs::File;
use std::io::Write;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let base_url = std::env::var("BASE_URL")
        .expect("Could not load environment variable.");

    let client = reqwest::Client::new();

    let response = client.get(&base_url).send().await?;
    let body = response.text().await?;

    let document = scraper::Html::parse_document(&body);

    let entry_media_selector = scraper::Selector::parse(".entry-media").unwrap();
    let media_image_selector = scraper::Selector::parse(".media-image").unwrap();
    
    let mut movies: Vec<Movie> = Vec::new();

    for entry in document.select(&entry_media_selector) {
        let id = entry
            .select(&media_image_selector)
            .next()
            .and_then(|el| el.value().attr("data-video-src"))
            .and_then(|src| src.split("/").nth(5))
            .unwrap_or_default();

        let title = entry
            .select(&media_image_selector)
            .next()
            .and_then(|el| el.value().attr("title"))
            .unwrap_or_default();

        let image_url = entry
            .select(&media_image_selector)
            .next()
            .and_then(|el| el.value().attr("src"))
            .unwrap_or_default();

        let movie = Movie {
            id: String::from(id),
            title: String::from(title),
            image_url: String::from(image_url)
        };

        movies.push(movie);
    }

    let mut file = File::create("movies.csv")
            .expect("Could not create file.");
    writeln!(file, "id, title, image_url")
            .expect("Could not write.");

    for movie in &movies {
        writeln!(file, "{},{},{}", movie.id, movie.title, movie.image_url)
            .expect("Could not write.")
    }

    Ok(())
}

struct Movie {
    id: String,
    title: String,
    image_url: String,
}