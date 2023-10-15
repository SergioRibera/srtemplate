#![allow(unused)]

use paste::paste;
use std::ops::{Add, Div, Mul, Sub};

use crate::function::FuncResult;
use crate::prelude::validations;

macro_rules! gen_math_fn {
    ($name: ident, $( $t: ty ),* ) => {
        $(
            paste! {
                /// Perform arithmetic operations on a list of values and return the result as a string.
                ///
                /// This function takes a slice of strings, attempts to parse them as values of type `$t`,
                /// and then applies the `$name` operation to those values.
                ///
                /// # Arguments
                ///
                /// * `args`: A slice of strings representing values to perform the operation on.
                ///
                /// # Returns
                ///
                /// * A [`FuncResult`] containing the result of the operation as a string.
                ///
                /// # Errors
                ///
                /// This function can return an error of [`crate::function::FunctionError`] variant:
                /// - `crate::function::FunctionError::InvalidType` if any argument cannot be parsed as a value of type `$t`.
                /// - `crate::function::FunctionError::ConvertArgsFailed` if parsing or conversion from arguments failed.
                #[cfg_attr(docsrs, doc(cfg(feature = "math")))]
                #[cfg(feature = "math")]
                pub fn [<$name _ $t>](args: &[String]) -> FuncResult {
                    for arg in args {
                        validations::arg_type::<$t>(arg.clone())?;
                    }
                    Ok(args.iter()
                        .map(|a| a.parse::<$t>().unwrap_or_default())
                        .fold($t::default(), |acc, x| acc.$name(x))
                        .to_string())
                }
            }
        )*
    };
}

gen_math_fn!(add, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
gen_math_fn!(sub, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
gen_math_fn!(mul, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
gen_math_fn!(div, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

#[doc(hidden)]
#[macro_export]
macro_rules! gen_math_use {
    ($tmp:ident) => {
        gen_math_use!($tmp, add, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
        gen_math_use!($tmp, sub, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
        gen_math_use!($tmp, mul, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
        gen_math_use!($tmp, div, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
    };

    ($tmp: ident, $name: ident, $( $t: ty ),* ) => {
        $(
            paste! {
                $tmp.add_function(stringify!([<$name _ $t>]), builtin::math::[<$name _ $t>]);
            }
        )*
    };
}
