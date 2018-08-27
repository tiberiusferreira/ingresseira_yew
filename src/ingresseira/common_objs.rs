#[derive(Display, EnumString, Debug, PartialEq)]
pub enum Routes{
    Parties,
    Tickets,
    CreateNewEvent,
    Settings
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Context{
    pub events: Vec<Event>
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Event {
    pub date: String,
    pub description: String,
    pub id: u64,
    pub image_url: String,
    pub image_alt: String,
    pub place: String,
    pub price: f64,
    pub sales_place: String,
    pub title: String,
    #[serde(default)]
    pub toggled: bool,
}