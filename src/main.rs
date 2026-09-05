use std::{env, fs};

macro_rules! pride_flags {
    ($($name:expr => $colors:expr),+ $(,)?) => {
        const PRIDE_FLAGS: &[(&str, &[(u8, u8, u8)])] = &[
            $(($name, $colors)),+
        ];

        const PRIDE_FLAG_NAMES: &[&str] = &[$($name),+];
    };
}

pride_flags!(
	"nonbinary" => &[(255, 255, 0), (255, 255, 255), (128, 0, 128), (64, 64, 64)],
	"boyflux" => &[(173, 216, 230), (65, 105, 225), (0, 35, 102), (144, 238, 144), (0, 35, 102), (173, 216, 230)],
	"girlflux" => &[(245, 222, 179), (255, 105, 180), (139, 0, 0), (210, 180, 140), (139, 0, 0), (255, 105, 180), (245, 222, 179)],
	"trans" => &[(0, 255, 255), (255, 0, 255), (255, 255, 255), (255, 0, 255), (0, 255, 255)],
	"lesbian" => &[(255, 127, 0), (255, 168, 126), (255, 255, 255), (255, 0, 255), (191, 21, 107)],
	"gay" => &[(255, 0, 0), (255, 165, 0), (255, 255, 0), (0, 128, 0), (0, 0, 255), (148, 0, 211)],
	"bi" => &[(255, 0, 255), (196, 62, 142), (0, 0, 255)],
	"bisexual" => &[(255, 0, 255), (196, 62, 142), (0, 0, 255)],
	"pan" => &[(255, 0, 0), (255, 255, 0), (0, 255, 255)],
	"pansexual" => &[(255, 0, 0), (255, 255, 0), (0, 255, 255)],
	"ace" => &[(64, 64, 64), (255, 255, 255), (255, 0, 255), (255, 255, 255)],
	"asexual" => &[(64, 64, 64), (255, 255, 255), (255, 0, 255), (255, 255, 255)],
	"aro" => &[(61, 165, 66), (167, 231, 69), (255, 255, 255), (64, 64, 64), (40, 40, 40)],
	"aromantic" => &[(61, 165, 66), (167, 231, 69), (255, 255, 255), (64, 64, 64), (40, 40, 40)],
	"genderfluid" => &[(255, 0, 255), (255, 255, 255), (150, 76, 150), (64, 64, 64), (0, 0, 255)],
	"genderflux" => &[(255, 0, 255), (255, 255, 255), (150, 76, 150), (64, 64, 64), (0, 0, 255)],
	"agender" => &[(64, 64, 64), (192, 192, 192), (255, 255, 255), (186, 225, 59), (255, 255, 255), (192, 192, 192), (64, 64, 64)],
	"demiboy" => &[(64, 64, 64), (100, 149, 237), (0, 0, 255), (255, 255, 255)],
	"demigirl" => &[(64, 64, 64), (255, 0, 255), (255, 192, 203), (255, 255, 255)],
	"bigender" => &[(255, 0, 255), (192, 192, 192), (0, 0, 255)],
	"genderqueer" => &[(255, 0, 255), (255, 255, 255), (186, 225, 59)],
	"femboy" => &[(217, 71, 168), (240, 180, 220), (255, 255, 255), (0, 255, 255), (240, 180, 220), (217, 71, 168)],
	"tomboy" => &[(0, 0, 255), (135, 206, 235), (255, 255, 255), (255, 165, 0), (255, 0, 0)],
	"intersex" => &[(255, 255, 0), (255, 255, 255), (255, 0, 255)],
	"demisexual" => &[(64, 64, 64), (255, 0, 255), (255, 255, 255), (255, 0, 255), (64, 64, 64)],
	"demiromantic" => &[(64, 64, 64), (255, 0, 0), (255, 255, 255), (255, 0, 0), (64, 64, 64)],
	"polygender" => &[(255, 105, 180), (255, 165, 0), (255, 255, 0), (186, 225, 59), (0, 0, 255)],
	"polyamorous" => &[(255, 0, 0), (0, 0, 255), (64, 64, 64)],
	"omnisexual" => &[(255, 0, 0), (255, 165, 0), (255, 255, 0), (150, 76, 150), (255, 0, 255)],
	"queer" => &[(176, 52, 139), (255, 255, 255), (0, 200, 88)],
	"questioning" => &[(255, 0, 255), (255, 165, 0), (255, 255, 0), (192, 192, 192)],
	"two-pirit" => &[(255, 0, 0), (255, 255, 0)],
	"demigender" => &[(64, 64, 64), (169, 169, 169), (255, 255, 255), (169, 169, 169), (64, 64, 64)],
);

