pub mod ai;
pub mod rule;
pub mod testplay;

#[cfg(test)]
mod tests {
    use crate::rule::print_board;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        print_board(0x8100000000000081, 0x42c300000000c342, 0);
    }
}
