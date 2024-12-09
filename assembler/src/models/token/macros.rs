macro_rules! token {
    ($name: ident) => {
        pub struct $name;

        impl CastToToken<$name> for Token {
            fn cast(self) -> Option<$name> {
                if let Self::$name = self {
                    Some($name)
                } else {
                    None
                }
            }
        }
    };

    ($name: ident ($($elem:ident : $elem_ty: ty),+)) => {
        pub struct $name ($( pub $elem_ty ),+);

        impl CastToToken<$name> for Token {
            fn cast(self) -> Option<$name> {
                if let Token::$name ($( $elem ),+) = self {
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