fn get_kernel() -> String {
	std::process::Command::new("uname")
		.arg("-r")
		.output()
		.ok()
		.and_then(|output| String::from_utf8(output.stdout).ok())
		.map(|s| s.trim().to_string())
		.unwrap_or_else(|| "Unknown".to_string())
}

fn get_gpu() -> String {
	std::process::Command::new("lspci")
		.output()
		.ok()
		.and_then(|output| String::from_utf8(output.stdout).ok())
		.and_then(|output| {
			output
				.lines()
				.find(|line| line.contains("VGA") || line.contains("3D"))
				.and_then(|line| {
					let gpu = line.split(": ").nth(1).unwrap_or("Unknown").trim();

					// Make AMD GPU names shorter and prettier
					if gpu.contains("Radeon 610M") {
						Some("Radeon 610M".to_string())
					} else if gpu.contains("Radeon") {
						Some(gpu.split('[').next().unwrap_or(gpu).trim().to_string())
					} else {
						Some(gpu.to_string())
					}
				})
		})
		.unwrap_or_else(|| "Unknown".to_string())
}

fn get_uptime() -> String {
	fs::read_to_string("/proc/uptime")
		.ok()
		.and_then(|content| {
			content
				.split_whitespace()
				.next()
				.and_then(|s| s.parse::<f64>().ok())
		})
		.map(|seconds| {
			let days = (seconds / 86400.0) as u32;
			let hours = ((seconds % 86400.0) / 3600.0) as u32;
			let mins = ((seconds % 3600.0) / 60.0) as u32;
			if days > 0 {
				format!("{}d {}h {}m", days, hours, mins)
			} else {
				format!("{}h {}m", hours, mins)
			}
		})
		.unwrap_or_else(|| "Unknown".to_string())
}

fn get_shell() -> String {
	env::var("SHELL")
		.ok()
		.and_then(|path| path.split('/').last().map(|s| s.to_string()))
		.unwrap_or_else(|| "Unknown".to_string())
}

fn get_terminal() -> String {
	env::var("TERM").unwrap_or_else(|_| "Unknown".to_string())
}

fn get_de() -> String {
	env::var("XDG_CURRENT_DESKTOP")
		.or_else(|_| env::var("DESKTOP_SESSION"))
		.unwrap_or_else(|_| "Unknown".to_string())
}

fn get_packages() -> String {
	let managers: [(&str, &[&str]); 4] = [
		("pacman", &["-Q"]),
		("dpkg", &["-l"]),
		("rpm", &["-qa"]),
		("nix", &["profile", "list"]),
	];

	for (cmd, args) in managers {
		if let Ok(output) = std::process::Command::new(cmd).args(args).output() {
			let count = String::from_utf8_lossy(&output.stdout).lines().count();
			if count > 0 {
				return format!("{} ({})", count, cmd);
			}
		}
	}
	"Unknown".to_string()
}

fn get_locale() -> String {
	env::var("LANG").unwrap_or_else(|_| "Unknown".to_string())
}

fn get_gtk_theme() -> String {
	fs::read_to_string(format!(
		"{}/.config/gtk-3.0/settings.ini",
		env::var("HOME").unwrap_or_default()
	))
	.ok()
	.and_then(|content| {
		content
			.lines()
			.find(|line| line.contains("gtk-theme-name"))
			.and_then(|line| line.split('=').nth(1).map(|s| s.trim().to_string()))
	})
	.unwrap_or_else(|| "Unknown".to_string())
}

