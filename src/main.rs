use RpnCalc1::eval;
//pub mod module_a;
use RpnCalc1::module_a::*;
//use new_crate::module_a;
fn main() {
    let formula = "3 3 + 3 *";
    let res = eval(formula);
    println!("res: {}", res);
    pit();
}
