use reqwest::Client;
use std::collections::HashSet;
use std::process::Command;
use std::time::Duration;

pub async fn get_port() -> Option<u16> {
    let pid = get_windows_starcraft_pid().await;
    if pid.is_none() {
        return None;
    }
    let ports = get_windows_open_ports(pid.unwrap()).await;
    find_working_port(ports).await
}

async fn get_windows_starcraft_pid() -> Option<u32> {
    let output = Command::new("tasklist")
        .args(&["/fi", "imagename eq StarCraft.exe", "/fo", "csv"])
        .output()
        .expect("Failed to execute tasklist");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.trim().split('\n').collect();
    if lines.len() == 1 {
        return None;
    }
    let pid_line = lines[1].trim();
    let pid: u32 = pid_line
        .split(',')
        .nth(1)?
        .trim()
        .replace('"', "")
        .parse()
        .ok()?;
    Some(pid)
}

async fn get_windows_open_ports(pid: u32) -> HashSet<u16> {
    let output = Command::new("netstat")
        .args(&["-ano"])
        .output()
        .expect("Failed to execute netstat");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let ports: HashSet<u16> = stdout
        .lines()
        .filter_map(|line| {
            let segments: Vec<&str> = line.trim().split_whitespace().collect();
            if segments.len() > 0 && segments.last()? == &pid.to_string() {
                let local_address = segments[1];
                local_address.split(':').last()?.parse::<u16>().ok()
            } else {
                None
            }
        })
        .collect();
    ports
}

async fn find_working_port(ports: HashSet<u16>) -> Option<u16> {
    let client = Client::new();
    for port in ports {
        let url = format!("http://127.0.0.1:{}/panel/ChatPanel/1", port);
        let response = client
            .get(&url)
            .timeout(Duration::from_millis(50))
            .send()
            .await;
        if let Ok(_) = response {
            return Some(port);
        }
    }
    None
}
