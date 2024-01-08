pub mod logma;

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
    use crate::logma::*;

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