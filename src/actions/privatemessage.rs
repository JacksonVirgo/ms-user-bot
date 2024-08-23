use std::time::Duration;

use async_std::task::sleep;
use chromiumoxide::Page;

pub struct PrivateMessage {
    pub subject: String,
    pub message: String,
    pub recipients: Vec<String>,
    pub cc: Vec<String>,
}

impl PrivateMessage {
    pub fn new<'a>(subject: &'a str, message: &'a str) -> Self {
        Self {
            subject: subject.to_owned(),
            message: message.to_owned(),
            recipients: vec![],
            cc: vec![],
        }
    }

    pub fn add_recipient(&mut self, recipient: &str) -> &mut Self {
        self.recipients.push(recipient.to_owned());
        self
    }

    pub fn add_cc(&mut self, cc: &str) -> &mut Self {
        self.cc.push(cc.to_owned());
        self
    }
}

pub async fn send_pm_to_users(
    page: &mut Page,
    private_message: PrivateMessage,
) -> Result<(), Box<dyn std::error::Error>> {
    let full_thread_url = "https://forum.mafiascum.net/ucp.php?i=pm&mode=compose";
    page.goto(full_thread_url)
        .await?
        .wait_for_navigation()
        .await?;

    match page.find_element("#subject").await {
        Ok(el) => Ok(el),
        Err(err) => Err(err),
    }?
    .click()
    .await?
    .type_str(private_message.subject)
    .await?;

    match page.find_element("#message").await {
        Ok(el) => Ok(el),
        Err(_) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not find message field",
        ))),
    }?
    .click()
    .await?
    .type_str(private_message.message)
    .await?;

    if private_message.recipients.len() > 0 {
        let recipients = private_message.recipients.join("\n");
        println!("Recipients: {}", recipients);

        match page.find_element("#username_list").await {
            Ok(el) => Ok(el),
            Err(_) => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not find username_list field",
            ))),
        }?
        .click()
        .await?
        .type_str(recipients)
        .await?;

        match page.find_element("input[name=\"add_to\"]").await {
            Ok(el) => Ok(el),
            Err(_) => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not find add button",
            ))),
        }?
        .click()
        .await?;

        page.wait_for_navigation().await?;
    }

    if private_message.cc.len() > 0 {
        let ccs = private_message.cc.join("\n");
        println!("Recipients: {}", ccs);

        match page.find_element("#username_list").await {
            Ok(el) => Ok(el),
            Err(_) => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not find username_list field",
            ))),
        }?
        .click()
        .await?
        .type_str(ccs)
        .await?;

        match page.find_element("input[name=\"add_bcc\"]").await {
            Ok(el) => Ok(el),
            Err(_) => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Could not find add_bcc button",
            ))),
        }?
        .click()
        .await?;
    }

    sleep(Duration::from_secs(1)).await;

    let mut inputs = page.find_elements("input").await?;
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
