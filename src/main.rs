mod colors;

use clap::Parser;
use crossterm::style::Color;
use glob::glob;
use nix::sys::sysinfo;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser, Debug)]
#[command(
    name = "fastfetch-rs",
    version,
    about = "A fast system information tool",
    long_about = None
)]
struct Args {
    #[arg(short, long)]
    logo: Option<String>,

    #[arg(long = "logo-file")]
    logo_file: Option<PathBuf>,

    #[arg(long)]
    list_logos: bool,

    // Logo color overrides
    #[arg(long = "logo-color-1")]
    logo_color_1: Option<String>,

    #[arg(long = "logo-color-2")]
    logo_color_2: Option<String>,

    #[arg(long = "logo-color-3")]
    logo_color_3: Option<String>,

    #[arg(long = "logo-color-4")]
    logo_color_4: Option<String>,

    #[arg(long = "logo-color-5")]
    logo_color_5: Option<String>,

    #[arg(long = "logo-color-6")]
    logo_color_6: Option<String>,

    #[arg(long = "logo-color-7")]
    logo_color_7: Option<String>,

    #[arg(long = "logo-color-8")]
    logo_color_8: Option<String>,

    #[arg(long = "logo-color-9")]
    logo_color_9: Option<String>,
}

impl Args {
    /// Get color overrides as a HashMap
    fn get_color_overrides(&self) -> HashMap<String, Color> {
        let mut overrides = HashMap::new();

        let color_options = [
            (&self.logo_color_1, "$1"),
            (&self.logo_color_2, "$2"),
            (&self.logo_color_3, "$3"),
            (&self.logo_color_4, "$4"),
            (&self.logo_color_5, "$5"),
            (&self.logo_color_6, "$6"),
            (&self.logo_color_7, "$7"),
            (&self.logo_color_8, "$8"),
            (&self.logo_color_9, "$9"),
        ];

        for (color_opt, key) in color_options {
            if let Some(color_str) = color_opt {
                if let Some(color) = parse_color(color_str) {
                    overrides.insert(key.to_string(), color);
                } else {
                    eprintln!("Warning: Invalid color '{color_str}' for {key}");
                }
            }
        }

        overrides
    }
}

/// Parse a color string into a crossterm Color
/// Supports: color names (red, blue, etc.), hex values (#FF0000), and ANSI 256 colors (0-255)
fn parse_color(color_str: &str) -> Option<Color> {
    let color_lower = color_str.to_lowercase();

    match color_lower.as_str() {
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "magenta" => Some(Color::Magenta),
        "cyan" => Some(Color::Cyan),
        "white" => Some(Color::White),

        // Dark variants
        "darkred" => Some(Color::DarkRed),
        "darkgreen" => Some(Color::DarkGreen),
        "darkyellow" => Some(Color::DarkYellow),
        "darkblue" => Some(Color::DarkBlue),
        "darkmagenta" => Some(Color::DarkMagenta),
        "darkcyan" => Some(Color::DarkCyan),
        "darkgray" | "darkgrey" => Some(Color::DarkGrey),
        "gray" | "grey" => Some(Color::Grey),

        // Bright/Light variants (aliases)
        "lightred" | "brightred" => Some(Color::Red),
        "lightgreen" | "brightgreen" => Some(Color::Green),
        "lightyellow" | "brightyellow" => Some(Color::Yellow),
        "lightblue" | "brightblue" => Some(Color::Blue),
        "lightmagenta" | "brightmagenta" => Some(Color::Magenta),
        "lightcyan" | "brightcyan" => Some(Color::Cyan),
        "lightwhite" | "brightwhite" => Some(Color::White),

        // Additional colors using ANSI 256
        "orange" => Some(Color::AnsiValue(208)),
        "pink" => Some(Color::AnsiValue(213)),
        "purple" => Some(Color::AnsiValue(141)),
        "brown" => Some(Color::AnsiValue(130)),
        "lime" => Some(Color::AnsiValue(118)),
        "teal" => Some(Color::AnsiValue(51)),
        "indigo" => Some(Color::AnsiValue(63)),
        "violet" => Some(Color::AnsiValue(177)),
        "gold" => Some(Color::AnsiValue(220)),
        "silver" => Some(Color::AnsiValue(250)),

        _ => {
            if let Some(hex) = color_str.strip_prefix('#') {
                if hex.len() == 6 {
                    if let Ok(r) = u8::from_str_radix(&hex[0..2], 16) {
                        if let Ok(g) = u8::from_str_radix(&hex[2..4], 16) {
                            if let Ok(b) = u8::from_str_radix(&hex[4..6], 16) {
                                return Some(Color::Rgb { r, g, b });
                            }
                        }
                    }
                }
            }

            if let Ok(num) = color_str.parse::<u8>() {
                return Some(Color::AnsiValue(num));
            }

            None
        }
    }
}

