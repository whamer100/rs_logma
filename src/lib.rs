#[allow(unused_imports)]  // even though this is used, my ide is upset for some reason


pub mod logma {
    use colored::Colorize;

    #[macro_export]
    macro_rules! info {
        ($fmt:expr) => {
            println!("{} {}", "[INFO]", $fmt)
        };

        ($fmt:expr, $($args:tt)*) => {
            println!("{} {}", "[INFO]", format_args!($fmt, $($args)*))
        };
    }

    #[macro_export]
    macro_rules! warn {
        ($fmt:expr) => {
            println!("{} {}", "[WARN]".yellow(), $fmt.yellow())
        };

        ($fmt:expr, $($args:tt)*) => {
            println!("{} {}", "[WARN]".yellow(), format_args!($fmt, $($args)*).to_string().yellow())
        };
    }

    #[macro_export]
    macro_rules! fatal {
        ($fmt:expr) => {
            println!("{} {}", "[FATAL]".red(), $fmt.red())
        };

        ($fmt:expr, $($args:tt)*) => {
            println!("{} {}", "[FATAL]".red(), format_args!($fmt, $($args)*).to_string().red())
        };
    }

    #[macro_export]
    macro_rules! debug {
        ($fmt:expr) => {
            if cfg!(debug_assertions) {
                println!("{} {}", "[DEBUG]".bright_black(), $fmt.bright_black())
            }
        };

        ($fmt:expr, $($args:tt)*) => {
            if cfg!(debug_assertions) {
                println!("{} {}", "[DEBUG]".bright_black(), format_args!($fmt, $($args)*).to_string().bright_black())
            }
        };
    }

    #[macro_export]
    macro_rules! custom {
        ($text:expr, $fmt:expr) => {
            println!("{} {}", $text, $fmt)
        };

        // not working yet
        // ($text:expr, $fmt:expr, $($args:tt)*) => {
        //     println!("{} {}", $text, format_args!($fmt, $($args)*))
        // };
    }
}

// todo: error variants

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
        info!("this is a test");
        warn!("this is a test");
        fatal!("this is a test");
        debug!("this is a test");
        custom!("[CUSTOM]".truecolor(255, 0, 255), "this is a test".truecolor(255, 0, 255));
    }

    #[test]
    fn test_format() {
        info!("{} is a test", "this");
    }
}