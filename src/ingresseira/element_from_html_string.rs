use yew::{prelude::*, virtual_dom::VNode};
use stdweb::{unstable::TryFrom, web::Node};

pub struct ElementFromHtmlString(pub String);

impl<M> Renderable<M> for ElementFromHtmlString
    where M: Component
{
    fn view(&self) -> Html<M> {
        let js_svg = js! {
             var template = document.createElement("template");
             template.innerHTML = @{self.0.to_string()};
             return template.content.firstChild;
        };
        let node = Node::try_from(js_svg).expect("convert js_svg");
        let vnode = VNode::VRef(node);
        vnode
    }
}