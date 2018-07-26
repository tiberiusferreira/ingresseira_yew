#[macro_use] extern crate yew;
#[macro_use] extern crate stdweb;
extern crate yew_simple;

mod ingresseira;
mod icons;
mod element_from_html_string;
use yew::prelude::*;
use ingresseira::{Model, Context};
use self::yew::services::console::ConsoleService;

fn main() {
    yew::initialize();

    let context = Context {
        console: ConsoleService::new(),
    };

    let app: App<_, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}