use super::*;

pub fn header(route: &Routes) -> Html<Context, Model>{
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
