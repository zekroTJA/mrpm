#[macro_export]
macro_rules! print_install_error {
    ($package:expr, $($arg:tt)*) => {
        println!("{}{}{} {}\n    {} {}", '['.dim(), '!'.red().bold(), ']'.dim(),
            $package, "error:".red().bold(), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! print_install_success {
    ($package:expr) => {
        println!(
            "{}{}{} {}",
            '['.dim(),
            '+'.green().bold(),
            ']'.dim(),
            $package
        )
    };
}

#[macro_export]
macro_rules! print_install_skipped {
    ($package:expr) => {
        println!(
            "{}{}{} {}",
            '['.dim(),
            '>'.dim().bold(),
            ']'.dim(),
            $package
        )
    };
}
