//! automatically pad the string according to the rendered length  

/// # Examples
/// ```no_run
/// use youran::auto_pad_str::*;
/// fn main() {
///     let s1 = "hello 😊😊";
///     let s2 = "hello word";
///
///     println!("{:<1$}|", s1, s1.len());
///     println!("{:<1$}|", s2, s1.len());
///
///     println!("{:<1$}|", s1.as_auto_pad_str(), s1.as_auto_pad_str().rendered_len());
///     println!("{:<1$}|", s2.as_auto_pad_str(), s1.as_auto_pad_str().rendered_len());
/// }
/// ```

use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};


pub struct AutoPadStr<'a>(&'a str);

impl<'a> AutoPadStr<'a> {
    pub fn rendered_len(&self) -> usize {
        self.0.width()
    }
}


/// # Examples
/// ```no_run
/// use youran::auto_pad_str::*;
/// fn main() {
///     let s1 = "hello 😊😊";
///     let s2 = "hello word";
///
///     println!("{:<1$}|", s1, s1.len());
///     println!("{:<1$}|", s2, s1.len());
///
///     println!("{:<1$}|", s1.as_auto_pad_str(), s1.as_auto_pad_str().rendered_len());
///     println!("{:<1$}|", s2.as_auto_pad_str(), s1.as_auto_pad_str().rendered_len());
/// }
/// ```
pub trait AsAutoPadStr {
    fn as_auto_pad_str(&self) -> AutoPadStr;
}

impl<'a, T: AsRef<str>> AsAutoPadStr for T {
    fn as_auto_pad_str(&self) -> AutoPadStr {
        AutoPadStr(self.as_ref())
    }
}

impl<'a> std::fmt::Display for AutoPadStr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = 
            if let Some(precision) = f.precision() {
                let mut acc = 0_usize;
                let mut idx = self.0.len();
                for (i, w) in self.0.char_indices()
                    .map(|(i, c)|(i, c.width().unwrap_or(0)))
                {
                    acc += w;
                    if acc > precision {
                        idx = i;
                        break;
                    }
                }
                &self.0.get(..idx).unwrap()
            } else {
                self.0
            };

        let len = str.width();

        let c = f.fill();
        if let Some(width) = f.width() {
            if len >= width {
                write!(f, "{}", str)
            } else {
                if let Some(s) = f.align() {
                    let n = width - len;
                    match s {
                        std::fmt::Alignment::Left => {
                            write!(f, "{}", str)?;
                            write!(f, "{}", c.to_string().repeat(n))
                        },
                        std::fmt::Alignment::Right => {
                            write!(f, "{}", c.to_string().repeat(n))?;
                            write!(f, "{}", str)
                        },
                        std::fmt::Alignment::Center => {
                            write!(f, "{}", c.to_string().repeat(n/2))?;
                            write!(f, "{}", str)?;
                            write!(f, "{}", c.to_string().repeat(n - n/2))
                        },
                    }
                } else {
                    write!(f, "{}", str)
                }
            }
        } else {
            write!(f, "{}", str)
        }

    }
}
