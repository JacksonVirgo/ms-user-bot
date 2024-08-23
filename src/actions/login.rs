use chromiumoxide::Page;

pub async fn login(page: &mut Page) -> Result<(), Box<dyn std::error::Error>> {
    page.goto("https://forum.mafiascum.net/ucp.php?mode=login&redirect=index.php")
        .await?
        .wait_for_navigation()
        .await?;

    let username_field = match page.find_element("input#username").await {
        Ok(f) => f,
        Err(_) => {
            println!("TODO: Verify logged in");
            return Ok(());
        }
    };

    let password_field = match page.find_element("input#password").await {
        Ok(f) => f,
        Err(_) => {
            println!("TODO: Verify logged in");
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

    page.wait_for_navigation().await?;
    Ok(())
}
