extern crate stdweb;
extern crate yew;

use self::stdweb::web::*;
use self::yew::prelude::*;

use self::yew::services::console::ConsoleService;

use super::icons::*;
use yew_simple::{RouterTask, RouteInfo};

use element_from_html_string::ElementFromHtmlString;
pub struct Context {
    pub console: ConsoleService,
}


pub struct Model {
    value: i64,
    routes: Routes,
    #[allow(dead_code)]
    router: RouterTask<Context, Model>,
}

#[derive(PartialEq)]
pub enum Routes{
    Parties,
    Tickets,
    CreateNewEvent,
    Settings
}

pub enum Msg {
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
    GoToParties,
    GoToTickets,
    GoToCreateNewEvent,
    GoToSettings,
    None,
}

fn handle_route(info: RouteInfo) -> Msg {
    let route = info.url.fragment().unwrap_or("");
    if route.to_ascii_lowercase() == "parties" {
        Msg::GoToParties
    } else if route.to_ascii_lowercase() == "tickets" {
        Msg::GoToTickets
    } else if route.to_ascii_lowercase() == "create_new_event" {
        Msg::GoToCreateNewEvent
    } else if route.to_ascii_lowercase() == "settings" {
        Msg::GoToSettings
    } else {
        Msg::None
    }
}

impl Component<Context> for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, context: &mut Env<Context, Self>) -> Self {
        Model {
            value: 0,
            routes: Routes::Parties,
            router: RouterTask::new(context, &handle_route),
        }
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.value = self.value + 1;
                context.console.log("plus one");
                return true;
            }
            Msg::Decrement => {
                self.value = self.value - 1;
                context.console.log("minus one");
                return true;
            }
            Msg::Bulk(list) => for msg in list {
                self.update(msg, context);
                context.console.log("Bulk action");
            },
            Msg::None => {
                context.console.log("No action");
                return false;
            },
            Msg::GoToParties => {
                self.routes = Routes::Parties;
                return true
            },
            Msg::GoToTickets => {
                self.routes = Routes::Tickets;
                return true
            },
            Msg::GoToCreateNewEvent => {
                self.routes = Routes::CreateNewEvent;
                return true
            },
            Msg::GoToSettings => {
                self.routes = Routes::Settings;
                return true
            }
        }
        return true;
    }
}


fn header(route: &Routes) -> Html<Context, Model>{
    let default_color = "black";
    let active_color = "deepskyblue";
    let parties_icon_color;
    let tickets_icon_color;
    let new_event_icon_color;
    let settings_icon_color;
    parties_icon_color = if route == &Routes::Parties {active_color} else {default_color};
    tickets_icon_color = if route == &Routes::Tickets {active_color} else {default_color};
    new_event_icon_color = if route == &Routes::CreateNewEvent {active_color} else {default_color};
    settings_icon_color = if route == &Routes::Settings {active_color} else {default_color};
    html! {
        <header>
            <div class="main-nav-organizer",>
                <a href="#parties", class="navlink",>
                    {
                        ElementFromHtmlString(parties_icon("main-nav-items", parties_icon_color)).view()
                    }
                </a>
                <a href="#tickets", class="navlink",>
                    {
                        ElementFromHtmlString(tickets_icon("main-nav-items", tickets_icon_color)).view()
                    }
                </a>
                <a href="#create_new_event", class="navlink",>
                    {
                        ElementFromHtmlString(new_event_icon("main-nav-items", new_event_icon_color)).view()
                    }
                </a>
                <a href="#settings", class="navlink",>
                    {
                        ElementFromHtmlString(setting_icon("main-nav-items", settings_icon_color)).view()
                    }
                </a>
            </div>
        </header>
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <div>
                {header(&self.routes)}
                <nav class="menu",>
//                    <button onclick=|_| Msg::Increment,>{ "Increment" }</button>
//                    <button onclick=|_| Msg::Decrement,>{ "Decrement" }</button>
//                    <button onclick=|_| Msg::Bulk(vec![Msg::Increment, Msg::Increment]),>{ "Increment Twice" }</button>
                </nav>
            </div>
        }
    }
}