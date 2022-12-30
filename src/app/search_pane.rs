use crate::app::status_bar::Message;
use crate::app::AppContext;
use std::ops::Deref;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SearchPaneProps {}

#[derive(PartialEq, Clone, Default)]
struct SearchPaneState {
    labels: Vec<ladle::models::Label>,
    label_tray_shown: bool,
}

#[function_component(SearchPane)]
pub fn search_pane() -> Html {
    let state = use_state(|| SearchPaneState::default());
    let context = use_context::<AppContext>().unwrap_or(AppContext::default());

    let cloned_state = state.clone();
    let context_cloned = context.clone();
    let refresh_labels = Callback::from(move |_| {
        let cloned_state = cloned_state.clone();
        let context_cloned = context_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let mut data = cloned_state.deref().clone();
            let fetched_labels = ladle::label_index(context_cloned.server.as_str(), "").await;

            match fetched_labels {
                Ok(mut index) => {
                    index.sort_by(|lhs, rhs| lhs.name.cmp(&rhs.name));
                    data.labels = index
                }
                Err(message) => {
                    context_cloned
                        .status
                        .emit(Message::Error(message.to_string(), chrono::Utc::now()));
                    data.labels = vec![];
                }
            }

            cloned_state.set(data);
        });
    });

    use_effect_with_deps(move |_| refresh_labels.emit(()), context.server.clone());

    let labels = state
        .labels
        .iter()
        .map(|l| html! {<li key={l.id.as_str()} class="label filter add">{l.name.clone()}</li>})
        .collect::<Html>();

    let cloned_state = state.clone();
    let toggle_tray = Callback::from(move |_| {
        let mut data = cloned_state.deref().clone();
        data.label_tray_shown = !data.label_tray_shown;
        cloned_state.set(data);
    });

    html! {
        <div class="search-pane">
            <div class="search-header">
                <div class="search-bar">
                </div>
                <button class="label-tray-toggle" onclick={toggle_tray}>{"labels"}</button>
            </div>
            <ul class={format!("available-labels {}", if state.label_tray_shown {"shown"} else {"hidden"})}>
                {labels}
            </ul>
        </div>
    }
}