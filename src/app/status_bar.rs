use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub enum Message {
    None,
    Info(String),
    Error(String),
}

#[derive(Properties, PartialEq, Clone)]
pub struct StatusBarProps {
    pub current: Message,
}

#[function_component(StatusBar)]
pub fn status_bar(props: &StatusBarProps) -> Html {
    match &props.current {
        Message::None => html! {},
        Message::Info(message) => html! {<div class={"status info"}>{message.as_str()}</div>},
        Message::Error(message) => html! {<div class={"status error"}>{message.as_str()}</div>},
    }
}
