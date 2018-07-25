extern crate stdweb;
extern crate yew;

use self::stdweb::web::*;

use self::yew::prelude::*;
use self::yew::virtual_dom::*;
use self::yew::services::ConsoleService;
use stdweb::unstable::TryFrom;
use super::svg_buttons::*;
pub struct Model {
    console: ConsoleService,
    value: i64,
}

pub enum Msg {
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value = self.value + 1;
                self.console.log("plus one");
            }
            Msg::Decrement => {
                self.value = self.value - 1;
                self.console.log("minus one");
            }
            Msg::Bulk(list) => for msg in list {
                self.update(msg);
                self.console.log("Bulk action");
            },
        }
        true
    }
}


struct ElementFromHtmlString(String);
impl<U> Renderable<U> for ElementFromHtmlString
    where
        U: Component
{
    fn view(&self) -> Html<U> {
        let js_svg = js! {
             var template = document.createElement("template");
             template.innerHTML = @{self.0.to_string()};
             return template.content.firstChild;
        };
        let node = Node::try_from(js_svg).expect("convert js_svg");
        let vnode = VNode::VRef(node);
        vnode
    }
}

fn header() -> Html<Model>{
    html! {
        <header>
            <div class="main-nav-organizer",>
                <a class="navlink",>
                    {ElementFromHtmlString(setting_svg("main-nav-items", "black")).view()}
                </a>
                <a class="navlink",>
                    {ElementFromHtmlString(ticket_svg("main-nav-items", "black")).view()}
                </a>
                <a class="navlink",>
                    {ElementFromHtmlString(edit_svg("main-nav-items", "black")).view()}
                </a>
                <a class="navlink",>
                    {ElementFromHtmlString(cone_svg("main-nav-items", "black")).view()}
                </a>
            </div>
        </header>
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                {header()}
                <nav class="menu",>
                    <button onclick=|_| Msg::Increment,>{ "Increment" }</button>
                    <button onclick=|_| Msg::Decrement,>{ "Decrement" }</button>
                    <button onclick=|_| Msg::Bulk(vec![Msg::Increment, Msg::Increment]),>{ "Increment Twice" }</button>
                </nav>
                <p>{ self.value }</p>
                <p>{ Date::new().to_string() }</p>
            </div>
        }
    }
}