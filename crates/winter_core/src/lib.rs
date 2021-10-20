#![cfg_attr(not(test), no_std)]
#![feature(const_option)]

extern crate no_std_compat as std;

pub mod parse;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