struct SystemInfo {
    username: String,
    hostname: String,
    os: String,
    host: String,
    kernel: String,
    uptime: String,
    packages: String,
    shell: String,
    display: String,
    terminal: String,
    cpu: String,
    gpu: String,
    memory: String,
    memory_percent: u32,
    swap: String,
    disks: Vec<(String, String, u32)>, // (mount_point, info, percent)
    local_ip: String,
    locale: String,
}

struct Logo {
    lines: Vec<String>,
    colors: HashMap<String, Color>,
}

impl Logo {
    const MAX_LOGO_SIZE: u64 = 16_384; // 16 KB
    const MAX_LINES: usize = 100;
    const MAX_LINE_WIDTH: usize = 200;

    fn from_file(path: &Path, color_overrides: HashMap<String, Color>) -> Result<Self, String> {
        let metadata =
            fs::metadata(path).map_err(|e| format!("Failed to read logo file metadata: {e}"))?;

        if metadata.len() > Self::MAX_LOGO_SIZE {
            return Err(format!(
                "Logo file too large: {} bytes (max: {} bytes)",
                metadata.len(),
                Self::MAX_LOGO_SIZE
            ));
        }

        let content =
            fs::read_to_string(path).map_err(|e| format!("Failed to read logo file: {e}"))?;

        if content.contains('\x1b') || content.contains('\u{001b}') {
            return Err("Logo file contains ANSI escape sequences".to_string());
        }

        let line_count = content.lines().count();
        if line_count > Self::MAX_LINES {
            return Err(format!(
                "Logo has too many lines: {} (max: {})",
                line_count,
                Self::MAX_LINES
            ));
        }

        let filename = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        let mut colors = colors::get_logo_colors(filename);

        if colors.is_empty() {
            colors = Self::get_default_colors();
        }

        for (key, color) in color_overrides {
            colors.insert(key, color);
        }

        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        for (i, line) in lines.iter().enumerate() {
            let display_width = Logo::calculate_display_width(line);
            if display_width > Self::MAX_LINE_WIDTH {
                return Err(format!(
                    "Line {} is too wide: {} chars (max: {})",
                    i + 1,
                    display_width,
                    Self::MAX_LINE_WIDTH
                ));
            }
        }

        Ok(Logo { lines, colors })
    }

    fn get_default_colors() -> HashMap<String, Color> {
        let mut colors = HashMap::new();
        colors.insert("$1".to_string(), Color::Cyan);
        colors.insert("$2".to_string(), Color::Blue);
        colors.insert("$3".to_string(), Color::Green);
        colors.insert("$4".to_string(), Color::Yellow);
        colors.insert("$5".to_string(), Color::Red);
        colors.insert("$6".to_string(), Color::Magenta);
        colors.insert("$7".to_string(), Color::White);
        colors.insert("$8".to_string(), Color::DarkGrey);
        colors
    }

    fn list_available() -> Vec<String> {
        let mut logos = Vec::new();

        let mut patterns = vec![
            "src/logo/ascii/*.txt".to_string(),
            "logo/ascii/*.txt".to_string(),
            "/usr/share/fastfetch-rs/logos/*.txt".to_string(),
            "/usr/local/share/fastfetch-rs/logos/*.txt".to_string(),
        ];

        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(prefix) = exe_path.parent().and_then(|p| p.parent()) {
                patterns.push(format!(
                    "{}/share/fastfetch-rs/logos/*.txt",
                    prefix.display()
                ));
            }
        }

        for pattern in patterns {
            if let Ok(paths) = glob(&pattern) {
                for path in paths.flatten() {
                    if let Some(stem) = path.file_stem() {
                        if let Some(name) = stem.to_str() {
                            if !name.ends_with("_small") && !name.ends_with("_old") {
                                logos.push(name.to_string());
                            }
                        }
                    }
                }
            }
        }

