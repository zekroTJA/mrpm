use std::fmt;

static mut VERBOSE_LOGGING_ENABLED: bool = false;

pub fn set_enable_verbose_logging(enable: bool) {
    unsafe { VERBOSE_LOGGING_ENABLED = enable }
}

pub fn get_verbose_logging_enabled() -> bool {
    unsafe { VERBOSE_LOGGING_ENABLED }
}

#[macro_export]
macro_rules! log_verbose {
    ( $($arg:tt)* ) => {
        if crate::logger::get_verbose_logging_enabled() {
            use yansi::Paint;
            println!("{}", format_args!("-- {}", format_args!($($arg)*)).dim());
        }
    };
}

pub struct DisplayList<'a, I>(pub &'a I);

impl<'a, I> fmt::Display for DisplayList<'a, I>
where
    &'a I: IntoIterator,
    <&'a I as IntoIterator>::Item: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut iter = (&self.0).into_iter();
        if let Some(first) = iter.next() {
            write!(f, "{first}")?;
        }
        for v in iter {
            write!(f, ", {v}")?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

pub struct Optional<T>(pub Option<T>);

impl<T> fmt::Display for Optional<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(v) => write!(f, "Some({v})"),
            None => write!(f, "None"),
        }
    }
}
