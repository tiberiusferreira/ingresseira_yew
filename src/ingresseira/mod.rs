extern crate stdweb;
extern crate yew;
mod element_from_html_string;
mod header;
mod icons;
mod parties_list;
use self::parties_list::PartiesList;
use yew::{prelude::*, services::console::ConsoleService};
use yew_simple::{RouterTask, RouteInfo};
use self::icons::*;
use self::header::*;
use self::element_from_html_string::ElementFromHtmlString;
use std::fmt;

pub struct Context {
    pub console: ConsoleService,
}

pub struct Model {
    routes: Routes,
    #[allow(dead_code)]
    router: RouterTask<Context, Model>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Event {
    date: String,
    description: String,
    id: u64,
    image_url: String,
    image_alt: String,
    place: String,
    price: f64,
    sales_place: String,
    title: String,
    toggled: bool
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


impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        let event = Event{
            date: String::new(),
            description: String::new(),
            id: 0,
            image_url: String::new(),
            image_alt: String::new(),
            place: String::new(),
            price: 0.0,
            sales_place: String::new(),
            title: String::new(),
            toggled: false,
        };
        html! {
            <div>
                {header(&self.routes)}
                <PartiesList: event=event,/>
            </div>
        }
    }
}