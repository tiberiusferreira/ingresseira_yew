extern crate stdweb;
extern crate failure;
extern crate serde_json;
mod element_from_html_string;
mod header;
mod icons;
mod common_objs;
mod parties_list;
mod services;
use self::common_objs::*;
use std::str::FromStr;
use std::string::ToString;
use self::parties_list::PartiesList;
use yew::{prelude::*, format::{Text}, services::fetch};
use self::icons::*;
use self::header::*;
use self::element_from_html_string::ElementFromHtmlString;
use yew::format::Nothing;
use ingresseira::services::router::*;
use ingresseira::services::context_provider::*;
use ingresseira::services::router::Request as RouterRequest;

pub struct RouterModel {
    routes: Routes,
    #[allow(unused)]
    fetch_task: fetch::FetchTask,
    router: Box<Bridge<Router<()>>>,
    context_provider_link: Box<Bridge<ContextProvider>>,
}


pub enum Msg {
    GoToParties,
    GoToTickets,
    GoToCreateNewEvent,
    GoToSettings,
    Fetch(fetch::Response<Text>),
    HandleRoute(Route<()>),
    HandleContextChange(Context)
}


impl Component for RouterModel {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties,  mut link: ComponentLink<Self>) -> Self {
        let request = fetch::Request::get("http://localhost:1024/api/events")
            .body(Nothing)
            .expect("Could not create body");
        let task = fetch::FetchService::new()
            .fetch(request, link.send_back(Msg::Fetch));
        let callback = link.send_back(|route: Route<()>| Msg::HandleRoute(route));
        let mut router = Router::bridge(callback);

        let cxt_callback = link.send_back(|ctx: Context| Msg::HandleContextChange(ctx));
        let context_provider_link = ContextProvider::bridge(cxt_callback);

        router.send(RouterRequest::GetCurrentRoute);

        RouterModel {
            routes: Routes::Parties,
            fetch_task: task,
            router,
            context_provider_link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GoToParties => {
                self.router.send(RouterRequest::ChangeRoute(Route{
                    path_segments: vec!(Routes::Parties.to_string()),
                    query: None,
                    fragment: None,
                    state: (),
                }));
                return false
            },
            Msg::GoToTickets => {
                self.router.send(RouterRequest::ChangeRoute(Route{
                    path_segments: vec!(Routes::Tickets.to_string()),
                    query: None,
                    fragment: None,
                    state: (),
                }));
                return false
            },
            Msg::GoToCreateNewEvent => {
                self.router.send(RouterRequest::ChangeRoute(Route{
                    path_segments: vec!(Routes::CreateNewEvent.to_string()),
                    query: None,
                    fragment: None,
                    state: (),
                }));
                return false
            },
            Msg::GoToSettings => {
                self.router.send(RouterRequest::ChangeRoute(Route{
                    path_segments: vec!(Routes::Settings.to_string()),
                    query: None,
                    fragment: None,
                    state: (),
                }));
                return false
            },
            Msg::Fetch(response) => {
                let body = &response.body();
                match body{
                    Ok(actual_body) => {
                        console!(log, format!("Got response: {:?}", actual_body));
                        match serde_json::from_str::<Vec<Event>>(actual_body){
                            Ok(events) => {
                                console!(log, format!("Parsed to events: {:?}", events));
                                self.context_provider_link.send(ContextRequest::EventsUpdate(events));
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
            },
            Msg::HandleRoute(route) => {
                let new_route = route.path_segments
                    .first()
                    .map(|s| s.to_owned())
                    .unwrap_or("".to_owned());
                console!(log, format!("New route = {}", new_route));
                if let Ok(parsed_route) = Routes::from_str(&new_route){
                    self.routes = parsed_route;
                    return true
                }else{
                    false
                }
            },
            Msg::HandleContextChange(cxt) => {
                console!(log, format!("New context!"));
                false
            }
        }
    }
}


impl RouterModel{
    fn get_view_for_route(&self) -> Html<RouterModel>{
        match self.routes{
            Routes::Parties => {
                html!{
                    <PartiesList: />
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