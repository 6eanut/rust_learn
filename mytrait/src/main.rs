use mytrait::fn1;
use mytrait::fn2;
use mytrait::Dayin;
use mytrait::S1;
use mytrait::S2;
fn main() {
    let s1 = S1 {
        name: String::from("check"),
        id: 20,
    };
    s1.dayin();
    s1.error();
    let s2 = S2 {
        name: String::from("check02"),
        checked: true,
    };
    s2.dayin();
    s2.error();

    fn1(s1);
    fn2(s2);
}
