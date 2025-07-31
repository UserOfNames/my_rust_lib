#[macro_export]
/// Evaluate a fallible expression and `continue` if it fails.
///
/// Accepts an expression that evaluates to a `Result`.
/// If the `Result` is `Ok(t)`, yields `t`.
/// If the `Result` is `Err(e)`, prints `e` to `stderr`, optionally prepending a given message, and
/// `continue`s.
///
/// # Patterns
/// `($result:expr)`: On `Err`, print the error and `continue`.
/// `($result:expr, $emsg:expr)`: On `Err`, print "$emsg: {error}" and `continue`.
macro_rules! continue_on_err {
    ($result:expr) => {
        match $result {
            Ok(x) => x,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        }
    };

    ($result:expr, $emsg:expr) => {
        match $result {
            Ok(x) => x,
            Err(e) => {
                eprintln!("{}: {}", $emsg, e);
                continue;
            }
        }
    };
}

#[macro_export]
/// Evaluate a fallible expression and `break` if it fails.
///
/// Accepts an expression that evaluates to a `Result`.
/// If the `Result` is `Ok(t)`, yields `t`.
/// If the `Result` is `Err(e)`, prints `e` to `stderr`, optionally prepending a given message,
/// and `break`s.
///
/// # Patterns
/// `($result:expr)`: On `Err`, print the error and `break`.
/// `($result:expr, $emsg:expr)`: On `Err`, print "$emsg: {error}" and `break`.
macro_rules! break_on_err {
    ($result:expr) => {
        match $result {
            Ok(x) => x,
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
        }
    };

    ($result:expr, $emsg:expr) => {
        match $result {
            Ok(x) => x,
            Err(e) => {
                eprintln!("{}: {}", $emsg, e);
                break;
            }
        }
    };
}

#[macro_export]
/// Evaluate an optional expression and `continue` if it's `None`.
///
/// Accepts an expression that evaluates to an `Option`.
/// If the `Option` is `Some(t)`, yields `t`.
/// If the `Option` is `None`, optionally prints a message and `continue`s.
///
/// # Patterns
/// `($result:expr)`: On `None`, `continue`.
/// `($result:expr, $msg:expr)`: On `None`, print "$msg" and `continue`.
macro_rules! continue_on_none {
    ($result:expr) => {
        match $result {
            Some(x) => x,
            None => {
                continue;
            }
        }
    };

    ($result:expr, $msg:expr) => {
        match $result {
            Some(x) => x,
            None => {
                println!("{}", $msg);
                continue;
            }
        }
    };
}

#[macro_export]
/// Evaluate an optional expression and `break` if it's `None`.
///
/// Accepts an expression that evaluates to an `Option`.
/// If the `Option` is `Some(t)`, yields `t`.
/// If the `Option` is `None`, optionally prints a message and `break`s.
///
/// # Patterns
/// `($result:expr)`: On `None`, `break`.
/// `($result:expr, $msg:expr)`: On `None`, print "$msg" and `break`.
macro_rules! break_on_none {
    ($result:expr) => {
        match $result {
            Some(x) => x,
            None => {
                break;
            }
        }
    };

    ($result:expr, $msg:expr) => {
        match $result {
            Some(x) => x,
            None => {
                println!("{}", $msg);
                break;
            }
        }
    };
}
