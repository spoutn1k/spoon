use crate::app::status_bar::Message;
use std::ops::Deref;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SearchPaneProps {
    pub url: String,
    pub status: Callback<Message>,
}

#[derive(PartialEq, Clone)]
pub struct SearchPaneState {
    pub labels: Vec<ladle::models::Label>,
}

#[function_component(SearchPane)]
pub fn search_pane(props: &SearchPaneProps) -> Html {
    let state = use_state(|| SearchPaneState { labels: vec![] });
    let props_cloned = props.clone();
    let cloned_state = state.clone();

    let refresh_labels = Callback::from(move |_| {
        let props_cloned = props_cloned.clone();
        let cloned_state = cloned_state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let mut data = cloned_state.deref().clone();
            let fetched_labels = ladle::label_index(props_cloned.url.as_str(), "").await;

            match fetched_labels {
                Ok(mut index) => {
                    index.sort_by(|lhs, rhs| lhs.name.cmp(&rhs.name));
                    data.labels = index
                }
                Err(message) => {
                    props_cloned
                        .status
                        .emit(Message::Error(message.to_string(), chrono::Utc::now()));
                    data.labels = vec![];
                }
            }

            cloned_state.set(data);
        });
    });

    use_effect_with_deps(move |_| refresh_labels.emit(()), props.clone());

    let labels = state
        .labels
        .iter()
        .map(|l| html! {<li key={l.id.as_str()} class="label filter add">{l.name.clone()}</li>})
        .collect::<Html>();

    html! {
        <div class="search-pane">
            <div class="search-bar">
            </div>
            <ul class="available-labels">
                {labels}
            </ul>
        </div>
    }
}
