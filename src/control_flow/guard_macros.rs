#[macro_export]
macro_rules! continue_on_err {
    ($result:expr) => {
        match $result {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Error: {}", e);
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
macro_rules! break_on_err {
    ($result:expr) => {
        match $result {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Error: {}", e);
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
macro_rules! continue_on_none {
    ($result:expr) => {
        match $result {
            Some(x) => x,
            None => {
                continue;
            }
        }
    };

    ($result:expr, $emsg:expr) => {
        match $result {
            Some(x) => x,
            None => {
                eprintln!("{}", $emsg);
                continue;
            }
        }
    };
}

#[macro_export]
macro_rules! break_on_none {
    ($result:expr) => {
        match $result {
            Some(x) => x,
            None => {
                break;;
            }
        }
    };

    ($result:expr, $emsg:expr) => {
        match $result {
            Some(x) => x,
            None => {
                eprintln!("{}", $emsg);
                break;
            }
        }
    };
}
