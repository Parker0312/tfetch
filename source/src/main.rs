// Rust is fucking unreadable I swear to GOD

use std::env;
use std::fs;
use colored::Colorize;

struct PrideFlag {
    colors: Vec<fn(&str) -> String>,
}

fn main() {

    let hostname = fs::read_to_string("/etc/hostname").unwrap_or_default().trim().to_string();

    let os = fs::read_to_string("/etc/os-release").unwrap_or_default().lines().find(|line| line.starts_with("PRETTY_NAME=")).and_then(|line| line.split('=').nth(1)).map(|s| s.trim_matches('"').to_string()).unwrap_or_else(|| "Unknown".to_string());

    let cpu = fs::read_to_string("/proc/cpuinfo").unwrap_or_default().lines().find(|line| line.starts_with("model name")).and_then(|line| line.split(':').nth(1)).map(|s| s.trim().to_string()).unwrap_or_else(|| "Unknown".to_string());

    let meminfo = fs::read_to_string("/proc/meminfo").unwrap_or_default();
    let mem_total: u64 = meminfo.lines().find(|line| line.starts_with("MemTotal")).and_then(|line| line.split_whitespace().nth(1)).and_then(|s| s.parse().ok()).unwrap_or(0);
    let memory = format!("~{} GiB", mem_total / 1024 / 1024);

    let disk = std::process::Command::new("df").args(&["-h", "/"]).output().ok().and_then(|output| { let s = String::from_utf8_lossy(&output.stdout); s.lines().nth(1).and_then(|line| line.split_whitespace().nth(1).map(|s| format!("~{}", s))) }).unwrap_or_else(|| "Unknown".to_string());


    let args: Vec<String> = env::args().collect();

    let ascii_art = vec![
        "⠀⠀⠀⠀⠀⠀⢀⡴⢲⡄⠀⠀⠀⢀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⢀⡞⠀⠀⡇⠀⢀⡴⠋⠁⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⣸⠁⠀⠀⡇⣠⠟⠀⠀⠀⣼⣠⣤⣤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⣿⠀⢠⢤⡿⠃⣀⠀⢀⡞⠉⠁⠀⠀⠈⠙⠶⡄⠀⠀⠀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⣻⣰⡏⣼⢁⡴⠋⣰⠋⠀⠀⠀⠀⠀⠀⠀⠀⠹⡄⣀⡀⠀⠀⠀⠀",
        "⠀⠀⠀⢀⡾⠋⠉⠁⠀⠙⠁⠞⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⡈⢷⠀⠀⠀⠀",
        "⠀⠀⠀⣼⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄⠀⠀⠀⠀⠀⠀⠀⢀⣧⡾⠁⠀⠀⠀",
        "⠀⠀⠀⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄⠀⠀⠀⠀⠀⠀⠀⣾⠃⠀⠀⠀⠀⠀",
        "⠀⠀⠀⣹⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⠁⠀⠀⡀⠀⠀⢰⡾⠋⠀⠀⠀⠀⠀⠀",
        "⠀⠀⠀⢻⡇⣀⡀⠀⠺⣿⠇⠀⣀⣤⣄⣀⣠⣬⣥⣤⠾⠛⠁⠀⠀⠀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠉⠛⠓⠂⠤⠤⠖⠊⠉⠉⠉⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
    ];


    if args.len() < 2 {
        for line in &ascii_art {
            println!("{}", line);
        }
        println!("✩｡:*•.─────  ❁ ❁  ─────.•*:｡✩\n");
        println!("Host: {}", hostname);
        println!("OS: {}", os);
        println!("CPU: {}", cpu);
        println!("Memory: {}", memory);
        println!("Disk: {}", disk);
        return;
    }

    let command = &args[1].to_lowercase();
    let flag_name = command.trim_start_matches("--");


    let flag = match flag_name {
        "nonbinary" => Some(PrideFlag {
            colors: vec![
                |s| s.yellow().to_string(),
                |s| s.white().to_string(),
                |s| s.purple().to_string(),
                |s| s.bright_black().to_string(),
            ],
        }),
        "boyflux" => Some(PrideFlag {
            colors: vec![
                |s| s.truecolor(173, 216, 230).to_string(),
                |s| s.truecolor(65, 105, 225).to_string(),
                |s| s.truecolor(0, 35, 102).to_string(),
                |s| s.truecolor(144, 238, 144).to_string(),
                |s| s.truecolor(0, 35, 102).to_string(),
                |s| s.truecolor(173, 216, 230).to_string(),
            ],
        }),
        "girlflux" => Some(PrideFlag {
            colors: vec![
                |s| s.truecolor(245, 222, 179).to_string(),
                |s| s.truecolor(255, 105, 180).to_string(),
                |s| s.truecolor(139, 0, 0).to_string(),
                |s| s.truecolor(210, 180, 140).to_string(),
                |s| s.truecolor(139, 0, 0).to_string(),
                |s| s.truecolor(255, 105, 180).to_string(),
                |s| s.truecolor(245, 222, 179).to_string(),
            ],
        }),
        "trans" => Some(PrideFlag {
            colors: vec![
                |s| s.cyan().to_string(),
                |s| s.bright_magenta().to_string(),
                |s| s.white().to_string(),
                |s| s.bright_magenta().to_string(),
                |s| s.cyan().to_string(),
            ],
        }),
        "lesbian" => Some(PrideFlag {
            colors: vec![
                |s| s.truecolor(255, 127, 0).to_string(),
                |s| s.truecolor(255, 168, 126).to_string(),
                |s| s.white().to_string(),
                |s| s.bright_magenta().to_string(),
                |s| s.truecolor(191, 21, 107).to_string(),
            ],
        }),
        "gay" => Some(PrideFlag {
            colors: vec![
                |s| s.bright_red().to_string(),
                |s| s.truecolor(255, 165, 0).to_string(),
                |s| s.yellow().to_string(),
                |s| s.green().to_string(),
                |s| s.bright_blue().to_string(),
                |s| s.truecolor(148, 0, 211).to_string(),
            ],
        }),
        "bi" | "bisexual" => Some(PrideFlag {
            colors: vec![
                |s| s.bright_magenta().to_string(),
                |s| s.truecolor(196, 62, 142).to_string(),
                |s| s.bright_blue().to_string(),
            ],
        }),
        "pan" | "pansexual" => Some(PrideFlag {
            colors: vec![
                |s| s.bright_red().to_string(),
                |s| s.yellow().to_string(),
                |s| s.bright_cyan().to_string(),
            ],
        }),
        "ace" | "asexual" => Some(PrideFlag {
            colors: vec![
                |s| s.bright_black().to_string(),
                |s| s.bright_white().to_string(),
                |s| s.bright_magenta().to_string(),
                |s| s.white().to_string(),
            ],
        }),
        "aro" | "aromantic" => Some(PrideFlag {
            colors: vec![
                |s| s.truecolor(61, 165, 66).to_string(),
                |s| s.truecolor(167, 231, 69).to_string(),
                |s| s.white().to_string(),
                |s| s.bright_black().to_string(),
                |s| s.truecolor(40, 40, 40).to_string(),
            ],
        }),
        "genderfluid" => Some(PrideFlag {
            colors: vec![
                |s| s.bright_magenta().to_string(),
                |s| s.white().to_string(),
                |s| s.truecolor(150, 76, 150).to_string(),
                |s| s.bright_black().to_string(),
                |s| s.bright_blue().to_string(),
            ],
        }),
        "genderflux" => Some(PrideFlag {
            colors: vec![
                |s| s.bright_magenta().to_string(),
                |s| s.white().to_string(),
                |s| s.truecolor(150, 76, 150).to_string(),
                |s| s.bright_black().to_string(),
                |s| s.bright_blue().to_string(),
            ],
        }),
        "agender" => Some(PrideFlag {
            colors: vec![
                |s| s.bright_black().to_string(),
                |s| s.truecolor(192, 192, 192).to_string(),
                |s| s.white().to_string(),
                |s| s.truecolor(186, 225, 59).to_string(),
                |s| s.white().to_string(),
                |s| s.truecolor(192, 192, 192).to_string(),
                |s| s.bright_black().to_string(),
            ],
        }),
        "demiboy" => Some(PrideFlag {
            colors: vec![
                |s| s.bright_black().to_string(),
                |s| s.truecolor(100, 149, 237).to_string(),
                |s| s.bright_blue().to_string(),
                |s| s.white().to_string(),
            ],
        }),
        "demigirl" => Some(PrideFlag {
            colors: vec![
                |s| s.bright_black().to_string(),
                |s| s.bright_magenta().to_string(),
                |s| s.truecolor(255, 192, 203).to_string(),
                |s| s.white().to_string(),
            ],
        }),
        "bigender" => Some(PrideFlag {
            colors: vec![
                |s| s.bright_magenta().to_string(),
                |s| s.truecolor(192, 192, 192).to_string(),
                |s| s.bright_blue().to_string(),
            ],
        }),
        "genderqueer" => Some(PrideFlag {
            colors: vec![
                |s| s.bright_magenta().to_string(),
                |s| s.white().to_string(),
                |s| s.truecolor(186, 225, 59).to_string(),
            ],
        }),
        "femboy" => Some(PrideFlag {
            colors: vec![
                |s| s.truecolor(217, 71, 168).to_string(),
                |s| s.truecolor(240, 180, 220).to_string(),
                |s| s.white().to_string(),
                |s| s.bright_cyan().to_string(),
                |s| s.truecolor(240, 180, 220).to_string(),
                |s| s.truecolor(217, 71, 168).to_string(),
            ],
        }),
        "tomboy" => Some(PrideFlag {
            colors: vec![
                |s| s.bright_blue().to_string(),
                |s| s.truecolor(135, 206, 235).to_string(),
                |s| s.white().to_string(),
                |s| s.truecolor(255, 165, 0).to_string(),
                |s| s.bright_red().to_string(),
            ],
        }),
        "intersex" => Some(PrideFlag {
            colors: vec![
                |s| s.yellow().to_string(),
                |s| s.white().to_string(),
                |s| s.bright_magenta().to_string(),
            ],
        }),
        "demisexual" => Some(PrideFlag {
            colors: vec![
                |s| s.bright_black().to_string(),
                |s| s.bright_magenta().to_string(),
                |s| s.white().to_string(),
                |s| s.bright_magenta().to_string(),
                |s| s.bright_black().to_string(),
            ],
        }),
        "demiromantic" => Some(PrideFlag {
            colors: vec![
                |s| s.bright_black().to_string(),
                |s| s.bright_red().to_string(),
                |s| s.white().to_string(),
                |s| s.bright_red().to_string(),
                |s| s.bright_black().to_string(),
            ],
        }),
        "polygender" => Some(PrideFlag {
            colors: vec![
                |s| s.truecolor(255, 105, 180).to_string(),
                |s| s.truecolor(255, 165, 0).to_string(),
                |s| s.yellow().to_string(),
                |s| s.truecolor(186, 225, 59).to_string(),
                |s| s.bright_blue().to_string(),
            ],
        }),
        "polyamorous" => Some(PrideFlag {
            colors: vec![
                |s| s.bright_red().to_string(),
                |s| s.bright_blue().to_string(),
                |s| s.bright_black().to_string(),
            ],
        }),
        "omnisexual" => Some(PrideFlag {
            colors: vec![
                |s| s.bright_red().to_string(),
                |s| s.truecolor(255, 165, 0).to_string(),
                |s| s.yellow().to_string(),
                |s| s.truecolor(150, 76, 150).to_string(),
                |s| s.bright_magenta().to_string(),
            ],
        }),
        "queer" => Some(PrideFlag {
            colors: vec![
                |s| s.truecolor(176, 52, 139).to_string(),
                |s| s.white().to_string(),
                |s| s.truecolor(0, 200, 88).to_string(),
            ],
        }),
        "questioning" => Some(PrideFlag {
            colors: vec![
                |s| s.bright_magenta().to_string(),
                |s| s.truecolor(255, 165, 0).to_string(),
                |s| s.yellow().to_string(),
                |s| s.truecolor(192, 192, 192).to_string(),
            ],
        }),
        "two-spirit" => Some(PrideFlag {
            colors: vec![
                |s| s.bright_red().to_string(),
                |s| s.bright_yellow().to_string(),
            ],
        }),
        "demigender" => Some(PrideFlag {
            colors: vec![
                |s| s.bright_black().to_string(),
                |s| s.truecolor(169, 169, 169).to_string(),
                |s| s.white().to_string(),
                |s| s.truecolor(169, 169, 169).to_string(),
                |s| s.bright_black().to_string(),
            ],
        }),
        _ => None,
    };

    if let Some(flag) = flag {
        for (i, line) in ascii_art.iter().enumerate() {
            let color_index = (i * flag.colors.len()) / ascii_art.len();
            let color_fn = &flag.colors[color_index];
            println!("{}", color_fn(line));
        }
        println!("✩｡:*•.─────  ❁ ❁  ─────.•*:｡✩\n");
        
        let info_lines = vec![
            ("Host:", hostname),
            ("OS:", os),
            ("CPU:", cpu),
            ("Memory:", memory),
            ("Disk:", disk),
        ];
        
        for (i, (label, value)) in info_lines.iter().enumerate() {
            let color_index = (i * flag.colors.len()) / info_lines.len();
            let color_fn = &flag.colors[color_index];
            println!("{} {}", color_fn(label), value);
        }
    } else {
        println!("Unknown flag: {}. Available flags:", flag_name);
        println!("  --nonbinary, --boyflux, --girlflux, --trans, --lesbian, --gay");
        println!("  --bi, --bisexual, --pan, --pansexual, --ace, --asexual");
        println!("  --aro, --aromantic, --genderfluid, --genderflux, --agender");
        println!("  --demiboy, --demigirl, --bigender, --genderqueer, --demigender");
        println!("  --femboy, --tomboy, --intersex, --demisexual, --demiromantic");
        println!("  --polygender, --polyamorous, --omnisexual, --queer, --questioning");
        println!("  --two-spirit");
    }
}
