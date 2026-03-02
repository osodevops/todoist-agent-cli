use anyhow::Result;
use clap::CommandFactory;
use clap_complete::Shell;

use td_cli::cli::Cli;

pub fn execute(shell: Shell) -> Result<()> {
    let mut cmd = Cli::command();
    clap_complete::generate(shell, &mut cmd, "td", &mut std::io::stdout());
    Ok(())
}
