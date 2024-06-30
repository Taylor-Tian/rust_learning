macro_rules! create_functions {
    ($($func_name:ident),*) => {
        $(
            pub fn $func_name() {
                println!("Function was called, {}!", stringify!($func_name));
            }
        )*
    };
}

create_functions!(alpha, beta, gamma);

fn main() {
    alpha();
    beta();
    gamma();

    println!("All functions called!");
}