use crate::app::search_pane::SearchPane;
use crate::app::status_bar::Message;
use crate::app::AppContext;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeElementProps {
    id: String,
    name: String,
    on_click: Callback<String>,
}

#[function_component(RecipeElement)]
pub fn recipe_element(RecipeElementProps { id, name, on_click }: &RecipeElementProps) -> Html {
    let on_recipe_select = {
        let on_click = on_click.clone();
        let id_cloned = id.clone();
        Callback::from(move |_| on_click.emit(id_cloned.clone()))
    };
    html! {
        <li key={String::from(id)} onclick={on_recipe_select}>{name}</li>
    }
}

#[derive(Clone)]
struct RecipeCreateState {
    clicked: bool,
    recipe_name: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeCreateProps {
    pub refresh_list: Callback<()>,
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
            if let Err(error) =
                ladle::recipe_create(context_cloned.server.as_str(), data.recipe_name.as_str())
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
        props_cloned.refresh_list.emit(());
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

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeListProps {
    pub update: u32,
    pub on_click: Callback<String>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeListState {
    recipes: Vec<ladle::models::RecipeIndex>,
}

#[function_component(RecipeList)]
pub fn recipe_list(props: &RecipeListProps) -> Html {
    let state = use_state(|| RecipeListState { recipes: vec![] });
    let context = use_context::<AppContext>().unwrap_or(AppContext::default());

    let cloned_state = state.clone();
    let context_cloned = context.clone();
    let refresh_list = Callback::from(move |_| {
        let cloned_state = cloned_state.clone();
        let context_cloned = context_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let mut data = cloned_state.deref().clone();
            let fetched_recipes = ladle::recipe_index(context_cloned.server.as_str(), "").await;

            match fetched_recipes {
                Ok(mut index) => {
                    index.sort_by(|lhs, rhs| lhs.name.cmp(&rhs.name));
                    data.recipes = index
                }
                Err(message) => {
                    context_cloned
                        .status
                        .emit(Message::Error(message.to_string(), chrono::Utc::now()));
                    data.recipes = vec![];
                }
            }

            cloned_state.set(data);
        });
    });

    let refresh_list_cloned = refresh_list.clone();
    use_effect_with_deps(move |_| refresh_list_cloned.emit(()), props.update);

    let items = state
        .recipes
        .iter()
        .map(|recipe| {
            html! {
                <RecipeElement
                    id={recipe.id.clone()}
                    name={recipe.name.clone()}
                    on_click={props.on_click.clone()}
                />
            }
        })
        .collect::<Html>();

    html! {
        <div class="recipe-list">
            <SearchPane />
            <ul class="recipe-index">
                {items}
                <RecipeCreateButton
                    refresh_list={refresh_list}
                />
            </ul>
        </div>
    }
}
