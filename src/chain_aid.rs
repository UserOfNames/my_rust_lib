#[macro_export]
/// Declare a set of immutable variables as `type`.
///
/// This is equivalent to `let var: type;`. As such, it will not initialize any of the given
/// identifiers, and it can shadow existing variables.
///
/// # Examples
/// ```
/// declare!(String, s1, s2, s3);
/// // Equivalent to:
/// let s1: String;
/// let s2: String;
/// let s3: String;
/// ```
macro_rules! declare {
    ($type:ty, $($var:ident),+ $(,)?) => {
        $(let $var: $type;)+
    };
}

#[macro_export]
/// Declare a set of mutable variables as `type`.
///
/// This is equivalent to `let mut var: type;`. As such, it will not initialize any of the given
/// identifiers, and it can shadow existing variables.
///
/// # Examples
/// ```
/// declare_mut!(String, s1, s2, s3);
/// // Equivalent to:
/// let mut s1: String;
/// let mut s2: String;
/// let mut s3: String;
/// ```
macro_rules! declare_mut {
    ($type:ty, $($var:ident),+ $(,)?) => {
        $(let mut $var: $type;)+
    };
}

#[macro_export]
/// Initialize a set of immutable variables with `value`.
///
/// This is equivalent to `let var = value;`. As such, it can shadow existing variables.
/// Non-`Copy` `value`s will be cloned for each variable.
///
/// # Examples
/// ```
/// initialize!(String::new(), s1, s2, s3);
/// // Equivalent to:
/// let s1 = String::new();
/// let s2 = String::new();
/// let s3 = String::new();
/// ```
macro_rules! initialize {
    ($value:expr, $($var:ident),+ $(,)?) => {
        let x = $value;
        $(let $var = x.clone();)+
    }
}

#[macro_export]
/// Initialize a set of mutable variables with `value`.
///
/// This is equivalent to `let mut var = value;`. As such, it can shadow existing variables.
/// Non-`Copy` `value`s will be cloned for each variable.
///
/// # Examples
/// ```
/// initialize_mut!(String::new(), s1, s2, s3);
/// // Equivalent to:
/// let mut s1 = String::new();
/// let mut s2 = String::new();
/// let mut s3 = String::new();
/// ```
macro_rules! initialize_mut {
    ($value:expr, $($var:ident),+ $(,)?) => {
        let x = $value;
        $(let mut $var = x.clone();)+
    }
}

#[macro_export]
/// Assign `value` to a set of variables.
///
/// This is equivalent to `var = value;`. As such, `var` must already exist and either be
/// uninitialized or declared as mutable.
/// Non-`Copy` `value`s will be cloned for each variable.
///
/// # Examples
/// ```
/// assign!(String::new(), s1, s2, s3);
/// // Equivalent to:
/// s1 = String::new();
/// s2 = String::new();
/// s3 = String::new();
/// ```
macro_rules! assign {
    ($value:expr, $($var:ident),+ $(,)?) => {
        let x = $value;
        $($var = x.clone();)+
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
