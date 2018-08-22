#[macro_use] extern crate yew;
#[macro_use] extern crate stdweb;
#[macro_use] extern crate serde_derive;
extern crate serde;
mod ingresseira;
use ingresseira::{RouterModel};

fn main() {
    yew::initialize();

    yew::app::App::<RouterModel>::new().mount_to_body();

    yew::run_loop();
}