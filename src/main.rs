use anyhow::Result;
use clap::{AppSettings, Clap};
use qotdsh::app::App;
use qotdsh::subcommands;

/// qotdsh (Quote of the Day for your shell)
#[derive(Clap)]
#[clap(
    version = "0.1",
    author = "Kerli Low <kerlilow@gmail.com>",
    setting = AppSettings::ColoredHelp,
)]
struct Opts {
    #[clap(subcommand)]
    subcmd: Subcommand,
}

#[derive(Clap)]
enum Subcommand {
    /// Show latest quote.
    Show,
    /// Fetch a new quote.
    Fetch,
    /// List recent quotes.
    Recent,
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    let app = App::new()?;
    match opts.subcmd {
        Subcommand::Show => subcommands::show::run(&app),
        Subcommand::Fetch => subcommands::fetch::run(&app),
        Subcommand::Recent => subcommands::recent::run(&app),
    }
}
