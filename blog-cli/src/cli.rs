use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "blog-cli")]
#[command(about = "CLI client for the blog service")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    #[arg(short, long)]
    pub server: Option<String>,
}

#[derive(Subcommand)]
pub enum Command {
    Register {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        email: String,
        #[arg(short, long)]
        password: String,
    },
    Login {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: String,
    },
    Create {
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        content: String,
    },
    Get {
        #[arg(short, long)]
        id: i64,
    },
    Update {
        #[arg(short, long)]
        id: i64,
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        content: String,
    },
    Delete {
        #[arg(short, long)]
        id: i64,
    },
    List {
        #[arg(short, long, default_value = "10")]
        limit: i64,
        #[arg(short, long, default_value = "0")]
        offset: i64,
    },
}
