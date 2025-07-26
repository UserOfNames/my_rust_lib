#[macro_export]
macro_rules! declare {
    ($type:ty, $($var:ident),+ $(,)?) => {
        $(let $var: $type;)+
    };
}

#[macro_export]
macro_rules! declare_mut {
    ($type:ty, $($var:ident),+ $(,)?) => {
        $(let mut $var: $type;)+
    };
}

#[macro_export]
macro_rules! initialize {
    ($value:expr, $($var:ident),+ $(,)?) => {
        $(let $var = $value;)+
    }
}

#[macro_export]
macro_rules! initialize_mut {
    ($value:expr, $($var:ident),+ $(,)?) => {
        $(let mut $var = $value;)+
    }
}

#[macro_export]
macro_rules! assign {
    ($value:expr, $($var:ident),+ $(,)?) => {
        $($var = $value;)+
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
