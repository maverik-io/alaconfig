use clap::{Args, Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(author = "Joseph Chacko <josephchacko2006@gmail.com>")]
#[command(version = "0.1.0")]
#[command(
    help_template = "{name} v{version}\n{author-section} {about-section}\n{usage-heading} {usage} \n\n{all-args}"
)]
#[command(about, long_about = None)]
pub struct AlaconfigArgs {
    #[clap(subcommand)]
    pub config: Config,
}

#[derive(Debug, Subcommand)]
pub enum Config {
    /// Set a config parameter
    Set(Setter),

    /// Get a config parameter
    Get(Getter),
}

#[derive(Debug, Args)]
pub struct Getter {
    /// Item can currenty be one of `font`, `size`, `opacity`, `theme`, `blur`
    pub item: String,
}

#[derive(Debug, Args)]
pub struct Setter {
    /// Item can currenty be one of `font`, `size`, `opacity`, `theme`, `blur`
    pub item: String,

    /// value to set
    pub value: Vec<String>,
}