fn main() {
	let hostname = fs::read_to_string("/etc/hostname")
		.unwrap_or_default()
		.trim()
		.to_string();

	let os = fs::read_to_string("/etc/os-release")
		.unwrap_or_default()
		.lines()
		.find(|line| line.starts_with("PRETTY_NAME="))
		.and_then(|line| line.split('=').nth(1))
		.map(|s| s.trim_matches('"').to_string())
		.unwrap_or_else(|| "Unknown".to_string());

	let kernel = get_kernel();

	let cpu = fs::read_to_string("/proc/cpuinfo")
		.unwrap_or_default()
		.lines()
		.find(|line| line.starts_with("model name"))
		.and_then(|line| line.split(':').nth(1))
		.map(|s| s.trim().to_string())
		.unwrap_or_else(|| "Unknown".to_string());

	let gpu = get_gpu();

	let meminfo = fs::read_to_string("/proc/meminfo").unwrap_or_default();
	let mem_total: u64 = meminfo
		.lines()
		.find(|line| line.starts_with("MemTotal"))
		.and_then(|line| line.split_whitespace().nth(1))
		.and_then(|s| s.parse().ok())
		.unwrap_or(0);
	let memory = format!("~{} GiB", mem_total / 1024 / 1024);

	let disk = std::process::Command::new("df")
		.args(&["-h", "/"])
		.output()
		.ok()
		.and_then(|output| {
			let s = String::from_utf8_lossy(&output.stdout);
			s.lines()
				.nth(1)
				.and_then(|line| line.split_whitespace().nth(1).map(|s| format!("~{}", s)))
		})
		.unwrap_or_else(|| "Unknown".to_string());

	let uptime = get_uptime();
	let shell = get_shell();
	let terminal = get_terminal();
	let de = get_de();
	let packages = get_packages();
	let locale = get_locale();
	let theme = get_gtk_theme();

	let args: Vec<String> = env::args().collect();

	let ascii_art = [
		"⠀⠀⣠⡶⢶⣦⠀⠀⠀⣠⡶⢶⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
		"⠀⢰⡟⠀⠀⢹⣧⠀⣸⠏⠀⠀⢻⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
		"⠀⣿⠁⠀⠀⠀⢿⣴⡿⠀⠀⠀⢸⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
		"⠀⣿⠀⠀⠀⠀⢸⣿⠇⠀⠀⠀⢸⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
		"⠀⢿⡆⠀⠀⠀⠈⣿⠀⠀⠀⠀⣸⡇⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⡀⠀⠀⠀⠀⠀",
		"⠀⢸⣷⠀⠀⠀⠀⠀⠀⠀⠀⢰⣟⠀⠀⠀⣀⣀⣀⣀⣀⣾⠋⠉⠹⣇⠀⠀⠀⠀",
		"⠀⣰⡿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⣧⣶⠞⠋⠉⠀⠈⠉⠃⠀⠀⢠⡟⠀⠀⠀⠀",
		"⢠⡿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⣦⡀⠀⠀",
		"⣸⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢘⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢷⣄⠀",
		"⢿⡇⠸⣿⠀⠀⠀⠀⠀⠀⣴⣆⠀⢸⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⡆",
		"⠸⣧⡀⠀⠀⢀⣶⣶⡆⠀⠈⠁⣰⡟⠁⠀⠀⠀⠀⠀⠀⠀⢀⣀⠀⠀⠀⠀⣸⡇",
		"⠀⠙⠻⣦⣄⣀⣀⣈⣁⣀⣤⠾⠋⠀⠀⠀⠀⠀⣀⣠⣴⡶⢿⡿⠿⠶⣶⠶⠟⠀",
		"⠀⠀⢠⡟⠉⢙⣿⠛⠋⠉⠁⠀⠀⣀⣠⣴⠶⠟⠛⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
		"⠀⠀⠘⢿⣤⣘⣿⡀⠀⠀⢀⣴⡿⠋⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
		"⠀⠀⠀⠀⠈⠉⠙⠛⠻⠿⠛⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
	];

	let info_lines = [
		("Host:", hostname),
		("OS:", os),
		("Kernel:", kernel),
		("CPU:", cpu),
		("GPU:", gpu),
		("Memory:", memory),
		("Disk:", disk),
		("Uptime:", uptime),
		("Shell:", shell),
		("Terminal:", terminal),
		("DE:", de),
		("Packages:", packages),
		("Locale:", locale),
		("Theme:", theme),
	];

	// no flag coloring needed
	if args.len() < 2 {
		for (i, line) in ascii_art.iter().enumerate() {
			if i < info_lines.len() {
				let (label, value) = &info_lines[i];
				println!("{}  {} {}", line, label, value);
			} else {
				println!("{}", line);
			}
		}
		return;
	}

	let flag = &args[1].to_lowercase();
	let flag_name = flag.trim_start_matches("--");

	if let Some(colors) = get_pride_flag(flag_name) {
		for (i, line) in ascii_art.iter().enumerate() {
			let (r, g, b) = colors[i * colors.len() / ascii_art.len()];
			let colored_line = format!("\x1b[38;2;{r};{g};{b}m{line}");

			if i < info_lines.len() {
				let (label, value) = &info_lines[i];
				println!("{colored_line}  {label}\x1b[0m {value}");
			} else {
				println!("{colored_line}\x1b[0m");
			}
		}
	} else {
		println!("Unknown flag: {flag_name}. Available flags:");

		let mut output = String::new();
		for (i, name) in PRIDE_FLAG_NAMES.iter().enumerate() {
			if i % 5 == 0 {
				if i > 0 {
					output.push('\n');
				}
				output.push('\t');
			} else {
				output.push_str(", ");
			}
			output.push_str("--");
			output.push_str(name);
		}

		println!("{output}");
	}
}

fn get_pride_flag(name: &str) -> Option<&&'static [(u8, u8, u8)]> {
	PRIDE_FLAGS
		.iter()
		.find(|(flag_name, _)| flag_name == &name)
		.map(|(_, colors)| colors)
}
