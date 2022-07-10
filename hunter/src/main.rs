use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {
    ///Subdomain enumeration using crt.sh API
    // ./hunter subdomain-enumerate --url "https://www.google.com"
    Enumerate {
        ///The URL you want to enumerate
        url: String,
    },
}




fn main() {
    let cli_arguments = Cli::parse();

    match cli_arguments.command {
        Commands::Enumerate { url } => recon::passive::subdomain::enumerate(&url),
    }
}
