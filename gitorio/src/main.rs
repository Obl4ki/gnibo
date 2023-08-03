use chrono::{DateTime, Local};
use std::env;
use std::error::Error;
use std::thread;
use std::{process::Command, time::Duration};
use sysinfo::{System, SystemExt};

const FACTORIO_STEAM_ID: &str = "427520";
const SLEEP_SECONDS: u64 = 10;
const GIT_DIR: &str = r"C:\Users\W10\AppData\Roaming\Factorio\saves\";
const SAVE_NAME: &str = "gnibo";


fn amirmir() {
    thread::sleep(Duration::from_millis(SLEEP_SECONDS * 1000));
}

fn main() -> Result<(), Box<dyn Error>> {
    env::set_current_dir(GIT_DIR)?;
    Command::new("git").arg("pull").output()?;

    // Run factorio
    // steam://rungameid/427520
    Command::new("steam")
        .arg(format!("steam://rungameid/{FACTORIO_STEAM_ID}"))
        .output()?;

    amirmir();

    // Block until closed
    loop {
        let s = System::new_all();
        if s.processes_by_name("factorio").count() > 0 {
            println!("Factorio still running");
        } else {
            println!("Factorio stopped running");
            break;
        }
        amirmir();
    }

    let now: DateTime<_> = Local::now();

    // Run git on save directory
    Command::new("git").args(["add", &format!("{SAVE_NAME}.zip")]).output()?;
    Command::new("git")
        .args(["commit", "-m", &format!("Saved at {now}")])
        .output()?;

    Command::new("git").arg("push").output()?;

    Ok(())
}
