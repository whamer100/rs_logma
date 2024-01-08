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

// example use
/*
        use logma::*;

        logma!(info, "this is a test");
        logma!(warn, "this is a test");
        logma!(fatal, "this is a test");
        logma!(debug, "this is a test");
        logma!(custom, "[LMAO]".truecolor(255, 0, 255), "this is a test".truecolor(255, 0, 255));

        logma!(info, "{} hi :)", "this is a test");
        logma!(warn, "{} hi :)", "this is a test");
        logma!(fatal, "{} hi :)", "this is a test");
        logma!(debug, "{} hi :)", "this is a test");
        // not working yet
        // logma!(custom, "[LMAO]".truecolor(255, 0, 255), "{} hi :)", "this is a test".truecolor(255, 0, 255));
*/

// its testing time //
#[cfg(test)]
mod tests {
    use colored::Colorize;
    use crate::*;

    #[test]
    fn test_normal() {
        logma!(info, "this is a test");
        logma!(warn, "this is a test");
        logma!(fatal, "this is a test");
        logma!(debug, "this is a test");
        logma!(custom, "[CUSTOM]".truecolor(255, 0, 255), "this is a test".truecolor(255, 0, 255));
    }

    #[test]
    fn test_error_text() { // writes to stderr
        elogma!(info, "this is a test");
        elogma!(warn, "this is a test");
        elogma!(fatal, "this is a test");
        elogma!(debug, "this is a test");
        elogma!(custom, "[CUSTOM]".truecolor(255, 0, 255), "this is a test".truecolor(255, 0, 255));
    }

    #[test]
    fn test_format() {
        logma!(info, "{} is a test", "this");
    }
}