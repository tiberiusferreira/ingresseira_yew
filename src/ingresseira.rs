extern crate stdweb;
extern crate yew;
use yew::{prelude::*, services::console::ConsoleService};
use yew_simple::{RouterTask, RouteInfo};
use super::icons::*;
use element_from_html_string::ElementFromHtmlString;
use std::fmt;

pub struct Context {
    pub console: ConsoleService,
}

pub struct Model {
    routes: Routes,
    #[allow(dead_code)]
    router: RouterTask<Context, Model>,
}

#[derive(Debug, PartialEq)]
pub enum Routes{
    Parties,
    Tickets,
    CreateNewEvent,
    Settings
}

impl fmt::Display for Routes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq)]
pub enum Msg {
    GoToParties,
    GoToTickets,
    GoToCreateNewEvent,
    GoToSettings,
    None,
}

fn handle_route(info: RouteInfo) -> Msg {
    let route = info.url.fragment().unwrap_or("");
    if route == Routes::Parties.to_string() {
        Msg::GoToParties
    } else if route == Routes::Tickets.to_string() {
        Msg::GoToTickets
    } else if route == Routes::CreateNewEvent.to_string() {
        Msg::GoToCreateNewEvent
    } else if route == Routes::Settings.to_string() {
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
            routes: Routes::Parties,
            router: RouterTask::new(context, &handle_route),
        }
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
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
                <a href={format!("#{}", Routes::Parties.to_string())}, class="navlink",>
                    {
                        ElementFromHtmlString(parties_icon("main-nav-items", parties_icon_color)).view()
                    }
                </a>
                <a href={format!("#{}", Routes::Tickets.to_string())}, class="navlink",>
                    {
                        ElementFromHtmlString(tickets_icon("main-nav-items", tickets_icon_color)).view()
                    }
                </a>
                <a href={format!("#{}", Routes::CreateNewEvent.to_string())}, class="navlink",>
                    {
                        ElementFromHtmlString(new_event_icon("main-nav-items", new_event_icon_color)).view()
                    }
                </a>
                <a href={format!("#{}", Routes::Settings.to_string())}, class="navlink",>
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