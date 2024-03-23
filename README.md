# Impl Variadic

A macro for generate variadic generics.

The syntax is similar to `quote`.

## Example

```rust
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
    /*
    10..20 "Ty*pe" "my_index_*" "and_more_*" => {
        ...
    }
    */
}
```

it expands to

```rust
impl Display for TupleDisplay<()> {
    fn fmt(&self, _f: &mut Formatter) -> Result {
        Ok(())
    }
}
impl<T0> Display for TupleDisplay<(T0,)>
where
    T0: Display,
{
    fn fmt(&self, _f: &mut Formatter) -> Result {
        self.0 .0.fmt(_f)?;
        Ok(())
    }
}
impl<T0, T1> Display for TupleDisplay<(T0, T1)>
where
    T0: Display,
    T1: Display,
{
    fn fmt(&self, _f: &mut Formatter) -> Result {
        self.0 .0.fmt(_f)?;
        self.0 .1.fmt(_f)?;
        Ok(())
    }
}
impl<T0, T1, T2> Display for TupleDisplay<(T0, T1, T2)>
where
    T0: Display,
    T1: Display,
    T2: Display,
{
    fn fmt(&self, _f: &mut Formatter) -> Result {
        self.0 .0.fmt(_f)?;
        self.0 .1.fmt(_f)?;
        self.0 .2.fmt(_f)?;
        Ok(())
    }
}
```

- ..4: maximum iterator count is 4, from 0. you can add lower bound like `2..10`.
- `"T*"`: a custom identifier pattern. will replace all `*` with indexes.
  you can try other patterns like "Ty*pe*" or "index_*".
- `#index`: a builtin iterator gives 0 ~ max_index.
- `#length`: a builtin integer equals to iterator length.
- `#T0`: custom identifier. it gives `T0`, `T1`, `T2` ... `TN`, where N is the \
 upper bound of the range minus 2. it coresponding to pattern `T*`, replace all `*`
 with `0`.