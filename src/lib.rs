pub mod ai;
pub mod rule;
pub mod testplay;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        println!("{:?}", crate::rule::encode_move("H8".to_string()));
    }
}
