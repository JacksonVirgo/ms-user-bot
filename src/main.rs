use async_std::task::sleep;
use chromiumoxide::{
    browser::{Browser, BrowserConfig},
    Element, Page,
};
use core::panic;
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
    page.wait_for_navigation().await?;

    println!("Page loaded");

    page.goto("https://forum.mafiascum.net/posting.php?mode=reply&t=12551#preview")
        .await?
        .wait_for_navigation()
        .await?;

    println!("Page navigated");

    let h2 = page.find_element("h2").await?;
    let h2_text = h2.inner_text().await?.unwrap_or("ERR".to_owned());
    println!("{}", h2_text);

    page.find_element("#message")
        .await?
        .click()
        .await?
        .type_str(format!("Automated test subject in thread {} (if this doesn't work, JV has almost definitely pulled his hair out)", h2_text))
        .await?;

    let mut inputs = page.find_elements("#postform input").await?;
    for input in inputs.iter_mut() {
        let name = input
            .attribute("name")
            .await
            .unwrap_or(Some("error".to_owned()))
            .unwrap_or("none".to_owned());

        let html_type = input
            .attribute("type")
            .await
            .unwrap_or(Some("error".to_owned()))
            .unwrap_or("none".to_owned());

        if name.eq("post") && html_type.eq("submit") {
            println!("Found subject field");
            input.click().await?;
        } else {
            println!("[ERR] {} {}", name, html_type);
        }
    }

    page.wait_for_navigation().await?;
    browser.close().await?;
    handle.await;
    Ok(())
}

async fn login(page: &mut Page) -> Result<(), Box<dyn std::error::Error>> {
    page.goto("https://forum.mafiascum.net/ucp.php?mode=login&redirect=index.php")
        .await?
        .wait_for_navigation()
        .await?;

    let username_field = match page.find_element("input#username").await {
        Ok(f) => f,
        Err(_) => {
            println!("Username field not found, verify logged in");
            return Ok(());
        }
    };

    let password_field = match page.find_element("input#password").await {
        Ok(f) => f,
        Err(_) => {
            println!("Password field not found, verify logged in");
            return Ok(());
        }
    };

    let username = std::env::var("MS_USERNAME")?;
    let password = std::env::var("MS_PASSWORD")?;

    username_field.click().await?.type_str(username).await?;

    password_field
        .click()
        .await?
        .type_str(password)
        .await?
        .press_key("Enter")
        .await?;

    Ok(())
}
