/// Tails your bookmarked Something Awful threads.
use clap::Parser;
use something_awful::client::{Client, ThreadPage, User};

#[derive(Debug, clap::Parser)]
struct Args {
    /// Credentials file. If provided, user credentials will be cached here. If
    /// the file doesn't exist or credentials are expired, you will be prompted
    /// for username and password.
    #[arg(long, default_value = ".something-awful.token")]
    auth: Option<String>,

    /// Time to sleep between rendering posts. Set to a higher value if you
    /// would like extra time to process each message as it scrolls by.
    #[arg(long, default_value_t = 1000)]
    sleep_between_posts_millis: u64,

    /// Time to sleep between polling threads. Set to a higher value if you have
    /// many favorite threads to poll and want to avoid sending too many
    /// requests to the server.
    #[arg(long, default_value_t = 1000)]
    sleep_between_threads_millis: u64,

    /// Time to sleep between refreshing new threads. Don't poll too frequently.
    #[arg(long, default_value_t = 30000)]
    sleep_between_refresh_millis: u64,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let args = Args::parse();
    let client = Client::new()?;

    let mut logged_in = false;
    if let Some(auth) = args.auth.as_ref() {
        if let Ok(file) = std::fs::File::open(auth) {
            let reader = std::io::BufReader::new(file);
            if let Ok(()) = client.load_credentials(reader) {
                if let Ok(Some(_)) = client.fetch_profile(User::CurrentUser).await {
                    println!("Logged in.");
                    logged_in = true;
                }
            }
        }
    }

    if !logged_in {
        let username = rpassword::prompt_password("Username (hidden): ")?;
        let password = rpassword::prompt_password("Password (hidden): ")?;
        client.login(&username, &password).await?;

        if let Some(auth) = args.auth.as_ref() {
            let mut file = std::fs::File::create(auth)?;
            client.save_credentials(&mut file)?;
        }
    }

    loop {
        let threads = client.fetch_bookmarked_threads().await?;
        for thread in threads.into_iter() {
            if thread.unread > 0 {
                let posts = client.fetch_posts(&thread.id, ThreadPage::New).await?;
                for post in posts.into_iter() {
                    println!("----------");
                    println!(r#" /\_/\ "#);
                    println!(r#"( o.o )"#);
                    println!(r#" > ^ <"#);
                    println!();
                    println!("thread: {}", thread.title);
                    println!("author: {}", post.author_username);
                    println!("time: {}", post.post_date);
                    println!("----------");
                    println!("{}", html2md::parse_html(&post.post_body));
                    tokio::time::sleep(std::time::Duration::from_millis(
                        args.sleep_between_posts_millis,
                    ))
                    .await;
                }
                tokio::time::sleep(std::time::Duration::from_millis(
                    args.sleep_between_threads_millis,
                ))
                .await;
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(
            args.sleep_between_refresh_millis,
        ))
        .await;
    }
}
