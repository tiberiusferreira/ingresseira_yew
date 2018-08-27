use yew::prelude::worker;
use yew::prelude::worker::Context as ContextAgent;
use std::collections::HashSet;
use ::ingresseira::common_objs::*;
/// The Router worker holds on to the RouteService singleton and mediates access to it.

pub struct ContextProvider
{
    link: worker::AgentLink<ContextProvider>,
    context: Context,
    /// A list of all entities connected to the router.
    /// When a route changes, either initiated by the browser or by the app,
    /// the route change will be broadcast to all listening entities.
    subscribers: HashSet<worker::HandlerId>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Msg {

}

#[derive(Serialize, Deserialize, Debug)]
pub enum ContextRequest {
    GetContext,
    EventsUpdate(Vec<Event>),
    ToggleEvent(u64)
}

impl worker::Transferable for ContextRequest {

}

impl worker::Transferable for Context{

}

impl worker::Agent for ContextProvider
{
    type Reach = ContextAgent;
    type Message = Msg;
    type Input = ContextRequest;
    type Output = Context;

    fn create(link: worker::AgentLink<Self>) -> Self {
        ContextProvider {
            link,
            context: Context{
                events: Vec::new()
            },
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
//            Msg::BrowserNavigationRouteChanged((_route_string, state)) => {
//                info!("Browser navigated");
//                let mut route = Route::current_route(&self.route_service);
//                route.state = state;
//                for sub in self.subscribers.iter() {
//                    self.link.response(*sub, route.clone());
//                }
            }
        }
//    }

    fn connected(&mut self, id: worker::HandlerId) {
        self.subscribers.insert(id);
    }

    fn handle(&mut self, msg: Self::Input, who: worker::HandlerId) {
        info!("Request: {:?}", msg);
        match msg {
            ContextRequest::GetContext => {
                self.link.response(who, self.context.clone())
            },
            ContextRequest::EventsUpdate(events) => {
                self.context.events = events;
                for sub in self.subscribers.iter() {
                    self.link.response(*sub, self.context.clone());
                }
            },
            ContextRequest::ToggleEvent(id) =>{
                self.context.events.iter_mut()
                    .filter(|event| event.id == id)
                    .for_each(|event| event.toggled = !event.toggled);
                for sub in self.subscribers.iter() {
                    self.link.response(*sub, self.context.clone());
                }
            }
//            Request::ChangeRoute(route) => {
//                let route_string: String = route.to_route_string();
//                // set the route
//                self.route_service.set_route(&route_string, route.state);
//                // get the new route. This will contain a default state object
//                let route = Route::current_route(&self.route_service);
//                // broadcast it to all listening components
//                for sub in self.subscribers.iter() {
//                    self.link.response(*sub, route.clone());
//                }
//            }
//            Request::ChangeRouteNoBroadcast(route) => {
//                let route_string: String = route.to_route_string();
//                self.route_service.set_route(&route_string, route.state);
//            }
//            Request::GetCurrentRoute => {
//                let route = Route::current_route(&self.route_service);
//                self.link.response(who, route.clone());
//            }
        }
    }

    fn disconnected(&mut self, id: worker::HandlerId) {
        self.subscribers.remove(&id);
    }
}