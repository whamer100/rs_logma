pub mod logma {
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
            println!("{} {}",
                colored::Colorize::yellow("[WARN]"),
                colored::Colorize::yellow($fmt)
            )
        };

        ($fmt:expr, $($args:tt)*) => {
            println!("{} {}",
                colored::Colorize::yellow("[WARN]"),
                colored::Colorize::yellow(format_args!($fmt, $($args)*).to_string().as_str()),
            )
        };
    }

    #[macro_export]
    macro_rules! fatal {
        ($fmt:expr) => {
            println!("{} {}",
                colored::Colorize::red("[FATAL]"),
                colored::Colorize::red($fmt)
            )
        };

        ($fmt:expr, $($args:tt)*) => {
            println!("{} {}",
                colored::Colorize::red("[FATAL]"),
                colored::Colorize::red(format_args!($fmt, $($args)*).to_string().as_str()),
            )
        };
    }

    #[macro_export]
    macro_rules! debug {
        ($fmt:expr) => {
            if cfg!(debug_assertions) {
                println!("{} {}",
                    colored::Colorize::bright_black("[DEBUG]"),
                    colored::Colorize::bright_black($fmt)
                )
            }
        };

        ($fmt:expr, $($args:tt)*) => {
            if cfg!(debug_assertions) {
                println!("{} {}",
                    colored::Colorize::bright_black("[DEBUG]"),
                    colored::Colorize::bright_black(format_args!($fmt, $($args)*).to_string().as_str())
                )
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
        warn!("{} is a test", "this");
        fatal!("{} is a test", "this");
        debug!("{} is a test", "this");
    }
}