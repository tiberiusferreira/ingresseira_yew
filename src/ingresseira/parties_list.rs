use super::*;

pub struct PartiesList {
    event: Event,
    toggled: bool
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Props{
    pub event: Event
}

#[derive(Debug, PartialEq, Clone)]
pub enum Message{
    Toggle
}

impl Component  for PartiesList{
    type Message = Message;
    type Properties = Props;

    fn create(props: Self::Properties,  _: ComponentLink<Self>) -> Self {
        PartiesList{
            event: props.event,
            toggled: false,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        self.toggled = !self.toggled;
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.event = props.event;
        true
    }
}

impl Renderable<PartiesList> for PartiesList {
    fn view(&self) -> Html<PartiesList> {
        html! {
                <div class="event-posts-container",>
                    <div class={if self.toggled {"expanded-event-container"} else {"event-container"}}, onclick=|_| Message::Toggle,>
                        <div class="event-detail",>
                            <h2>{&self.event.title}</h2>
                            <h3>{&self.event.date}</h3>
                            <h4>{&self.event.place}</h4>
                            <h4>{&self.event.sales_place}</h4>
                            <h2>{format!("R$ {}", &self.event.price)}</h2>
                        </div>
                        <img class={"event-img"}, src={&self.event.image_url}, alt={&self.event.image_alt},/>
                        <div style={if self.toggled {"display:block"} else {"display:none"}}, class="expanded-event-details",>
                            {&self.event.description}
                        </div>
                    </div>
                </div>
        }
    }
}

