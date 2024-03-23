use std::collections::{hash_map::Entry, HashMap};

use proc_macro2::Span;
use syn::{Error, Ident, LitStr, Result};

pub struct CustomIdMap {
    map: HashMap<Ident, String>,
}

impl CustomIdMap {
    pub fn new() -> Self {
        CustomIdMap {
            map: HashMap::new(),
        }
    }

    pub fn parse(&mut self, s: LitStr) -> Result<()> {
        let pattern = s.value();
        let key = fmt_idx(0, &pattern, s.span())?;

        if !pattern.contains("*") {
            return Err(Error::new_spanned(
                s,
                "custom identifier must contains at least one `*` token",
            ));
        }

        match self.map.entry(key) {
            Entry::Occupied(occ) => {
                return Err(Error::new_spanned(
                    pattern,
                    format!(
                        "custom identifier `{}` has already exists",
                        occ.key().to_string()
                    ),
                ))
            }
            Entry::Vacant(vac) => {
                vac.insert(pattern);
            }
        }
        Ok(())
    }

    pub fn get_nth_id(&self, key: &Ident, index: u32) -> Result<Ident> {
        let Some(pattern) = self.map.get(key) else {
            return Err(Error::new_spanned(
                key,
                format!("custom identifier `{}` is not defined", key.to_string()),
            ));
        };

        fmt_idx(index, pattern, key.span())
    }
}

fn fmt_idx(index: u32, s: &str, span: Span) -> Result<Ident> {
    let string = s.replace("*", &index.to_string());
    let mut id = syn::parse_str::<Ident>(&string)
        .map_err(|_| Error::new(span, format!("`{string}` is not a valid identifier")))?;
    id.set_span(span);
    Ok(id)
}
