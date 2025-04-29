#[macro_export]
macro_rules! print_install_error {
    ($package:expr, $($arg:tt)*) => {
        println!("{}{}{} {}\n    {} {}", '['.dim(), '!'.red().bright().bold(), ']'.dim(),
            $package, "error:".red().bold(), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! print_install_new {
    ($package:expr, $version:expr) => {
        println!(
            "{}{}{} {} ({})",
            '['.dim(),
            '+'.green().bright().bold(),
            ']'.dim(),
            $package,
            $version.green()
        )
    };
}

#[macro_export]
macro_rules! print_install_updated {
    ($package:expr, $previous_version:expr, $version:expr) => {
        println!(
            "{}{}{} {} ({} => {})",
            '['.dim(),
            '^'.cyan().bright().bold(),
            ']'.dim(),
            $package,
            $previous_version.dim(),
            $version.green()
        )
    };
}

#[macro_export]
macro_rules! print_install_skipped {
    ($package:expr,  $version:expr) => {
        println!(
            "{}{}{} {} ({})",
            '['.dim(),
            '>'.dim().bold(),
            ']'.dim(),
            $package,
            $version
        )
    };
}
