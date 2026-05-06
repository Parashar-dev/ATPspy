mod models;
mod ui;

use models::{Package,RiskLevel};

use crossterm::{
    event::{self,Event,KeyCode,KeyEventKind}, execute, terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}
};


use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::stdout;
use std::time::Duration;



enum Screen {
    Password,
    PackageList,
}

fn main() {
    let mut password = String::new();
    let mut screen = Screen::Password;
    let mut packages: Vec<Package> = Vec::new();
    let mut selected_idx: usize = 0;
    enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen).unwrap();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
    loop {
        // ── Draw ──
        let pass_len = password.len();
        terminal.draw(|f| {
            match screen {
                Screen::Password => ui::password_screen::draw(f, pass_len),
                Screen::PackageList => ui::list_screen::draw(f, &packages, selected_idx),
            }
        }).unwrap();
        // ── Input ──
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if key.kind == KeyEventKind::Press {
                    match screen {
                        Screen::Password => match key.code {
                            KeyCode::Char(c) => password.push(c),
                            KeyCode::Backspace => { password.pop(); },
                            KeyCode::Esc => break,
                            KeyCode::Enter => {
                                // Password done → load packages → switch screen
                                packages = parse_packages();
                                screen = Screen::PackageList;
                            },
                            _ => {}
                        },
                        Screen::PackageList => match key.code {
                            KeyCode::Down | KeyCode::Char('j') => {
                                if selected_idx < packages.len().saturating_sub(1) {
                                    selected_idx += 1;
                                }
                            },
                            KeyCode::Up | KeyCode::Char('k') => {
                                if selected_idx > 0 {
                                    selected_idx -= 1;
                                }
                            },
                            KeyCode::Char('q') | KeyCode::Esc => break,
                            _ => {}
                        },
                    }
                }
            }
        }
    }
    disable_raw_mode().unwrap();
    execute!(stdout(), LeaveAlternateScreen).unwrap();
}







fn is_dev_mode() -> bool {
    dotenvy::dotenv().ok();
    let mode = std::env::var("APP_MODE").unwrap_or_else(|_| "production".to_string());
    mode == "development"
}

fn parse_packages() -> Vec<Package> {
    let stdout = if is_dev_mode() {
        println!("        [DEV-MODE] using mock data ---\n");
        std::fs::read_to_string("src/test/mockData/upgradable.txt")
            .expect("Mock data fille not found!")
    } else {
        let output = std::process::Command::new("apt")
            .args(["list", "--upgradable"])
            .env("LANG", "C")
            .output()
            .expect("apt command failed");
        String::from_utf8_lossy(&output.stdout).to_string()
    };

    let mut packages = Vec::new();

    for line in stdout.lines() {
        if line.starts_with("Listing") || line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 {
            let mut name_repo = parts[0].splitn(2, '/');
            let name = name_repo.next().unwrap_or("").to_string();
            let repo = name_repo.next().unwrap_or("unknown").to_string();
            let new_version = parts[1].to_string();
            let architecture = parts[2].to_string();
            let current_version = if parts.len() >= 6 {
                parts[5].trim_end_matches(']').to_string()
            } else {
                "unknown".to_string()
            };

            packages.push(Package {
                name,
                repo,
                current_version,
                new_version,
                architecture,
                risk_level,
            });
        }
    }
    packages
}


fn assign_risk(pkg : &mut Package){
    //Critical packages
    if ["libc6","grub-pc", "grub-efi"].contains(&pkg,name.as_string())
    || pkg.name.starts_with("linux-image"){
        pkg.risk_level=RiskLevel::Critical;
    }
}