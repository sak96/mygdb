use structopt::{clap::AppSettings, StructOpt};
#[derive(StructOpt)]
#[structopt(about, global_settings(&[AppSettings::VersionlessSubcommands, AppSettings::NoBinaryName, AppSettings::DisableHelpFlags, AppSettings::DisableVersion, AppSettings::DisableHelpSubcommand]))]
pub enum DebugCommand {
    #[structopt(visible_alias = "q", about = "quit debugging session")]
    Quit,
    #[structopt(visible_alias = "r", about = "run program with arguments")]
    Run { args: Vec<String> },
    #[structopt(visible_alias = "c", about = "continue debugging session")]
    Continue,
    #[structopt(visible_alias = "h", about = "help with debugging session")]
    Help,
}
