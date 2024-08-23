mod actions;

use actions::login::login;
use actions::thread::send_message_to_thread;
use chromiumoxide::browser::{Browser, BrowserConfig};
use dotenv::dotenv;
use futures::StreamExt;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let (mut browser, mut handler) =
        Browser::launch(BrowserConfig::builder().with_head().build()?).await?;

    let handle = async_std::task::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    let mut page = browser.new_page("https://forum.mafiascum.net").await?;

    login(&mut page).await?;
    send_message_to_thread(&mut page, "12551", "Test").await?;

    browser.close().await?;
    handle.await;
    Ok(())
}
