use syn::{
    parse::{Parse, ParseBuffer},
    Error, Ident, Result, Token, Type, Visibility,
};

pub(crate) struct MacroArgs {
    pub name: Ident,
    pub deps: Option<Type>,
    pub vis: Option<Visibility>,
}

impl Parse for MacroArgs {
    fn parse(buffer: &ParseBuffer<'_>) -> Result<Self> {
        let mut name = None;
        let mut deps = None;
        let mut vis = None;

        while buffer.peek(Ident) {
            let key: Ident = buffer.parse()?;
            let _: Token![=] = buffer.parse()?;

            match &*key.to_string() {
                "name" => {
                    if name.is_some() {
                        return Err(Error::new(
                            key.span(),
                            "`name` macro parameter specified multiple times",
                        ));
                    }

                    name = Some(buffer.parse()?);
                }
                "deps" => {
                    if deps.is_some() {
                        return Err(Error::new(
                            key.span(),
                            "`deps` macro parameter specified multiple times",
                        ));
                    }

                    deps = Some(buffer.parse()?);
                }
                "vis" => {
                    if vis.is_some() {
                        return Err(Error::new(
                            key.span(),
                            "`vis` macro parameter specified multiple times",
                        ));
                    }

                    vis = Some(buffer.parse()?);
                }
                x => {
                    return Err(Error::new(key.span(), format!("unknown argument `{}`", x)));
                }
            }

            if buffer.peek(Token![,]) {
                let _: Token![,] = buffer.parse()?;
            }
        }

        if name.is_none() {
            return Err(Error::new(
                buffer.cursor().span(),
                "A `name` macro parameter is required",
            ));
        }

        Ok(MacroArgs {
            name: name.unwrap(),
            deps,
            vis,
        })
    }
}
