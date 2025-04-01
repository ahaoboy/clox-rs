use std::ffi::CStr;
use std::os::raw::c_char;

pub trait IntoRust<T> {
    fn into_rust(&self) -> T;
}

impl IntoRust<String> for *const c_char {
    fn into_rust(&self) -> String {
        unsafe { CStr::from_ptr(*self).to_string_lossy().into_owned() }
    }
}

impl IntoRust<String> for *mut c_char {
    fn into_rust(&self) -> String {
        unsafe {
            CStr::from_ptr(*self as *const c_char)
                .to_string_lossy()
                .into_owned()
        }
    }
}

impl IntoRust<i32> for i32 {
    fn into_rust(&self) -> i32 {
        *self
    }
}

impl IntoRust<f64> for f64 {
    fn into_rust(&self) -> f64 {
        *self
    }
}

pub fn to_str(p: *const c_char) -> String {
    unsafe { CStr::from_ptr(p).to_string_lossy().into_owned() }
}

#[macro_export]
macro_rules! printf_stdout {
    (
        $fmt:expr, // Format string, which should implement FormatString.
        $($arg:expr),* // arguments
        $(,)? // optional trailing comma
    ) => {
        {
          use std::io::Write;
            let mut target = String::new();
            fish_printf::sprintf!(=> &mut target, clox_rs::to_str($fmt).as_str(), $(($arg.into_rust())),*);
            print!("{}", target);
            std::io::stdout().flush().unwrap();
        }
    };

    // Handling the case where there are no arguments
    (
        $fmt:expr // format string
        $(,)? // optional trailing comma
    ) => {
        {
          use std::io::Write;
            print!("{}", clox_rs::to_str($fmt));
            std::io::stdout().flush().unwrap();
        }
    };
}

#[macro_export]
macro_rules! printf_stderr {
    (
        $fmt:expr, // Format string, which should implement FormatString.
        $($arg:expr),* // arguments
        $(,)? // optional trailing comma
    ) => {
        {
          use std::io::Write;
            let mut target = String::new();
            fish_printf::sprintf!(=> &mut target, clox_rs::to_str($fmt).as_str(), $(($arg.into_rust())),*);
            eprint!("{}", target);
            std::io::stderr().flush().unwrap();
        }
    };

    // Handling the case where there are no arguments
    (
        $fmt:expr // format string
        $(,)? // optional trailing comma
    ) => {
        {
          use std::io::Write;
          eprint!("{}", clox_rs::to_str($fmt));
            std::io::stderr().flush().unwrap();
        }
    };
}