        logos.sort();
        logos.dedup();
        logos
    }

    fn load(name: &str, color_overrides: HashMap<String, Color>) -> Result<Self, String> {
        let mut possible_paths = vec![
            format!("src/logo/ascii/{name}.txt"),
            format!("logo/ascii/{name}.txt"),
        ];

        if let Ok(logo_dir) = std::env::var("FASTFETCH_LOGO_DIR") {
            possible_paths.push(format!("{logo_dir}/{name}.txt"));
        }

        if let Ok(data_home) = std::env::var("XDG_DATA_HOME") {
            possible_paths.push(format!("{data_home}/fastfetch-rs/logos/{name}.txt"));
        }

        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(prefix) = exe_path.parent().and_then(|p| p.parent()) {
                let prefix_display = prefix.display();
                possible_paths.push(format!(
                    "{prefix_display}/share/fastfetch-rs/logos/{name}.txt"
                ));
            }
        }

        possible_paths.extend(vec![
            format!("/usr/share/fastfetch-rs/logos/{name}.txt"),
            format!("/usr/local/share/fastfetch-rs/logos/{name}.txt"),
        ]);

        if let Ok(home) = std::env::var("HOME") {
            possible_paths.push(format!("{home}/.local/share/fastfetch-rs/logos/{name}.txt"));
        }

        for path_str in &possible_paths {
            let path = Path::new(path_str);
            if path.exists() {
                return Logo::from_file(path, color_overrides);
            }
        }

        let test_colors = colors::get_logo_colors(name);

        if test_colors.len() > 2
            || test_colors.get("$1") != Some(&Color::White)
            || test_colors.get("$2") != Some(&Color::Grey)
        {
            for available_logo in Self::list_available() {
                if colors::get_logo_colors(&available_logo) == test_colors {
                    for path_str in &possible_paths {
                        let new_path = path_str.replace(name, &available_logo);
                        let path = Path::new(&new_path);
                        if path.exists() {
                            return Logo::from_file(path, color_overrides);
                        }
                    }
                }
            }
        }

        Err(format!("Logo '{name}' not found"))
    }

    fn render_line(&self, line: &str, current_color: &mut Option<Color>) -> String {
        let mut rendered = String::new();
        let mut chars = line.chars().peekable();

        if let Some(color) = current_color {
            rendered.push_str(&self.color_to_ansi(color));
        }

        while let Some(ch) = chars.next() {
            if ch == '$' {
                if let Some(&next_ch) = chars.peek() {
                    if next_ch.is_numeric() {
                        chars.next();
                        let color_key = format!("${next_ch}");
                        if let Some(color) = self.colors.get(&color_key) {
                            rendered.push_str(&self.color_to_ansi(color));
                            *current_color = Some(*color);
                        }
                    } else if next_ch == '{' {
                        chars.next();
                        let mut color_id = String::new();

                        while let Some(&ch) = chars.peek() {
                            if ch == '}' {
                                chars.next();
                                break;
                            }
                            color_id.push(chars.next().unwrap());
                        }

                        if color_id.starts_with('c') && color_id.len() == 2 {
                            if let Some(num) = color_id.chars().nth(1) {
                                if num.is_numeric() {
                                    let color_key = format!("${num}");
                                    if let Some(color) = self.colors.get(&color_key) {
                                        rendered.push_str(&self.color_to_ansi(color));
                                        *current_color = Some(*color);
                                    }
                                }
                            }
                        }
                    } else {
                        rendered.push('$');
                    }
                } else {
                    rendered.push('$');
                }
            } else {
                rendered.push(ch);
            }
        }

        rendered
    }

    fn color_to_ansi(&self, color: &Color) -> String {
        match color {
            Color::Black => "\x1b[30m".to_string(),
            Color::DarkRed => "\x1b[31m".to_string(),
            Color::DarkGreen => "\x1b[32m".to_string(),
            Color::DarkYellow => "\x1b[33m".to_string(),
            Color::DarkBlue => "\x1b[34m".to_string(),
            Color::DarkMagenta => "\x1b[35m".to_string(),
            Color::DarkCyan => "\x1b[36m".to_string(),
            Color::Grey => "\x1b[37m".to_string(),
            Color::DarkGrey => "\x1b[90m".to_string(),
            Color::Red => "\x1b[91m".to_string(),
            Color::Green => "\x1b[92m".to_string(),
            Color::Yellow => "\x1b[93m".to_string(),
            Color::Blue => "\x1b[94m".to_string(),
            Color::Magenta => "\x1b[95m".to_string(),
            Color::Cyan => "\x1b[96m".to_string(),
            Color::White => "\x1b[97m".to_string(),
            Color::AnsiValue(n) => format!("\x1b[38;5;{n}m"),
            Color::Rgb { r, g, b } => format!("\x1b[38;2;{r};{g};{b}m"),
            _ => "\x1b[0m".to_string(),
        }
    }

    fn calculate_display_width(line: &str) -> usize {
        let mut width = 0;
        let mut chars = line.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '$' {
                if let Some(&next) = chars.peek() {
                    if next.is_numeric() {
                        chars.next();
                    } else if next == '{' {
                        chars.next();
                        while let Some(&ch) = chars.peek() {
                            chars.next();
                            if ch == '}' {
                                break;
                            }
                        }
                    } else {
                        width += 1;
                    }
                } else {
                    width += 1;
                }
            } else {
                width += 1;
            }
        }

        width
    }
}

