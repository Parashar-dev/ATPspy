mod models;
mod ui;

use std::process::Stdio;
use std::thread;
use std::{process::Command, sync::mpsc};

use models::{Package, RiskLevel};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{
         EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
    },
};

use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io::stdout;
use std::time::Duration;

enum Screen {
    Password,
    Scanning,
    PackageList,
    Upgrading,
}

fn main() {
    let mut scan_rx: Option<mpsc::Receiver<String>> = None;
    let mut upgrade_rx: Option<mpsc::Receiver<String>> = None;

    let mut password_error: String = String::new();

    let mut upgrade_progress: usize = 0;
    let mut upgrade_total: usize = 0;
    let mut upgrade_logs: Vec<String> = Vec::new();
    let mut current_pkg: String = String::new();
    let mut upgrade_list: Vec<String> = Vec::new();

    let mut password = String::new();
    let mut spinner_tick: usize = 0;
    let mut scan_logs: Vec<String> = Vec::new();
    let mut screen = Screen::Password;
    let mut packages: Vec<Package> = Vec::new();
    let mut selected_idx: usize = 0;

    enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen).unwrap();
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
    loop {
        // ── Draw ──
        let pass_len = password.len();
        terminal
            .draw(|f| match screen {
                Screen::Password => ui::password_screen::draw(f, pass_len, &password_error),
                Screen::Scanning => ui::scan_screen::draw(f, spinner_tick, &scan_logs),
                Screen::PackageList => ui::list_screen::draw(f, &packages, selected_idx),
                Screen::Upgrading => ui::upgrad_screen::draw(
                    f,
                    &current_pkg,
                    upgrade_progress,
                    upgrade_total,
                    &upgrade_logs,
                ),
            })
            .unwrap();
        // ── Input ──
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if key.kind == KeyEventKind::Press {
                    match screen {
                        Screen::Password => match key.code {
                            KeyCode::Char(c) => password.push(c),
                            KeyCode::Enter => {
                                password_error.clear();
                                screen = Screen::Scanning;
                                scan_logs.clear();
                                spinner_tick = 0;
                                scan_logs.push("Running sudo at update....".to_string());

                                let pass = password.clone();
                                let (tx, rx) = mpsc::channel();
                                scan_rx = Some(rx);

                                thread::spawn(move || {
                                    use std::io::Write;

                                    // Clear any cached sudo credentials
                                    let _ = Command::new("sudo").arg("-k").status();

                                    let mut child = Command::new("sudo")
                                        .args(["-S", "apt", "update"])
                                        .env("DEBIAN_FRONTEND", "noninteractive")
                                        .env("SUDO_PROMPT", "") // ← empty prompt to avoid issues
                                        .stdin(Stdio::piped())
                                        .stdout(Stdio::piped())
                                        .stderr(Stdio::piped())
                                        .spawn()
                                        .expect("Failed to spawn sudo");

                                    if let Some(mut stdin) = child.stdin.take() {
                                        let _ = writeln!(stdin, "{}", pass);
                                        drop(stdin);
                                    }

                                    // Timeout — 30 sec max wait
                                    let timeout = std::time::Duration::from_secs(30);
                                    let start = std::time::Instant::now();

                                    loop {
                                        match child.try_wait() {
                                            Ok(Some(_)) => break, // process finished
                                            Ok(None) => {
                                                if start.elapsed() > timeout {
                                                    let _ = child.kill(); // kill if stuck
                                                    let _ = tx.send("__ERROR__".to_string());
                                                    return;
                                                }
                                                std::thread::sleep(
                                                    std::time::Duration::from_millis(100),
                                                );
                                            }
                                            Err(_) => {
                                                let _ = tx.send("__ERROR__".to_string());
                                                return;
                                            }
                                        }
                                    }

                                    let output = child.wait_with_output().expect("failed to wait");
                                    let stdout_str = String::from_utf8_lossy(&output.stdout);
                                    let stderr_str = String::from_utf8_lossy(&output.stderr);

                                    for line in stdout_str.lines() {
                                        let _ = tx.send(line.to_string());
                                    }
                                    for line in stderr_str.lines() {
                                        let _ = tx.send(line.to_string());
                                    }

                                    if output.status.success() {
                                        let _ = tx.send("__DONE__".to_string());
                                    } else {
                                        let _ = tx.send("__ERROR__".to_string());
                                    }
                                });
                            }
                            KeyCode::Backspace => {
                                password.pop();
                            }
                            KeyCode::Esc => break,

                            _ => {}
                        },
                        Screen::PackageList => match key.code {
                            KeyCode::Down | KeyCode::Char('j') => {
                                if selected_idx < packages.len().saturating_sub(1) {
                                    selected_idx += 1;
                                }
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                if selected_idx > 0 {
                                    selected_idx -= 1;
                                }
                            }
                            KeyCode::Char(' ') => {
                                if let Some(pkg) = packages.get_mut(selected_idx) {
                                    pkg.selected = !pkg.selected
                                }
                            }
                            KeyCode::Char('a') => {
                                for pkg in &mut packages {
                                    pkg.selected = true;
                                }
                            }
                            KeyCode::Char('d') => {
                                for pkg in &mut packages {
                                    pkg.selected = false;
                                }
                            }
                            KeyCode::Char('u') => {
                                upgrade_list = packages
                                    .iter()
                                    .filter(|p| p.selected)
                                    .map(|p| p.name.clone())
                                    .collect();
                                if !upgrade_list.is_empty() {
                                    upgrade_total = upgrade_list.len();
                                    upgrade_progress = 0;
                                    upgrade_logs.clear();
                                    spinner_tick = 0;
                                    current_pkg = upgrade_list[0].clone();
                                    upgrade_logs.push("Starting upgrade...".to_string());
                                    screen = Screen::Upgrading;

                                    let pass = password.clone();
                                    let pkgs = upgrade_list.clone();
                                    let (tx, rx) = mpsc::channel();
                                    upgrade_rx = Some(rx);

                                    thread::spawn(move || {
                                        use std::io::Write;

                                        let mut child = Command::new("sudo")
                                            .args(["-S", "apt", "install", "-y"])
                                            .args(&pkgs)
                                            .stdin(Stdio::piped())
                                            .stdout(Stdio::piped())
                                            .stderr(Stdio::piped())
                                            .spawn()
                                            .expect("Failed to spawn sudo");

                                        if let Some(mut stdin) = child.stdin.take() {
                                            let _ = writeln!(stdin, "{}", pass);
                                            drop(stdin);
                                        }

                                        let output =
                                            child.wait_with_output().expect("failed to wait");
                                        let stdout_str = String::from_utf8_lossy(&output.stdout);
                                        let stderr_str = String::from_utf8_lossy(&output.stderr);

                                        for line in stdout_str.lines() {
                                            let _ = tx.send(line.to_string());
                                        }
                                        for line in stderr_str.lines() {
                                            if !line.contains("[sudo]") {
                                                let _ = tx.send(line.to_string());
                                            }
                                        }
                                        if output.status.success() {
                                            let _ = tx.send("__DONE__".to_string());
                                        } else {
                                            let _ = tx.send("__ERROR__".to_string());
                                        }
                                    });
                                }
                            }

                            KeyCode::Char('q') | KeyCode::Esc => break,

                            _ => {}
                        },
                        Screen::Scanning => match key.code {
                            KeyCode::Esc => break,
                            _ => {}
                        },
                        Screen::Upgrading => match key.code {
                            KeyCode::Esc => break,
                            KeyCode::Enter => {
                                if upgrade_progress >= upgrade_total {
                                    screen = Screen::PackageList;
                                    spinner_tick = 0;
                                }
                            }
                            _ => {}
                        },
                    }
                }
            }
        }
        if let Screen::Scanning = screen {
            spinner_tick += 1;
            if let Some(ref rx) = scan_rx {
                while let Ok(msg) = rx.try_recv() {
                    if msg == "__DONE__" {
                        // Check if any error happened
                        let has_error = scan_logs.iter().any(|l| {
                            l.contains("incorrect password")
                                || l.contains("Sorry")
                                || l.contains("try again")
                                || l.contains("authentication failure")
                                || l.contains("authentication failure")
                                || l.contains("not in the sudoers")
                        });
                        if has_error {
                            scan_logs.push("❌ Wrong password!".to_string());
                            screen = Screen::Password;
                            password.clear();
                        } else {
                            scan_logs.push("Done! Loading packages...".to_string());
                            packages = parse_packages();
                            screen = Screen::PackageList;
                        }
                        scan_rx = None;
                        break;
                    } else if msg == "__ERROR__" {
                        scan_logs.push("❌ Authentication failed!".to_string());
                        password_error = "❌ Wrong password! Try again.".to_string();
                        screen = Screen::Password;
                        password.clear();
                        scan_rx = None;
                        break;
                    } else {
                        scan_logs.push(msg);
                    }
                }
            }
        }
        if let Screen::Upgrading = screen {
            spinner_tick += 1;

            if let Some(ref rx) = upgrade_rx {
                while let Ok(msg) = rx.try_recv() {
                    if msg == "__DONE__" {
                        upgrade_logs.push("".to_string());
                        upgrade_logs.push("✅ All packages upgraded!".to_string());
                        upgrade_logs.push("Press Enter to go back...".to_string());
                        upgrade_progress = upgrade_total;
                        upgrade_rx = None;
                        break;
                    } else {
                        upgrade_logs.push(msg.clone());
                        if msg.contains("Setting up") || msg.contains("Unpacking") {
                            if upgrade_progress < upgrade_total {
                                upgrade_progress += 1;
                            }
                            current_pkg = msg.split_whitespace().nth(2).unwrap_or("").to_string();
                        }
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
                //architecture,
                risk_level: RiskLevel::Low,
                selected: false,
            });
        }
    }

    for pkg in &mut packages {
        assingn_risk(pkg);
    }
    packages
}

//code to check risk levels

fn assingn_risk(pkg: &mut Package) {
    pkg.risk_level = if is_critical(pkg) {
        RiskLevel::Critical
    } else if is_high(pkg) {
        RiskLevel::High
    } else if is_medium(pkg) {
        RiskLevel::Medium
    } else {
        RiskLevel::Low
    };
}

fn is_critical(pkg: &Package) -> bool {
    ["libc6", "grub-pc", "grub-efi"].contains(&pkg.name.as_str())
        || pkg.name.starts_with("linux-image")
}

fn is_high(pkg: &Package) -> bool {
    ["systemd", "dbus"].contains(&pkg.name.as_str()) || pkg.name.starts_with("libssl")
}

fn is_medium(pkg: &Package) -> bool {
    pkg.name.starts_with("python") || pkg.repo.contains("security")
}
