extern crate stdweb;
extern crate failure;
extern crate serde_json;
mod element_from_html_string;
mod header;
mod icons;
mod parties_list;
mod services;

use self::parties_list::PartiesList;
use yew::{prelude::*, format::{Json, Text}, services::{console::ConsoleService, fetch::*}};
use self::icons::*;
use self::header::*;
use self::element_from_html_string::ElementFromHtmlString;
use std::fmt;
use yew::format::Nothing;

pub struct RouterModel {
    routes: Routes,
    #[allow(unused)]
    fetch_task: FetchTask,
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

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
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
}

pub enum Msg {
    GoToParties,
    GoToTickets,
    GoToCreateNewEvent,
    GoToSettings,
    None,
    Fetch(Response<Text>),
//    Fetch(Response<Json<Result<Vec<Event>, failure::Error>>>),
}


//
//fn handle_route(info: RouteInfo) -> Msg {
//    let route = info.url.fragment().unwrap_or("");
//    console!(log,format!("New route: {}", route));
//    if route == Routes::Parties.to_string() {
//        Msg::GoToParties
//    } else if route == Routes::Tickets.to_string() {
//        Msg::GoToTickets
//    } else if route == Routes::CreateNewEvent.to_string() {
//        Msg::GoToCreateNewEvent
//    } else if route == Routes::Settings.to_string() {
//        Msg::GoToSettings
//    } else {
//        Msg::None
//    }
//}

impl Component for RouterModel {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties,  link: ComponentLink<Self>) -> Self {
        let request = Request::get("http://localhost:1024/api/events")
            .body(Nothing)
            .expect("Could not create body");
        let task = FetchService::new()
            .fetch(request, link.send_back(Msg::Fetch));


        RouterModel {
            routes: Routes::Parties,
            fetch_task: task
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
//        fn test(response: Response<Result<String,Error>>){
//
//        };
//        let callback = context.send_back(|response: Response<String>| {Msg::None});
//        context.fetch.fetch(
//            get_request,
//            callback
//        );

        match msg {
            Msg::None => {
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
            },
            Msg::Fetch(response) => {
                let body = &response.body();
                match body{
                    Ok(actual_body) => {
                        console!(log, format!("Got response: {:?}", actual_body));
                        match serde_json::from_str::<Vec<Event>>(actual_body){
                            Ok(events) => {
                                console!(log, format!("Parsed to events: {:?}", events));
                            },
                            Err(e) => {
                                console!(log, format!("Error parsing to events: {:?}", e));
                            },
                        }

                    },
                    Err(e) => {
                        console!(log, format!("Got error: {}", e));
                    }
                }
                return true;
            }
        }
    }
}


impl RouterModel{
    fn get_view_for_route(&self) -> Html<RouterModel>{
        let event = Event{
            date: "23/12".to_string(),
            description: "dadw".to_string(),
            id: 0,
            image_url: "https://s3-us-west-2.amazonaws.com/pixel-solutions/event/banner/e2b5aa26eafeeee141566a642e634526.jpg".to_string(),
            image_alt: "Img Alt".to_string(),
            place: "Gringos House".to_string(),
            price: 20.0,
            sales_place: "Casa da Tia Rita".to_string(),
            title: "Evento Topzera".to_string(),
        };

        match self.routes{
            Routes::Parties => {
                html!{
                    <PartiesList: event=event,/>
                }
            },
            Routes::Tickets => {html!{<span>{"Tickets"}<span/>}},
            Routes::CreateNewEvent => {html!{<span>{"New event"}<span/>}},
            Routes::Settings => {html!{<span>{"Settings"}<span/>}},
        }
    }
}
impl Renderable<RouterModel> for RouterModel {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                {header(&self.routes)}
                {self.get_view_for_route()}
            </div>
        }
    }
}