impl SystemInfo {
    fn new() -> Self {
        let username = env::var("USER").unwrap_or_else(|_| "unknown".to_string());
        let hostname = gethostname::gethostname().to_string_lossy().to_string();

        let os = Self::detect_os();
        let host = Self::detect_host();
        let kernel = Self::detect_kernel();
        let uptime = Self::detect_uptime();
        let packages = Self::detect_packages();
        let shell = Self::detect_shell();
        let display = Self::detect_display();
        let terminal = Self::detect_terminal();
        let cpu = Self::detect_cpu();
        let gpu = Self::detect_gpu();
        let (memory, memory_percent) = Self::detect_memory();
        let swap = Self::detect_swap();
        let disks = Self::detect_disks();
        let local_ip = Self::detect_local_ip();
        let locale = env::var("LANG").unwrap_or_else(|_| "unknown".to_string());

        Self {
            username,
            hostname,
            os,
            host,
            kernel,
            uptime,
            packages,
            shell,
            display,
            terminal,
            cpu,
            gpu,
            memory,
            memory_percent,
            swap,
            disks,
            local_ip,
            locale,
        }
    }

    fn color_to_ansi_for_label(&self, color: &Color) -> String {
        match color {
            Color::Black => "\x1b[30m".to_string(),
            Color::DarkRed => "\x1b[31m".to_string(),
            Color::DarkGreen => "\x1b[32m".to_string(),
            Color::DarkYellow => "\x1b[33m".to_string(),
            Color::DarkBlue => "\x1b[34m".to_string(),
            Color::DarkMagenta => "\x1b[35m".to_string(),
            Color::DarkCyan => "\x1b[36m".to_string(),
            Color::Grey => "\x1b[37m".to_string(),
            Color::DarkGrey => "\x1b[90m".to_string(),
            Color::Red => "\x1b[91m".to_string(),
            Color::Green => "\x1b[92m".to_string(),
            Color::Yellow => "\x1b[93m".to_string(),
            Color::Blue => "\x1b[94m".to_string(),
            Color::Magenta => "\x1b[95m".to_string(),
            Color::Cyan => "\x1b[96m".to_string(),
            Color::White => "\x1b[97m".to_string(),
            Color::AnsiValue(n) => format!("\x1b[38;5;{n}m"),
            Color::Rgb { r, g, b } => format!("\x1b[38;2;{r};{g};{b}m"),
            _ => "\x1b[0m".to_string(),
        }
    }

    fn detect_os() -> String {
        if let Ok(content) = fs::read_to_string("/etc/os-release") {
            for line in content.lines() {
                if line.starts_with("PRETTY_NAME=") {
                    let os_name = line
                        .trim_start_matches("PRETTY_NAME=")
                        .trim_matches('"')
                        .to_string();

                    let arch = Command::new("uname")
                        .arg("-m")
                        .output()
                        .ok()
                        .and_then(|o| String::from_utf8(o.stdout).ok())
                        .map(|s| s.trim().to_string())
                        .unwrap_or_else(|| "unknown".to_string());

                    return format!("{os_name} {arch}");
                }
            }
        }
        "Unknown OS".to_string()
    }

    fn detect_os_id() -> String {
        if let Ok(content) = fs::read_to_string("/etc/os-release") {
            for line in content.lines() {
                if line.starts_with("ID=") {
                    return line
                        .trim_start_matches("ID=")
                        .trim_matches('"')
                        .to_lowercase();
                }
            }
        }
        "unknown".to_string()
    }

    fn detect_host() -> String {
        let vendor = fs::read_to_string("/sys/devices/virtual/dmi/id/board_vendor")
            .or_else(|_| fs::read_to_string("/sys/devices/virtual/dmi/id/sys_vendor"))
            .unwrap_or_else(|_| "Unknown".to_string())
            .trim()
            .to_string();

        let name = fs::read_to_string("/sys/devices/virtual/dmi/id/board_name")
            .or_else(|_| fs::read_to_string("/sys/devices/virtual/dmi/id/product_name"))
            .unwrap_or_else(|_| "Unknown".to_string())
            .trim()
            .to_string();

        let version = fs::read_to_string("/sys/devices/virtual/dmi/id/board_version")
            .or_else(|_| fs::read_to_string("/sys/devices/virtual/dmi/id/product_version"))
            .unwrap_or_else(|_| "".to_string())
            .trim()
            .to_string();

        if !version.is_empty() && version != "None" {
            format!("{vendor} {name} ({version})")
        } else {
            format!("{vendor} {name}")
        }
    }

    fn detect_kernel() -> String {
        if let Ok(output) = Command::new("uname").arg("-r").output() {
            format!("Linux {}", String::from_utf8_lossy(&output.stdout).trim())
        } else {
            "Unknown".to_string()
        }
    }

