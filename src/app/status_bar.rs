use chrono::{DateTime, Utc};
use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(PartialEq, Clone, Debug)]
pub enum Message {
    None,
    Success(String, DateTime<Utc>),
    Info(String, DateTime<Utc>),
    Error(String, DateTime<Utc>),
}

#[derive(Properties, PartialEq, Clone)]
pub struct StatusBarProps {
    pub current: Message,
}

#[derive(Properties, PartialEq, Clone)]
pub struct StatusBarState {
    pub shown: bool,
}

#[function_component(StatusBar)]
pub fn status_bar(props: &StatusBarProps) -> Html {
    let state = use_state(|| StatusBarState { shown: false });

    {
        let state = state.clone();
        let props_cloned = props.clone();
        use_effect_with_deps(
            move |_| {
                match props_cloned.current {
                    Message::Error(_, _) | Message::Info(_, _) | Message::Success(_, _) => {
                        state.set(StatusBarState { shown: true })
                    }
                    _ => (),
                };
            },
            props.clone(),
        )
    }

    {
        let state = state.clone();
        use_debounce_effect_with_deps(
            move || {
                state.set(StatusBarState { shown: false });
            },
            2000,
            props.clone(),
        );
    };

    let class = format!(
        "status {}",
        match state.shown {
            true => "",
            false => "hidden",
        }
    );

    match &props.current {
        Message::None => html! {},
        Message::Success(message, _) => {
            html! {<div class={format!("{} success", class)}>{message.as_str()}</div>}
        }
        Message::Info(message, _) => {
            html! {<div class={format!("{} info", class)}>{message.as_str()}</div>}
        }
        Message::Error(message, _) => {
            html! {<div class={format!("{} error", class)}>{message.as_str()}</div>}
        }
    }
}
