use anyhow::anyhow;
use blog_cli::{
    cli::{Cli, Command},
    output, token,
};
use blog_client::GrpcClient;
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let server = cli
        .server
        .unwrap_or_else(|| String::from("http://localhost:50051"));
    let client = GrpcClient::connect(&server)
        .await
        .map_err(|e| anyhow!("Failed to connect to gRPC server: {}", e))?;
    let token = token::load_token()?;

    match cli.command {
        Command::Register {
            username,
            email,
            password,
        } => {
            let resp = client
                .request()
                .register(&username, &email, &password)
                .await
                .map_err(|e| anyhow!("Registration failed: {}", e))?;
            token::save_token(&resp.token)?;
            println!("Registration successful. Token saved.");
            output::print_json(&resp.user)?;
        }

        Command::Login { username, password } => {
            let resp = client
                .request()
                .login(&username, &password)
                .await
                .map_err(|e| anyhow!("Login failed: {}", e))?;
            token::save_token(&resp.token)?;
            println!("Login successful. Token saved.");
            output::print_json(&resp.user)?;
        }

        Command::Create { title, content } => {
            let token = token.ok_or_else(|| anyhow!("Not logged in. Please run `login` first."))?;
            let post = client
                .request()
                .with_auth(&token)
                .create_post(&title, &content)
                .await
                .map_err(|e| anyhow!("Create post failed: {}", e))?;
            println!("Post created.");
            output::print_json(&post)?;
        }

        Command::Get { id } => {
            let post = client
                .request()
                .get_post(id)
                .await
                .map_err(|e| anyhow!("Get post failed: {}", e))?;
            println!("Post found.");
            output::print_json(&post)?;
        }

        Command::Update { id, title, content } => {
            let token = token.ok_or_else(|| anyhow!("Not logged in. Please run `login` first."))?;
            let post = client
                .request()
                .with_auth(&token)
                .update_post(id, &title, &content)
                .await
                .map_err(|e| anyhow!("Update post failed: {}", e))?;
            println!("Post updated.");
            output::print_json(&post)?;
        }

        Command::Delete { id } => {
            let token = token.ok_or_else(|| anyhow!("Not logged in. Please run `login` first."))?;
            client
                .request()
                .with_auth(&token)
                .delete_post(id)
                .await
                .map_err(|e| anyhow!("Delete post failed: {}", e))?;
            println!("Post deleted.");
        }

        Command::List { limit, offset } => {
            let resp = client
                .request()
                .list_posts(limit, offset)
                .await
                .map_err(|e| anyhow!("List posts failed: {}", e))?;
            println!("Posts found (total: {})", resp.total);
            output::print_json(&resp.posts)?;
        }
    }

    Ok(())
}
