use testlib::{self, check_number};

mod common;
#[test]
fn test00_00() {
    common::check();
    let a = -1;
    let g = check_number(a);
    g.print_guess();
}
