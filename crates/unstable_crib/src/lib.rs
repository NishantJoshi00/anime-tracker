#![feature(tuple_trait, unboxed_closures, fn_traits)]

use std::marker::Tuple;

pub fn arbitrary_function_wrapper<F, I, O>(func: F, input: I) -> O
where
    F: FnOnce<I, Output = O>,
    I: Tuple,
{
    func.call_once(input)
}

#[cfg(test)]
mod tests {
    fn concat(a: String, b: String) -> String {
        a + &b
    }

    #[test]
    fn concat_2_strings() {
        let output = super::arbitrary_function_wrapper(concat, ("hello".to_string(), "world".to_string()));
    }
}
