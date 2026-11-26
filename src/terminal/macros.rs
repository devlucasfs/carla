/// get_flag!(args, "output")                 -> Option<&str>
/// get_flag!(args, "output", || "a.out")     -> &str (default)
/// get_flag!(args, "jobs", usize)            -> Option<usize>
/// get_flag!(args, "jobs", usize, 4)         -> usize (default)
#[macro_export]
macro_rules! get_flag {
    ($args:expr, $name:expr) => {
        $args
            .iter()
            .find(|(k, _)| k.as_str() == $name)
            .map(|(_, v)| v.as_str())
    };
    // string com default (esta regra vem antes das que usam $ty)
    ($args:expr, $name:expr, || $default:expr) => {
        $crate::get_flag!($args, $name).unwrap_or($default)
    };
    ($args:expr, $name:expr, $ty:ty) => {
        $crate::get_flag!($args, $name).and_then(|v| v.parse::<$ty>().ok())
    };
    ($args:expr, $name:expr, $ty:ty, $default:expr) => {
        $crate::get_flag!($args, $name, $ty).unwrap_or($default)
    };
}

#[macro_export]
macro_rules! has_flag {
    ($args:expr, $name:expr) => {
        $args.iter().any(|(k, _)| k.as_str() == $name)
    };
}

/// bind_args!(args => {
///     output: &str = "output" || "a.out",          // &str (default)
///     input: &str = "input",                       // Option<&str>
///     name: String = "name" || String::new(),      // String (default)
///     jobs: usize = "jobs" || 4,                   // usize (default)
///     @bool verbose = "verbose",                   // bool
/// });
#[macro_export]
macro_rules! bind_args {
    ($args:ident => { }) => {};

    // boolean
    ($args:ident => { @bool $var:ident = $flag:literal $(, $($rest:tt)*)? }) => {
        let $var: bool = $crate::has_flag!($args, $flag);
        $crate::bind_args!($args => { $($($rest)*)? });
    };

    // &str com default
    ($args:ident => { $var:ident : & str = $flag:literal || $def:expr $(, $($rest:tt)*)? }) => {
        let $var: &str = $crate::get_flag!($args, $flag, || $def);
        $crate::bind_args!($args => { $($($rest)*)? });
    };

    // &str opcional
    ($args:ident => { $var:ident : & str = $flag:literal $(, $($rest:tt)*)? }) => {
        let $var: Option<&str> = $crate::get_flag!($args, $flag);
        $crate::bind_args!($args => { $($($rest)*)? });
    };

    // default
    ($args:ident => { $var:ident : $ty:ty = $flag:literal || $def:expr $(, $($rest:tt)*)? }) => {
        let $var: $ty = $crate::get_flag!($args, $flag, $ty, $def);
        $crate::bind_args!($args => { $($($rest)*)? });
    };

    // optional
    ($args:ident => { $var:ident : $ty:ty = $flag:literal $(, $($rest:tt)*)? }) => {
        let $var: Option<$ty> = $crate::get_flag!($args, $flag, $ty);
        $crate::bind_args!($args => { $($($rest)*)? });
    };
}

#[macro_export]
macro_rules! command_fn {
    ($name:ident($args:ident) { $($binds:tt)* } => $body:block) => {
        #[allow(unused_variables)]
        pub fn $name($args: $crate::terminal::Args) {
            $crate::bind_args!($args => { $($binds)* });
            $body
        }
    };
}

#[macro_export]
macro_rules! flags {
    ($( $kind:ident($long:literal, $short:literal) => $desc:literal ),* $(,)?) => {
        &[ $( ($crate::terminal::Flag::$kind($long, $short), $desc) ),* ]
    };
}

#[macro_export]
macro_rules! command {
    (
        name: $name:literal,
        description: $desc:literal,
        since: $since:expr,
        example: $example:literal,
        flags: { $($flags:tt)* },
        function: $f:path $(,)?
    ) => {
        $crate::terminal::Command {
            function: $f,
            name: $name,
            flags: $crate::flags! { $($flags)* },
            description: $desc,
            since: $since,
            example: $example,
        }
    };
}
