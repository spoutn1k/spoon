use crate::app::status_bar::Message;
use crate::app::AppContext;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Clone)]
struct RecipeCreateState {
    clicked: bool,
    recipe_name: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeCreateProps {
    pub refresh_recipes: Callback<()>,
}

#[function_component(RecipeCreateButton)]
pub fn recipe_create_button(props: &RecipeCreateProps) -> Html {
    let state = use_state(|| RecipeCreateState {
        clicked: false,
        recipe_name: String::from(""),
    });

    let context = use_context::<AppContext>().unwrap_or(AppContext::default());

    let cloned_state = state.clone();
    let label_clicked = Callback::from(move |_| {
        let mut data = cloned_state.deref().clone();
        data.clicked = true;
        cloned_state.set(data)
    });

    let cloned_state = state.clone();
    let name_changed = Callback::from(move |e: Event| {
        let target: EventTarget = e.target().expect("Error accessing name input");
        let name = target.unchecked_into::<HtmlInputElement>().value();
        let mut data = cloned_state.deref().clone();
        data.recipe_name = name;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let context_cloned = context.clone();
    let props_cloned = props.clone();
    let name_submit = Callback::from(move |_| {
        let mut data = cloned_state.deref().clone();
        let context_cloned = context_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if let Err(error) = ladle::recipe_create(
                context_cloned.settings.server_url.as_str(),
                data.recipe_name.as_str(),
                "",
                "",
                "",
            )
            .await
            {
                context_cloned
                    .status
                    .emit(Message::Error(error.to_string(), chrono::Utc::now()));
            };
        });
        data.clicked = false;
        data.recipe_name = String::default();
        cloned_state.set(data);
        props_cloned.refresh_recipes.emit(());
    });

    match (*state).clicked {
        false => html! {
            <li key="new" onclick={label_clicked}>
                {"Add recipe"}
            </li>
        },
        true => html! {
            <li key="new">
                <input type="text" value={state.recipe_name.clone()} onchange={name_changed} />
                <input type="submit" onclick={name_submit}/>
            </li>
        },
    }
}
