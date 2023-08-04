mod config;

use chrono::Local;
use color_eyre::Result;
use std::env;
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::str::FromStr;
use std::thread;
use std::{process::Command, time::Duration};
use sysinfo::{System, SystemExt};

const FACTORIO_STEAM_ID: &str = "427520";

fn amirmir(duration_secs: u64) {
    thread::sleep(Duration::from_millis(duration_secs * 1000));
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cfg = config::load_config(PathBuf::from_str("config.yaml")?)?;
    if cfg.dry_run {
        println!("App is running in the dry-run mode.")
    }

    env::set_current_dir(cfg.save_dir)?;

    println!("git pull -X theirs;");

    if !cfg.dry_run {
        let out = Command::new("git").args(["pull", "-X", "theirs"]).output()?;
        stdout().write_all(&out.stdout)?;
    }

    println!("{} steam://rungameid/{}", cfg.steam_path, FACTORIO_STEAM_ID);

    if !cfg.dry_run {
        let out = Command::new("steam")
            .arg(format!("steam://rungameid/{FACTORIO_STEAM_ID}"))
            .output()?;
        stdout().write_all(&out.stdout)?;
    }

    amirmir(cfg.refresh_interval_seconds);

    loop {
        println!("Waiting for factorio to close...");
        let s = System::new_all();
        if s.processes_by_name("factorio").count() == 0 {
            println!("Factorio stopped running");
            break;
        }
        amirmir(cfg.refresh_interval_seconds);
    }

    let now = Local::now().format("%d/%m/%Y %H:%M");

    println!("git add {}.zip", cfg.save_name);
    println!("git commit -m \"Saved at {now}\"");
    println!("git push;");

    if !cfg.dry_run {
        let out = Command::new("git")
            .args(["add", &format!("{}.zip", cfg.save_name)])
            .output()?;
        stdout().write_all(&out.stdout)?;

        let out = Command::new("git")
            .args(["commit", "-m", &format!("Saved at {now}")])
            .output()?;
        stdout().write_all(&out.stdout)?;

        let out = Command::new("git").arg("push").output()?;
        stdout().write_all(&out.stdout)?;
    }

    println!("Done.");
    Ok(())
}
