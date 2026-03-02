use clap::CommandFactory;
use clap_mangen::Man;
use std::fs;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let out_dir =
        PathBuf::from(std::env::var("MAN_OUT_DIR").unwrap_or_else(|_| "target/man".to_string()));
    fs::create_dir_all(&out_dir)?;

    let cmd = td_cli::cli::Cli::command();

    // Generate the top-level man page: td(1)
    let man = Man::new(cmd.clone());
    let mut buf = Vec::new();
    man.render(&mut buf)?;
    fs::write(out_dir.join("td.1"), buf)?;

    // Generate man pages for each subcommand: td-<sub>(1)
    for sub in cmd.get_subcommands() {
        if sub.get_name() == "help" {
            continue;
        }
        let name = format!("td-{}", sub.get_name());
        let sub_cmd = sub.clone().name(&name);
        let man = Man::new(sub_cmd);
        let mut buf = Vec::new();
        man.render(&mut buf)?;
        fs::write(out_dir.join(format!("{name}.1")), buf)?;

        // Generate man pages for nested subcommands: td-<sub>-<action>(1)
        for nested in sub.get_subcommands() {
            if nested.get_name() == "help" {
                continue;
            }
            let nested_name = format!("td-{}-{}", sub.get_name(), nested.get_name());
            let nested_cmd = nested.clone().name(&nested_name);
            let man = Man::new(nested_cmd);
            let mut buf = Vec::new();
            man.render(&mut buf)?;
            fs::write(out_dir.join(format!("{nested_name}.1")), buf)?;
        }
    }

    let count = fs::read_dir(&out_dir)?.count();
    println!("Generated {count} man pages in {}", out_dir.display());
    Ok(())
}
