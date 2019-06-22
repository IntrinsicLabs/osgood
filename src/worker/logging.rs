use super::*;

static VALID_COLORS: [u8; 75] = [
    20, 21, 26, 27, 33, 38, 39, 40, 41, 42, 43, 44, 45, 56, 57, 62, 63, 68, 69, 74, 75, 76, 77, 78,
    79, 80, 81, 92, 93, 98, 99, 112, 113, 128, 129, 134, 135, 148, 149, 160, 161, 162, 163, 164,
    165, 166, 167, 168, 169, 170, 171, 172, 173, 178, 179, 184, 185, 196, 197, 198, 199, 200, 201,
    202, 203, 204, 205, 206, 207, 208, 209, 214, 215, 220, 221,
];

#[macro_export]
macro_rules! log_osgood_message {
    ($($item:expr),+) => {
        println!("[{}] {}", ansi_term::Colour::Green.paint("OSGOOD"), format!($($item),+));
    }
}

#[macro_export]
macro_rules! log_osgood_error {
    ($($item:expr),+) => {
        eprintln!("[{}] {}: {}",
        ansi_term::Colour::Green.paint("OSGOOD"),
        ansi_term::Colour::Red.paint("ERROR"),
        format!($($item),+));
    }
}

#[macro_export]
macro_rules! log_worker_warning {
    ($($item:expr),+) => {
        eprintln!("[{}] [{}] {}: {}", $crate::worker::logging::color_name(),
        ansi_term::Colour::Green.paint("OSGOOD"),
        ansi_term::Colour::Yellow.paint("WARNING"),
        format!($($item),+));
    }
}

#[macro_export]
macro_rules! fmt_worker_error {
    ($($item:expr),+) => {
        format!("[{}] [{}] {}: {}", $crate::worker::logging::color_name(),
        ansi_term::Colour::Green.paint("OSGOOD"),
        ansi_term::Colour::Red.paint("ERROR"),
        format!($($item),+));
    }
}

#[macro_export]
macro_rules! log_worker_error {
    ($($item:expr),+) => {
        eprintln!("[{}] [{}] {}: {}", $crate::worker::logging::color_name(),
        ansi_term::Colour::Green.paint("OSGOOD"),
        ansi_term::Colour::Red.paint("ERROR"),
        format!($($item),+));
    }
}

#[macro_export]
macro_rules! log_info {
    ($($item:expr),+) => {
        info!("[{}] {}", $crate::worker::logging::color_name(), format!($($item),+));
    }
}

#[macro_export]
macro_rules! log_error {
    ($($item:expr),+) => {
        error!("[{}] {}", $crate::worker::logging::color_name(), format!($($item),+));
    }
}

#[macro_export]
macro_rules! log_debug {
    ($($item:expr),+) => {
        debug!("[{}] {}", $crate::worker::logging::color_name(), format!($($item),+));
    }
}

#[macro_export]
macro_rules! log_trace {
    ($($item:expr),+) => {
        trace!("[{}] {}", $crate::worker::logging::color_name(), format!($($item),+));
    }
}

#[macro_export]
macro_rules! log_warn {
    ($($item:expr),+) => {
        warn!("[{}] {}", $crate::worker::logging::color_name(), format!($($item),+));
    }
}

fn log_color(name: &str) -> u8 {
    let mut hash: usize = 0;
    for c in name.chars() {
        hash += c as usize;
    }
    // hash += std::process::id() as usize;
    VALID_COLORS[hash % VALID_COLORS.len()]
}

pub fn color_name() -> std::string::String {
    super::super::NAME.with(|name| {
        let name = name.borrow();
        let color = ansi_term::Colour::Fixed(log_color(&name));
        format!("{}", color.paint(name.as_str()))
    })
}
