use impl_variadics::variadic;
use std::fmt::{Display, Formatter, Result};

struct TupleDisplay<T>(T);

variadic! {
    ..4 "T*" => {
        impl<#(#T0),*> Display for TupleDisplay<(#(#T0,)*)>
        where
            #(#T0: Display,)*
        {
            fn fmt(&self, _f: &mut Formatter) -> Result {
                #(self.0.#index.fmt(_f)?;)*
                Ok(())
            }
        }
    };
}

fn main() {
    println!("{}", TupleDisplay((1, 2, "aa")));
}
