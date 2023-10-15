use std::str::FromStr;

use super::serialize::{FromArgs, FromArgsError, FromArgsResult};

macro_rules! tuple_impls {
    ( $( $name:ident )+ ) => {
        impl<$($name: FromStr),+> FromArgs for ($($name,)+)
        {
            fn from_args(args: &[String]) -> FromArgsResult<Self> {
                let mut index = 0;
                Ok(($(
                    #[allow(unused_assignments)]
                    {
                        let last = index;
                        index += 1;
                        args.get(last)
                            .ok_or(
                                FromArgsError::ArgumentNotExists(stringify!($name).to_string(), last)
                            )?
                            .parse::<$name>()
                            .map_err(|_| FromArgsError::ParseFailed(last))?
                    },
                )+))
            }
        }
    };
}

tuple_impls! { A }
tuple_impls! { A B }
tuple_impls! { A B C }
tuple_impls! { A B C D }
tuple_impls! { A B C D E }
tuple_impls! { A B C D E F }
tuple_impls! { A B C D E F G }
tuple_impls! { A B C D E F G H }
tuple_impls! { A B C D E F G H I }
tuple_impls! { A B C D E F G H I J }
tuple_impls! { A B C D E F G H I J K }
tuple_impls! { A B C D E F G H I J K L }
tuple_impls! { A B C D E F G H I J K L M }
tuple_impls! { A B C D E F G H I J K L M N }
tuple_impls! { A B C D E F G H I J K L M N O }
tuple_impls! { A B C D E F G H I J K L M N O P }
