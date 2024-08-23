mod actions;

use actions::privatemessage::PrivateMessage;
use actions::{login::login, privatemessage::send_pm_to_users};
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

    let pm = PrivateMessage {
        subject: String::from("Subject"),
        message: String::from("Content"),
        recipients: vec!["JacksonVirgo".to_owned()],
        cc: vec![],
    };
    send_pm_to_users(&mut page, pm).await?;

    browser.close().await?;
    handle.await;
    Ok(())
}
