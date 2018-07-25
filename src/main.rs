#[macro_use] extern crate yew;
#[macro_use] extern crate stdweb;
mod ingresseira;
use yew::prelude::*;
use ingresseira::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}