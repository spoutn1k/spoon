use chrono::{DateTime, Utc};
use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(PartialEq, Clone)]
pub enum Message {
    None,
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
    pub message: Option<Message>,
}

#[function_component(StatusBar)]
pub fn status_bar(props: &StatusBarProps) -> Html {
    let state = use_state(|| StatusBarState {
        shown: false,
        message: None,
    });

    {
        let state = state.clone();
        let props_cloned = props.clone();
        use_effect_with_deps(
            move |_| {
                log::debug!("Attempting to show");
                match props_cloned.current {
                    Message::Error(_, _) | Message::Info(_, _) => state.set(StatusBarState {
                        shown: true,
                        message: Some(props_cloned.current.clone()),
                    }),
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
                log::debug!("Hiding");
                state.set(StatusBarState {
                    shown: false,
                    message: None,
                });
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
        Message::Info(message, _) => html! {<div class={"status info"}>{message.as_str()}</div>},
        Message::Error(message, _) => {
            html! {<div class={format!("{} error", class)}>{message.as_str()}</div>}
        }
    }
}
