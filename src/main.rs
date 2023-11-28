use crate::test_target::Guest;

mod closure;
mod closure_2;
mod debug_trait;
mod default_trait;
mod display_trait;
mod iter;
mod partial_eq;
mod summarize;
mod test_target;
mod traits;
mod vers;
mod web;

fn main() {
    // vers::run();
    // traits::run();
    // display_trait::run();
    // debug_trait::run();
    // default_trait::run();
    //
    // let guest = Guest::new(33, true);
    // println!("{:?}", guest.calc_fee())
    // summarize::run();
    // iter::run();
    web::run();
}
