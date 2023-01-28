use crate::app::search_pane::SearchPane;
use crate::app::status_bar::Message;
use crate::app::AppContext;
use futures::future::join_all;
use std::collections::HashSet;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

fn simplify_name(original: &str) -> String {
    unidecode::unidecode(original).to_lowercase()
}

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
        <li
        key={String::from(id)}
        onclick={on_recipe_select}>{
            name
        }</li>
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
            if let Err(error) = ladle::recipe_create(
                context_cloned.server.as_str(),
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

async fn fetch_recipes_label_union(
    server: &str,
    labels: HashSet<ladle::models::LabelIndex>,
    status: Callback<Message>,
) -> Vec<ladle::models::RecipeIndex> {
    let fetches = labels.iter().map(|l| ladle::label_get(server, &l.id));

    let recipes: HashSet<ladle::models::RecipeIndex> = join_all(fetches)
        .await
        .iter()
        .filter_map(|r| match r {
            Ok(label) => Some(label.tagged_recipes.iter()),
            Err(message) => {
                status.emit(Message::Error(message.to_string(), chrono::Utc::now()));
                None
            }
        })
        .flatten()
        .cloned()
        .collect();

    Vec::from_iter(recipes)
}

async fn fetch_recipes_label_intersection(
    server: &str,
    labels: HashSet<ladle::models::LabelIndex>,
    status: Callback<Message>,
) -> Vec<ladle::models::RecipeIndex> {
    let fetches = labels.iter().map(|l| ladle::label_get(server, &l.id));

    let recipes: Option<HashSet<ladle::models::RecipeIndex>> = join_all(fetches)
        .await
        .iter()
        .filter_map(|r| match r {
            Ok(label) => Some(label.tagged_recipes.iter().cloned().collect()),
            Err(message) => {
                status.emit(Message::Error(message.to_string(), chrono::Utc::now()));
                None
            }
        })
        .reduce(|acc: HashSet<ladle::models::RecipeIndex>, e| {
            acc.intersection(&e).cloned().collect()
        });

    match recipes {
        Some(intersection) => intersection.iter().cloned().collect(),
        _ => vec![],
    }
}

async fn fetch_recipes_index(
    server: &str,
    _labels: HashSet<ladle::models::LabelIndex>,
    status: Callback<Message>,
) -> Vec<ladle::models::RecipeIndex> {
    match ladle::recipe_index(server, "").await {
        Ok(mut index) => {
            index.sort_by(|lhs, rhs| lhs.name.cmp(&rhs.name));
            index
        }
        Err(message) => {
            status.emit(Message::Error(message.to_string(), chrono::Utc::now()));
            vec![]
        }
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeListProps {
    pub update: u32,
    pub on_click: Callback<String>,
}

#[derive(Properties, PartialEq, Clone, Default)]
pub struct RecipeListState {
    recipes: Vec<ladle::models::RecipeIndex>,
    pattern: String,
    selected_labels: HashSet<ladle::models::LabelIndex>,
}

#[function_component(RecipeList)]
pub fn recipe_list(props: &RecipeListProps) -> Html {
    let state = use_state(|| RecipeListState::default());
    let context = use_context::<AppContext>().unwrap_or(AppContext::default());

    let cloned_state = state.clone();
    let context_cloned = context.clone();
    let refresh_list = Callback::from(move |_| {
        let cloned_state = cloned_state.clone();
        let context_cloned = context_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let mut data = cloned_state.deref().clone();
            let mut fetched_recipes = match cloned_state.selected_labels.len() {
                0 => {
                    fetch_recipes_index(
                        context_cloned.server.as_str(),
                        cloned_state.selected_labels.clone(),
                        context_cloned.status,
                    )
                    .await
                }
                _ => {
                    fetch_recipes_label_union(
                        context_cloned.server.as_str(),
                        cloned_state.selected_labels.clone(),
                        context_cloned.status,
                    )
                    .await
                }
            };

            fetched_recipes
                .sort_by(|lhs, rhs| simplify_name(&lhs.name).cmp(&simplify_name(&rhs.name)));
            data.recipes = fetched_recipes;
            cloned_state.set(data);
        });
    });

    let cloned_state = state.clone();
    let update_selected_labels: Callback<HashSet<ladle::models::LabelIndex>> =
        Callback::from(move |selected| {
            let mut data = cloned_state.deref().clone();
            data.selected_labels = selected;
            cloned_state.set(data);
        });

    let refresh_list_cloned = refresh_list.clone();
    use_effect_with_deps(
        move |_| refresh_list_cloned.emit(()),
        (state.selected_labels.clone(), props.update),
    );

    let state_cloned = state.clone();
    let change_pattern = Callback::from(move |pattern: String| {
        let mut data = state_cloned.deref().clone();
        data.pattern = simplify_name(&pattern);
        state_cloned.set(data);
    });

    let items = state
        .recipes
        .iter()
        .filter(|recipe| simplify_name(&recipe.name).contains(&state.pattern))
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
        <div class="recipe-selection">
            <SearchPane
                {update_selected_labels}
                {change_pattern}
                selected_labels={state.selected_labels.clone()}
            />
            <ul class="recipe-index">
                {items}
                <RecipeCreateButton
                    refresh_list={refresh_list}
                />
            </ul>
        </div>
    }
}
