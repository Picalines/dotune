use core::fmt;
use core::fmt::Display;
use std::path::{MAIN_SEPARATOR, Path};

use super::system_info::{CURRENT_DIR, HOME_DIR};

pub trait RelativeDisplay {
    fn relative_display(&self) -> impl fmt::Display;
}

impl RelativeDisplay for Path {
    fn relative_display(&self) -> impl fmt::Display {
        let path = dunce::simplified(self);
        let cwd = dunce::simplified(CURRENT_DIR.as_ref());
        let home = dunce::simplified(HOME_DIR.as_ref());

        fmt::from_fn(move |formatter| {
            if let Ok(relative) = path.strip_prefix(cwd) {
                return relative.display().fmt(formatter);
            }

            if let Ok(relative) = path.strip_prefix(home) {
                formatter.write_str("~")?;
                if relative.is_empty() {
                    return Ok(());
                }
                return write!(formatter, "{}{}", MAIN_SEPARATOR, relative.display());
            }

            path.display().fmt(formatter)
        })
    }
}
