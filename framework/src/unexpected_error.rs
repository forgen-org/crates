use backtrace::{Backtrace, BacktraceFmt, BacktraceFrame, PrintFmt};
use std::fmt;
use std::fmt::Debug;
use thiserror::Error;
use tracing::error;

/// Generic Error
#[derive(Error, Debug)]
#[error("An unexpected error occurred: {message}")]
pub struct UnexpectedError {
    message: String,
}

impl UnexpectedError {
    pub fn from<T: std::fmt::Display>(err: T) -> Self {
        error!(
            error = %err,
            trace = ?Trace::new(),
            "Unexpected Error"
        );
        UnexpectedError {
            message: err.to_string(),
        }
    }
}

const FILTERS: [&str; 3] = [".cargo", "rustc", "framework/src/"];

struct Trace(Backtrace);

impl Trace {
    pub fn new() -> Self {
        Self(backtrace::Backtrace::new())
    }
}

impl Debug for Trace {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Filter out frames that are not relevant to the user
        let frames = self
            .0
            .frames()
            .iter()
            .filter(|frame| {
                let symbols = frame.symbols();
                for symbol in symbols {
                    if let Some(name) = symbol.filename() {
                        let filename = format!("{:?}", name);
                        if !FILTERS.iter().any(|filter| filename.contains(filter)) {
                            return true;
                        }
                    }
                }
                false
            })
            .collect::<Vec<&BacktraceFrame>>();

        // Print the backtrace (see capture.rs for the original implementation)
        let full = fmt.alternate();
        let style = PrintFmt::Short;
        let cwd = std::env::current_dir();
        let mut print_path =
            move |fmt: &mut fmt::Formatter<'_>, path: backtrace::BytesOrWideString<'_>| {
                let path = path.into_path_buf();
                if !full {
                    if let Ok(cwd) = &cwd {
                        if let Ok(suffix) = path.strip_prefix(cwd) {
                            return fmt::Display::fmt(&suffix.display(), fmt);
                        }
                    }
                }
                fmt::Display::fmt(&path.display(), fmt)
            };

        let mut f = BacktraceFmt::new(fmt, style, &mut print_path);
        f.add_context()?;
        for frame in frames {
            f.frame().backtrace_frame(frame)?;
        }
        f.finish()?;
        Ok(())
    }
}
