use impl_variadics::impl_variadics;

impl_variadics! {
    1..6 "GrowMode*" "T*" => {
        pub type #GrowMode0<#(#T0,)*> = (#(#T0,)*);
    };

    1..6 "RepeatMode*" => {
        pub type #RepeatMode0<T> = (#(T,)*);
    }
}

fn main() {}
