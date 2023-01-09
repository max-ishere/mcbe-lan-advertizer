use clap::CommandFactory;

#[path = "src/config/args.rs"]
mod args;

use args::Args;

fn main() -> std::io::Result<()> {
    let out_dir = std::path::PathBuf::from("doc/man");

    let cmd = Args::command();
    let man = clap_mangen::Man::new(cmd.clone());
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    // std::fs::write(out_dir.join(format!("{}.roff", cmd.get_name())), buffer)?;

    Ok(())
}
