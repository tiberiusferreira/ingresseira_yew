use super::*;

pub struct PartiesList {

}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Props{
    pub event: Event
}

impl Component<Context>  for PartiesList{
    type Message = ();
    type Properties = Props;

    fn create(_props: Self::Properties, _context: &mut Env<Context, Self>) -> Self {
        PartiesList{

        }
    }

    fn update(&mut self, msg: Self::Message, _context: &mut Env<Context, Self>) -> ShouldRender {
        true
    }
}

impl Renderable<Context, PartiesList> for PartiesList {
    fn view(&self) -> Html<Context, PartiesList> {
        html! {
                <div class="event-posts-container",>
                    <div class="event-container",>
                        <div class="event-detail",>
                            <h2>{"this.props.event.title"}</h2>
                            <h3>{"this.props.event.date"}</h3>
                            <h4>{"this.props.event.place"}</h4>
                            <h4>{"this.props.event.sales_place"}</h4>
                            <h2>{"R$ this.props.event.price"}</h2>
                        </div>
                    </div>
                </div>
        }
    }
}

