use reqwest;
use std::error::Error;

pub async fn resolve_final_url(url: &str) -> Result<String, Box<dyn Error>> {
    // Make a GET request, which will follow redirects by default
    let response = reqwest::get(url).await?;

    // Extract the final URL after following all redirects
    let final_url = response.url().clone();

    Ok(final_url.as_str().to_string())
}

#[cfg(test)]
pub mod test {
    use crate::*;

    #[tokio::test]
    async fn test_resolve_final_url() {
        let url = "https://spotify.link/fLKHSV0gZDb"; // Replace with your starting URL
        match resolve_final_url(url).await {
            Ok(final_url) => println!("Final URL: {}", final_url),
            Err(e) => println!("Error: {}", e),
        }
    }
}
