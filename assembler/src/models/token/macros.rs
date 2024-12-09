macro_rules! token {
    ($name: ident) => {
        pub struct $name;

        impl FromToken for $name {
            fn from_token(token: Token) -> Option<$name> {
                if let Token::$name = token {
                    Some($name)
                } else {
                    None
                }
            }
        }
    };

    ($name: ident ($($elem:ident : $elem_ty: ty),+)) => {
        pub struct $name ($( pub $elem_ty ),+);

        impl FromToken for $name {
            fn from_token(token: Token) -> Option<$name> {
                if let Token::$name ($( $elem ),+) = token {
                    Some($name ($( $elem ),+))
                } else {
                    None
                }
            }
        }
    };
}

macro_rules! tokens {
    ($( $name: ident $(($elem_name: ident : $elem_ty: ty))? ),+) => {
        $(
            token!($name $(( $elem_name : $elem_ty ))?);
        )+
    };
}


pub(crate) use {token, tokens};
