use reqwest::Client;
use std::collections::HashSet;
use std::process::{Command, Output};
use std::time::Duration;

pub async fn get_port() -> Option<u16> {
    let pid = get_starcraft_pid().await?;
    let ports = get_open_ports(pid).await;
    find_working_port(ports).await
}

async fn get_starcraft_pid() -> Option<u32> {
    if cfg!(target_os = "windows") {
        let output = Command::new("tasklist")
            .args(&["/fi", "imagename eq StarCraft.exe", "/fo", "csv"])
            .output()
            .expect("Failed to execute tasklist");
        return get_pid_from_task_list_stdout(output);
    } else {
        let output = Command::new("pgrep")
            .arg("-f")
            .arg("StarCraft")
            .output()
            .expect("Failed to execute pgrep");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let pid_str = stdout.trim().lines().next()?;
        let pid: u32 = pid_str.parse().ok()?;
        return Some(pid);
    };
}

fn get_pid_from_task_list_stdout(output: Output) -> Option<u32> {
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

async fn get_open_ports(pid: u32) -> HashSet<u16> {
    if cfg!(target_os = "windows") {
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
        return ports;
    } else {
        let output = Command::new("lsof")
            .args(&["-Pan", "-p", &pid.to_string()])
            .output()
            .expect("Failed to execute lsof");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let ports: HashSet<u16> = stdout
            .lines()
            .filter_map(|line| {
                let segments: Vec<&str> = line.trim().split_whitespace().collect();
                if segments.len() > 0 {
                    // On Unix, the port will be in the format `127.0.0.1:port`
                    if let Some(local_address) = segments.get(8) {
                        // Adjust index as necessary
                        let port_str = local_address.split(':').last()?;
                        port_str.parse::<u16>().ok()
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        return ports;
    };
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
