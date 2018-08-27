use super::*;

pub struct PartiesList {
    context_provider_link: Box<Bridge<ContextProvider>>,
    events: Vec<Event>,
}


#[derive(Debug, Clone)]
pub enum Message{
    Toggle(u64),
    HandleContextChange(Context)
}

impl Component  for PartiesList{
    type Message = Message;
    type Properties = ();

    fn create(props: Self::Properties,  mut link: ComponentLink<Self>) -> Self {
        let cxt_callback = link.send_back(|ctx: Context| Message::HandleContextChange(ctx));
        let mut context_provider_link = ContextProvider::bridge(cxt_callback);
        context_provider_link.send(ContextRequest::GetContext);
        PartiesList{
            context_provider_link,
            events: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::Toggle(id) => {
                self.context_provider_link.send(ContextRequest::ToggleEvent(id));
                return true;
            },
            Message::HandleContextChange(cxt) => {
                self.events = cxt.events;
                return true;
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }
}

impl PartiesList{
    fn render_event(event: &Event) -> Html<PartiesList>{
        let id = event.id;
        html!{
                <div class="event-posts-container",>
                    <div class={if event.toggled {"expanded-event-container"} else {"event-container"}}, onclick=|_| Message::Toggle(id),>
                        <div class="event-detail",>
                            <h2>{&event.title}</h2>
                            <h3>{&event.date}</h3>
                            <h4>{&event.place}</h4>
                            <h4>{&event.sales_place}</h4>
                            <h2>{format!("R$ {}", &event.price)}</h2>
                        </div>
                        <img class={"event-img"}, src={&event.image_url}, alt={&event.image_alt},/>
                        <div style={if event.toggled {"display:block"} else {"display:none"}}, class="expanded-event-details",>
                            {&event.description}
                        </div>
                    </div>
                </div>
        }
    }
}

impl Renderable<PartiesList> for PartiesList {
    fn view(&self) -> Html<PartiesList> {
        html! {
                <div>
                {for self.events.iter().map(Self::render_event)}
                </div>
        }
    }
}

