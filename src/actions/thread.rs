use chromiumoxide::Page;

pub async fn send_message_to_thread(
    page: &mut Page,
    thread_id: &str,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let full_thread_url = format!(
        "https://forum.mafiascum.net/posting.php?mode=reply&t={}#preview",
        thread_id
    );

    page.goto(full_thread_url)
        .await?
        .wait_for_navigation()
        .await?;

    page.find_element("#message")
        .await?
        .click()
        .await?
        .type_str(message)
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
            input.click().await?;
        }
    }

    page.wait_for_navigation().await?;
    Ok(())
}
