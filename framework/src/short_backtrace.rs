use backtrace::{Backtrace, BacktraceFmt, BacktraceFrame, PrintFmt};
use std::fmt;
use std::fmt::Debug;

const FILTERS: [&str; 3] = [".cargo", "rustc", "framework/src/"];

pub struct ShortBacktrace(Backtrace);

impl ShortBacktrace {
    pub fn new() -> Self {
        Self(backtrace::Backtrace::new())
    }
}

impl Debug for ShortBacktrace {
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
