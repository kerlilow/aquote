use anyhow::Result;
use aquote::app::App;
use aquote::subcommands;
use clap::{AppSettings, Clap};

/// aquote (Quote of the Day)
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
    Show(subcommands::show::Opts),
    /// Fetch a new quote.
    Fetch,
    /// List recent quotes.
    Recent,
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    let app = App::new()?;
    match opts.subcmd {
        Subcommand::Show(sub_opts) => subcommands::show::run(&app, &sub_opts),
        Subcommand::Fetch => subcommands::fetch::run(&app),
        Subcommand::Recent => subcommands::recent::run(&app),
    }
}