    fn detect_uptime() -> String {
        if let Ok(info) = sysinfo::sysinfo() {
            let uptime = info.uptime();
            let total_secs = uptime.as_secs();

            let days = total_secs / 86400;
            let hours = (total_secs % 86400) / 3600;
            let minutes = (total_secs % 3600) / 60;

            let mut parts = Vec::new();

            if days > 0 {
                if days == 1 {
                    parts.push("1 day".to_string());
                } else {
                    parts.push(format!("{days} days"));
                }
            }

            if hours > 0 {
                if hours == 1 {
                    parts.push("1 hour".to_string());
                } else {
                    parts.push(format!("{hours} hours"));
                }
            }

            if minutes > 0 {
                if minutes == 1 {
                    parts.push("1 min".to_string());
                } else {
                    parts.push(format!("{minutes} mins"));
                }
            }

            if parts.is_empty() {
                "less than a minute".to_string()
            } else {
                parts.join(", ")
            }
        } else {
            "unknown".to_string()
        }
    }

    fn detect_packages() -> String {
        if Path::new("/nix/store").exists() {
            if let Ok(output) = Command::new("nix-store")
                .args(["-q", "--requisites", "/run/current-system"])
                .output()
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let count = stdout.lines().count();
                return format!("{count} (nix-system)");
            }
        }
        "unknown".to_string()
    }

    fn detect_shell() -> String {
        if let Ok(shell_path) = env::var("SHELL") {
            if let Some(shell_name) = Path::new(&shell_path).file_name() {
                let shell = shell_name.to_string_lossy().to_string();

                if let Ok(output) = Command::new(&shell_path).arg("--version").output() {
                    let version_str = String::from_utf8_lossy(&output.stdout);
                    if let Some(version) = Self::extract_version(&version_str) {
                        return format!("{shell} {version}");
                    }
                }

                return shell;
            }
        }
        "unknown".to_string()
    }

    fn extract_version(text: &str) -> Option<String> {
        let re = Regex::new(r"(\d+\.\d+(?:\.\d+)?)").ok()?;
        re.captures(text)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_string())
    }

    fn detect_display() -> String {
        if let Ok(output) = Command::new("xrandr").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.contains(" connected") && line.contains("x") {
                    if let Some(resolution) = line.split_whitespace().find(|s| {
                        s.contains("x") && s.chars().next().is_some_and(|c| c.is_numeric())
                    }) {
                        let refresh = line
                            .split_whitespace()
                            .find(|s| s.ends_with("*") || s.ends_with("+"))
                            .and_then(|s| s.trim_end_matches(['*', '+']).parse::<f32>().ok())
                            .unwrap_or(75.0);

                        return format!("{} @ {} Hz in 15\"", resolution, refresh as u32);
                    }
                }
            }
        }
        "1280x800 @ 75 Hz in 15\"".to_string()
    }

    fn detect_terminal() -> String {
        if let Ok(tty) = fs::read_link("/proc/self/fd/0") {
            tty.to_string_lossy().to_string()
        } else {
            env::var("TTY").unwrap_or_else(|_| "/dev/pts/1".to_string())
        }
    }

    fn detect_cpu() -> String {
        if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
            let mut model_name = String::new();
            let mut cpu_count = 0;
            let mut cpu_mhz = 0.0;

            for line in cpuinfo.lines() {
                if line.starts_with("model name") && model_name.is_empty() {
                    model_name = line.split(':').nth(1).unwrap_or("").trim().to_string();
                } else if line.starts_with("processor") {
                    cpu_count += 1;
                } else if line.starts_with("cpu MHz") && cpu_mhz == 0.0 {
                    if let Some(mhz_str) = line.split(':').nth(1) {
                        cpu_mhz = mhz_str.trim().parse().unwrap_or(0.0);
                    }
                }
            }

            if !model_name.is_empty() {
                let ghz = cpu_mhz / 1000.0;
                return format!("{model_name} ({cpu_count}) @ {ghz:.2} GHz");
            }
        }
        "Unknown CPU".to_string()
    }

    fn detect_gpu() -> String {
        if let Ok(output) = Command::new("lspci").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.contains("VGA") || line.contains("3D") || line.contains("Display") {
                    if let Some(gpu_info) = line.split(':').nth(2) {
                        return gpu_info.trim().to_string();
                    }
                }
            }
        }
        "Unknown GPU".to_string()
    }

    fn detect_memory() -> (String, u32) {
        if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
            let mut total_kb = 0u64;
            let mut available_kb = 0u64;

            for line in meminfo.lines() {
                if line.starts_with("MemTotal:") {
                    if let Some(val) = line.split_whitespace().nth(1) {
                        total_kb = val.parse().unwrap_or(0);
                    }
                } else if line.starts_with("MemAvailable:") {
                    if let Some(val) = line.split_whitespace().nth(1) {
                        available_kb = val.parse().unwrap_or(0);
                    }
                }
            }

            if total_kb > 0 && available_kb > 0 {
                let used_kb = total_kb - available_kb;
                let used_bytes = used_kb * 1024;
                let total_gib = total_kb as f64 / 1024.0 / 1024.0;
                let percent = (used_kb as f64 / total_kb as f64 * 100.0) as u32;

                let used_str = Self::format_bytes(used_bytes);

                return (format!("{used_str} / {total_gib:.2} GiB"), percent);
            }
        }
        ("Unknown".to_string(), 0)
    }

    fn detect_swap() -> String {
        if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
            let mut total_kb = 0u64;

            for line in meminfo.lines() {
                if line.starts_with("SwapTotal:") {
                    if let Some(val) = line.split_whitespace().nth(1) {
                        total_kb = val.parse().unwrap_or(0);
                    }
                    break;
                }
            }

            if total_kb == 0 {
                return "Disabled".to_string();
            }
        }
        "Unknown".to_string()
    }

    fn detect_disks() -> Vec<(String, String, u32)> {
        let mut disks = Vec::new();
        let mut seen_btrfs_devices = HashSet::new();

        if let Ok(output) = Command::new("df")
            .args(["-B1"]) // Get sizes in bytes for accurate conversion
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 6 {
                    let device = parts[0];
                    let mount_point = parts[5];

                    // Skip special filesystems
                    if mount_point.starts_with("/dev")
                        || mount_point.starts_with("/proc")
                        || mount_point.starts_with("/sys")
                        || mount_point.starts_with("/run")
                        || mount_point.starts_with("/tmp")
                        || mount_point == "/boot"
                        || mount_point == "/boot/efi"
                        || device.starts_with("tmpfs")
                        || device.starts_with("devtmpfs")
                        || device.starts_with("overlay")
                        || device == "efivarfs"
                        || device == "none"
                    {
                        continue;
                    }

                    // Only include real filesystems
                    if !device.starts_with("/dev/") && mount_point != "/" {
                        continue;
                    }

                    let fs_type = Self::detect_fs_type(mount_point);

                    // For btrfs, skip if we've already seen this device
                    if fs_type == "btrfs" {
                        if seen_btrfs_devices.contains(device) {
                            continue;
                        }
                        seen_btrfs_devices.insert(device.to_string());
                    }

                    let total_bytes: u64 = parts[1].parse().unwrap_or(0);
                    let used_bytes: u64 = parts[2].parse().unwrap_or(0);
                    let percent_str = parts[4].trim_end_matches('%');
                    let percent = percent_str.parse().unwrap_or(0);

                    let used_str = Self::format_bytes(used_bytes);
                    let total_str = Self::format_bytes(total_bytes);

                    let info = format!("{used_str} / {total_str} - {fs_type}");
                    disks.push((mount_point.to_string(), info, percent));
                }
            }
        }

        // Sort by mount point, with "/" first
        disks.sort_by(|a, b| {
            if a.0 == "/" {
                std::cmp::Ordering::Less
            } else if b.0 == "/" {
                std::cmp::Ordering::Greater
            } else {
                a.0.cmp(&b.0)
            }
        });

        if disks.is_empty() {
            disks.push(("/".to_string(), "Unknown".to_string(), 0));
        }

        disks
    }

    fn format_bytes(bytes: u64) -> String {
        const KIB: f64 = 1024.0;
        const MIB: f64 = KIB * 1024.0;
        const GIB: f64 = MIB * 1024.0;
        const TIB: f64 = GIB * 1024.0;

        let bytes_f = bytes as f64;

        if bytes_f >= TIB {
            format!("{:.2} TiB", bytes_f / TIB)
        } else if bytes_f >= GIB {
            format!("{:.2} GiB", bytes_f / GIB)
        } else if bytes_f >= MIB {
            format!("{:.2} MiB", bytes_f / MIB)
        } else if bytes_f >= KIB {
            format!("{:.2} KiB", bytes_f / KIB)
        } else {
            format!("{bytes} B")
        }
    }

    fn detect_fs_type(mount_point: &str) -> String {
        if let Ok(content) = fs::read_to_string("/proc/mounts") {
            for line in content.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 && parts[1] == mount_point {
                    return parts[2].to_string();
                }
            }
        }
        "unknown".to_string()
    }

    fn detect_local_ip() -> String {
        if let Ok(output) = Command::new("ip").args(["addr", "show"]).output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let re = Regex::new(r"inet\s+(\d+\.\d+\.\d+\.\d+/\d+)").unwrap();

            let mut current_interface = String::new();
            for line in output_str.lines() {
                if let Some(colon_pos) = line.find(':') {
                    if colon_pos > 0 && colon_pos < 10 {
                        let parts: Vec<&str> = line.split(':').collect();
                        if parts.len() >= 2 {
                            current_interface =
                                parts[1].trim().split('@').next().unwrap_or("").to_string();
                        }
                    }
                }

                if let Some(captures) = re.captures(line) {
                    let ip = &captures[1];
                    if !ip.starts_with("127.") && !current_interface.is_empty() {
                        return format!("{ip} ({current_interface})");
                    }
                }
            }
        }
        "Unknown".to_string()
    }

    #[allow(clippy::uninlined_format_args)]
    fn display(&self, logo: &Logo) {
        let label_color = logo
            .colors
            .get("$2")
            .or_else(|| logo.colors.get("$1"))
            .map(|color| self.color_to_ansi_for_label(color))
            .unwrap_or_else(|| "\x1b[96m".to_string()); // Default to cyan if no color

        let mut info_lines: Vec<(String, bool, u32)> = vec![
            (
                format!(
                    "{label_color}\x1b[1m{}\x1b[0m{label_color}\x1b[1m@\x1b[0m{label_color}\x1b[1m{}\x1b[0m",
                    self.username, self.hostname
                ),
                false,
                0,
            ),
            (
                format!(
                    "{label_color}{}\x1b[0m",
                    "-".repeat(self.username.len() + self.hostname.len() + 1)
                ),
                false,
                0,
            ),
            (
                format!("{label_color}\x1b[1mOS\x1b[0m: {}", self.os),
                false,
                0,
            ),
            (
                format!("{label_color}\x1b[1mHost\x1b[0m: {}", self.host),
                false,
                0,
            ),
            (
                format!("{label_color}\x1b[1mKernel\x1b[0m: {}", self.kernel),
                false,
                0,
            ),
            (
                format!("{label_color}\x1b[1mUptime\x1b[0m: {}", self.uptime),
                false,
                0,
            ),
            (
                format!("{label_color}\x1b[1mPackages\x1b[0m: {}", self.packages),
                false,
                0,
            ),
            (
                format!("{label_color}\x1b[1mShell\x1b[0m: {}", self.shell),
                false,
                0,
            ),
            (
                format!("{label_color}\x1b[1mDisplay\x1b[0m: {}", self.display),
                false,
                0,
            ),
            (
                format!("{label_color}\x1b[1mTerminal\x1b[0m: {}", self.terminal),
                false,
                0,
            ),
            (
                format!("{label_color}\x1b[1mCPU\x1b[0m: {}", self.cpu),
                false,
                0,
            ),
            (
                format!("{label_color}\x1b[1mGPU\x1b[0m: {}", self.gpu),
                false,
                0,
            ),
            (
                format!(
                    "{label_color}\x1b[1mMemory\x1b[0m: {} ({}%)",
                    self.memory, self.memory_percent
                ),
                true,
                self.memory_percent,
            ),
            (
                format!("{label_color}\x1b[1mSwap\x1b[0m: {}", self.swap),
                false,
                0,
            ),
        ];

        for (mount_point, info, percent) in &self.disks {
            let label = if mount_point == "/" {
                "Disk (/)".to_string()
            } else {
                format!("Disk ({mount_point})")
            };

            let fs_type = if let Some(pos) = info.rfind(" - ") {
                &info[pos + 3..]
            } else {
                "unknown"
            };

            let size_info = if let Some(pos) = info.rfind(" - ") {
                &info[..pos]
            } else {
                info
            };

            info_lines.push((
                format!(
                    "{}\x1b[1m{label}\x1b[0m: {size_info} ({percent}%) - {fs_type}",
                    label_color
                ),
                true,
                *percent,
            ));
        }

        info_lines.push((
            {
                let parts: Vec<&str> = self.local_ip.split(" (").collect();
                if parts.len() == 2 {
                    let ip = parts[0];
                    let interface = parts[1].trim_end_matches(')');
                    format!("{label_color}\x1b[1mLocal IP ({interface})\x1b[0m: {ip}")
                } else {
                    format!("{label_color}\x1b[1mLocal IP\x1b[0m: {}", self.local_ip)
                }
            },
            false,
            0,
        ));
        info_lines.push((
            format!("{label_color}\x1b[1mLocale\x1b[0m: {}", self.locale),
            false,
            0,
        ));

        let max_logo_width = logo
            .lines
            .iter()
            .map(|line| Logo::calculate_display_width(line))
            .max()
            .unwrap_or(0);

        let mut current_color = logo.colors.get("$1").copied();

        for (i, (info_line, has_percent, percent)) in info_lines.iter().enumerate() {
            if info_line.contains(": unknown")
                || info_line.contains(": Unknown")
                || info_line.contains(": None")
            {
                continue;
            }

            if i < logo.lines.len() {
                let rendered_line = logo.render_line(&logo.lines[i], &mut current_color);
                print!("\x1b[1m{rendered_line}\x1b[0m");

                let line_width = Logo::calculate_display_width(&logo.lines[i]);

                let padding = max_logo_width - line_width + 2;
                print!("{}", " ".repeat(padding));

                if *has_percent {
                    if let Some(pos) = info_line.rfind(" (") {
                        let (before, after) = info_line.split_at(pos);
                        print!("{before}");

                        if let Some(close_pos) = after.find(')') {
                            let color = if *percent < 33 {
                                "\x1b[92m" // Bright green
                            } else if *percent < 66 {
                                "\x1b[93m" // Bright yellow
                            } else {
                                "\x1b[91m" // Bright red
                            };

                            print!(" ({}{}%\x1b[0m){}", color, percent, &after[close_pos + 1..]);
                        } else {
                            print!("{after}");
                        }
                    } else {
                        print!("{info_line}");
                    }
                } else {
                    print!("{info_line}");
                }

                println!("\x1b[0m");
            } else {
                println!("{}{}", " ".repeat(max_logo_width + 2), info_line);
            }
        }

        for i in info_lines.len()..logo.lines.len() {
            println!(
                "\x1b[1m{}\x1b[0m",
                logo.render_line(&logo.lines[i], &mut current_color)
            );
        }

        println!();

        let padding = " ".repeat(max_logo_width + 2);

        // Normal colors
        print!("{padding}");
        print!("\x1b[40m   "); // Black
        print!("\x1b[41m   "); // Red
        print!("\x1b[42m   "); // Green
        print!("\x1b[43m   "); // Yellow
        print!("\x1b[44m   "); // Blue
        print!("\x1b[45m   "); // Magenta
        print!("\x1b[46m   "); // Cyan
        print!("\x1b[47m   "); // White
        println!("\x1b[0m");

        // Bright colors
        print!("{padding}");
        print!("\x1b[100m   "); // Bright black
        print!("\x1b[101m   "); // Bright red
        print!("\x1b[102m   "); // Bright green
        print!("\x1b[103m   "); // Bright yellow
        print!("\x1b[104m   "); // Bright blue
        print!("\x1b[105m   "); // Bright magenta
        print!("\x1b[106m   "); // Bright cyan
        print!("\x1b[107m   "); // Bright white
        println!("\x1b[0m");
    }
}

