use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    name: Option<String>,

    #[arg(short, long)]
    xxs: Option<String>,

    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    println!("name: {:?}", cli.name.as_deref());
}
