#[allow(unused_imports)]  // even though this is used, my ide is upset for some reason
use colored::Colorize;

macro_rules! logma {
    (info, $fmt:expr) => {
        println!("{} {}", "[INFO]", $fmt)
    };
    (warn, $fmt:expr) => {
        println!("{} {}", "[WARN]".yellow(), $fmt.yellow())
    };
    (fatal, $fmt:expr) => {
        println!("{} {}", "[FATAL]".red(), $fmt.red())
    };
    (debug, $fmt:expr) => {
        if cfg!(debug_assertions) {
            println!("{} {}", "[DEBUG]".bright_black(), $fmt.bright_black())
        }
    };
    (custom, $text:expr, $fmt:expr) => {
        println!("{} {}", $text, $fmt)
    };

    (info, $fmt:expr, $($args:tt)*) => {
        println!("{} {}", "[INFO]", format_args!($fmt, $($args)*))
    };
    (warn, $fmt:expr, $($args:tt)*) => {
        println!("{} {}", "[WARN]".yellow(), format_args!($fmt, $($args)*).to_string().yellow())
    };
    (fatal, $fmt:expr, $($args:tt)*) => {
        println!("{} {}", "[FATAL]".red(), format_args!($fmt, $($args)*).to_string().red())
    };
    (debug, $fmt:expr, $($args:tt)*) => {
        if cfg!(debug_assertions) {
            println!("{} {}", "[DEBUG]".bright_black(), format_args!($fmt, $($args)*).to_string().bright_black())
        }
    };
    // not working yet
    // (custom, $text:expr, $fmt:expr, $($args:tt)*) => {
    //     println!("{} {}", $text, format_args!($fmt, $($args)*))
    // };
}
pub(crate) use logma;

macro_rules! elogma {
    (info, $fmt:expr) => {
        eprintln!("{} {}", "[INFO]", $fmt)
    };
    (warn, $fmt:expr) => {
        eprintln!("{} {}", "[WARN]".yellow(), $fmt.yellow())
    };
    (fatal, $fmt:expr) => {
        eprintln!("{} {}", "[FATAL]".red(), $fmt.red())
    };
    (debug, $fmt:expr) => {
        if cfg!(debug_assertions) {
            eprintln!("{} {}", "[DEBUG]".bright_black(), $fmt.bright_black())
        }
    };
    (custom, $text:expr, $fmt:expr) => {
        eprintln!("{} {}", $text, $fmt)
    };

    (info, $fmt:expr, $($args:tt)*) => {
        eprintln!("{} {}", "[INFO]", format_args!($fmt, $($args)*))
    };
    (warn, $fmt:expr, $($args:tt)*) => {
        eprintln!("{} {}", "[WARN]".yellow(), format_args!($fmt, $($args)*).to_string().yellow())
    };
    (fatal, $fmt:expr, $($args:tt)*) => {
        eprintln!("{} {}", "[FATAL]".red(), format_args!($fmt, $($args)*).to_string().red())
    };
    (debug, $fmt:expr, $($args:tt)*) => {
        if cfg!(debug_assertions) {
            eprintln!("{} {}", "[DEBUG]".bright_black(), format_args!($fmt, $($args)*).to_string().bright_black())
        }
    };
    // not working yet
    // (custom, $text:expr, $fmt:expr, $($args:tt)*) => {
    //     println!("{} {}", $text, format_args!($fmt, $($args)*))
    // };
}
pub(crate) use elogma;