use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author="Carlos Guerrero Alvarez", version, about)]
pub struct Arguments {
    /// a file to scan for numbers
    pub file_name: Option<String>,
}
