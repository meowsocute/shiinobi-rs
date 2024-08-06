use clap::Command;

pub fn cli() -> Command {
    Command::new("shiinobi")
        .about("Anime Data Parser")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(Command::new("get-anime-themes").about("Fetch Anime themes"))
        .subcommand(
            Command::new("get-specific-staff-information").about("Fetch Specific Staff info."),
        )
}