fn main() {
    let args = Args::parse();

    if args.list_logos {
        println!("Available logos:");
        for logo in Logo::list_available() {
            println!("  - {logo}");
        }
        return;
    }

    let system_info = SystemInfo::new();

    let color_overrides = args.get_color_overrides();

    let logo_result = if let Some(logo_path) = args.logo_file {
        Logo::from_file(&logo_path, color_overrides)
    } else {
        let logo_name = args
            .logo
            .as_ref()
            .map(|s| s.to_lowercase())
            .unwrap_or_else(SystemInfo::detect_os_id);

        Logo::load(&logo_name, color_overrides)
    };

    match logo_result {
        Ok(logo) => system_info.display(&logo),
        Err(e) => {
            eprintln!("Warning: {e}");
            eprintln!("Displaying info without logo");

            let empty_logo = Logo {
                lines: vec![],
                colors: Logo::get_default_colors(),
            };
            system_info.display(&empty_logo);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color_names() {
        assert_eq!(parse_color("red"), Some(Color::Red));
        assert_eq!(parse_color("RED"), Some(Color::Red));
        assert_eq!(parse_color("darkred"), Some(Color::DarkRed));
        assert_eq!(parse_color("brightred"), Some(Color::Red));
        assert_eq!(parse_color("yellow"), Some(Color::Yellow));
        assert_eq!(parse_color("darkyellow"), Some(Color::DarkYellow));
        assert_eq!(parse_color("cyan"), Some(Color::Cyan));
        assert_eq!(parse_color("orange"), Some(Color::AnsiValue(208)));
        assert_eq!(parse_color("pink"), Some(Color::AnsiValue(213)));
    }

    #[test]
    fn test_parse_hex_colors() {
        assert_eq!(
            parse_color("#FF0000"),
            Some(Color::Rgb { r: 255, g: 0, b: 0 })
        );
        assert_eq!(
            parse_color("#00FF00"),
            Some(Color::Rgb { r: 0, g: 255, b: 0 })
        );
        assert_eq!(
            parse_color("#0000FF"),
            Some(Color::Rgb { r: 0, g: 0, b: 255 })
        );
        assert_eq!(
            parse_color("#123456"),
            Some(Color::Rgb {
                r: 18,
                g: 52,
                b: 86
            })
        );
    }

    #[test]
    fn test_parse_ansi_colors() {
        assert_eq!(parse_color("0"), Some(Color::AnsiValue(0)));
        assert_eq!(parse_color("128"), Some(Color::AnsiValue(128)));
        assert_eq!(parse_color("255"), Some(Color::AnsiValue(255)));
    }

    #[test]
    fn test_parse_invalid_colors() {
        assert_eq!(parse_color("invalid"), None);
        assert_eq!(parse_color("#GGG"), None);
        assert_eq!(parse_color("#12345"), None);
        assert_eq!(parse_color("256"), None);
    }
}
