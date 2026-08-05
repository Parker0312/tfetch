use std::fs;

fn main() {
    println!(r#"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢀⡴⢲⡄⠀⠀⠀⢀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⢀⡞⠀⠀⡇⠀⢀⡴⠋⠁⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⣸⠁⠀⠀⡇⣠⠟⠀⠀⠀⣼⣠⣤⣤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⣿⠀⢠⢤⡿⠃⣀⠀⢀⡞⠉⠁⠀⠀⠈⠙⠶⡄⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⣻⣰⡏⣼⢁⡴⠋⣰⠋⠀⠀⠀⠀⠀⠀⠀⠀⠹⡄⣀⡀⠀⠀⠀⠀
⠀⠀⠀⢀⡾⠋⠉⠁⠀⠙⠁⠞⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⡈⢷⠀⠀⠀⠀
⠀⠀⠀⣼⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄⠀⠀⠀⠀⠀⠀⠀⢀⣧⡾⠁⠀⠀⠀
⠀⠀⠀⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄⠀⠀⠀⠀⠀⠀⠀⣾⠃⠀⠀⠀⠀⠀
⠀⠀⠀⣹⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⠁⠀⠀⡀⠀⠀⢰⡾⠋⠀⠀⠀⠀⠀⠀
⠀⠀⠀⢻⡇⣀⡀⠀⠺⣿⠇⠀⣀⣤⣄⣀⣠⣬⣥⣤⠾⠛⠁⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠉⠛⠓⠂⠤⠤⠖⠊⠉⠉⠉⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    "#);
    println!("✩｡:*•.─────  ❁ ❁  ─────.•*:｡✩\n");

    println!("\x1b[34mHost: \x1b[0m{}", 
        fs::read_to_string("/proc/sys/kernel/hostname")
            .ok()
            .as_deref()
            .map(|s| s.trim())
            .unwrap_or("Unknown")
    );

    let os = fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|c| c.lines()
            .find(|l| l.starts_with("PRETTY_NAME="))
            .map(|l| l.strip_prefix("PRETTY_NAME=").unwrap_or("").trim_matches('"').to_string()))
        .unwrap_or_else(|| "Unknown".to_string());
    println!("\x1b[34mOS: \x1b[0m{}", os);

    let cpu = fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|c| c.lines()
            .find(|l| l.starts_with("model name"))
            .map(|l| l.split(':').nth(1).unwrap_or("").trim().to_string()))
        .unwrap_or_else(|| "Unknown".to_string());
    println!("\x1b[34mCPU: \x1b[0m{}", cpu);

    if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
        let total = meminfo.lines()
            .find(|l| l.starts_with("MemTotal"))
            .and_then(|l| l.split_whitespace().nth(1).and_then(|n| n.parse::<i64>().ok()))
            .map(|kb| (kb / (1024 * 1024)) as i64)
            .unwrap_or(0);
        println!("\x1b[34mMemory: \x1b[0m~{} GiB", total);
    }

    if let Ok(partitions) = fs::read_to_string("/proc/partitions") {
        if let Some(disk) = partitions.lines().nth(2) {
            if let Some(size_kb) = disk.split_whitespace().nth(2)
                .and_then(|n| n.parse::<i64>().ok()) {
                println!("\x1b[34mDisk: \x1b[0m~{} GiB", (size_kb / (1024 * 1024)));
            }
        }
    }
    println!();
}

