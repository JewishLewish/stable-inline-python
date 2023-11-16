use lib;

///Experimenting...
///problem with macros is that it doesn't recognize newlines
#[macro_export]
macro_rules! python {
    ($($code:tt)*) => {
        {
            println!("{:?}",$($code)*);
            let code_str = stringify!($($code)*);
            code_str
        }
    };
}