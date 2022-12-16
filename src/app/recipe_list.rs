use crate::app::status_bar::Message;
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
    pub url: String,
    pub refresh_list: Callback<()>,
    pub status: Callback<Message>,
}

#[function_component(RecipeCreateButton)]
pub fn recipe_create_button(props: &RecipeCreateProps) -> Html {
    let state = use_state(|| RecipeCreateState {
        clicked: false,
        recipe_name: String::from(""),
    });

    let cloned_state = state.clone();
    let label_clicked = Callback::from(move |_| {
        let mut data = cloned_state.deref().clone();
        data.clicked = true;
        cloned_state.set(data)
    });

    let cloned_state = state.clone();
    let name_changed = Callback::from(move |e: Event| {
        let target: EventTarget = e.target().expect("Error accessing pattern input");
        let name = target.unchecked_into::<HtmlInputElement>().value();
        let mut data = cloned_state.deref().clone();
        data.recipe_name = name;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let cloned_props = props.clone();
    let name_submit = Callback::from(move |_| {
        let mut data = cloned_state.deref().clone();
        let cloned_props = cloned_props.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if let Err(error) =
                ladle::recipe_create(cloned_props.url.as_str(), data.recipe_name.as_str()).await
            {
                cloned_props.status.emit(Message::Error(error.to_string()));
            };
        });
        data.clicked = false;
        data.recipe_name = String::default();
        cloned_state.set(data);
        cloned_props.refresh_list.emit(());
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
    pub url: String,
    pub update: u32,
    pub on_click: Callback<String>,
    pub status: Callback<Message>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeListState {
    pattern: String,
    recipes: Vec<ladle::models::Recipe>,
}

#[function_component(RecipeList)]
pub fn recipe_list(props: &RecipeListProps) -> Html {
    let state = use_state(|| RecipeListState {
        pattern: String::default(),
        recipes: vec![],
    });

    let cloned_state = state.clone();
    let on_pattern_change = Callback::from(move |e: InputEvent| {
        let target: EventTarget = e.target().expect("Error accessing pattern input");
        let mut data = cloned_state.deref().clone();
        data.pattern = target.unchecked_into::<HtmlInputElement>().value();
        cloned_state.set(data);
    });

    let props_cloned = props.clone();
    let cloned_state = state.clone();

    let refresh_list = Callback::from(move |_| {
        let props_cloned = props_cloned.clone();
        let cloned_state = cloned_state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let mut data = cloned_state.deref().clone();
            let fetched_recipes =
                ladle::recipe_index(props_cloned.url.as_str(), data.pattern.as_str()).await;

            match fetched_recipes {
                Ok(index) => data.recipes = index,
                Err(message) => {
                    props_cloned
                        .status
                        .emit(Message::Error(message.to_string()));
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
        .filter(|recipe| {
            unidecode::unidecode(&recipe.name.to_lowercase())
                .contains(&state.pattern.to_lowercase())
        })
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
            <input type="text"
                oninput={on_pattern_change}
                value={state.pattern.clone()}
            />
            <ul>
                {items}
                <RecipeCreateButton
                    url={props.url.clone()}
                    refresh_list={refresh_list}
                    status={props.status.clone()}
                />
            </ul>
        </div>
    }
}
