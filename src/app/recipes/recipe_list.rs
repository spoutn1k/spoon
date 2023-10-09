use crate::app::recipes::search_pane::SearchPane;
use crate::app::status_bar::Message;
use crate::app::AppContext;
use crate::app::Route;
use futures::future::join_all;
use ladle::models::RecipeIndex;
use log::debug;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashSet;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;

fn simplify_name(original: &str) -> String {
    unidecode::unidecode(original).to_lowercase()
}

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeElementProps {
    item: RecipeIndex,
}

#[function_component(RecipeElement)]
pub fn recipe_element(
    RecipeElementProps {
        item: RecipeIndex { id, name },
    }: &RecipeElementProps,
) -> Html {
    html! {
        <li class={"recipe-item"} key={id.as_str()}>
            <Link<Route> to={Route::ShowRecipe {id:id.clone()}}>
                {name}
            </Link<Route>>
            <span class={"knife-id"}>{id.as_str()}</span>
        </li>
    }
}

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

/*
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
*/

async fn fetch_recipes_index(
    server: &str,
    _: HashSet<ladle::models::LabelIndex>,
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
pub struct RecipeListProps {}

#[derive(Properties, PartialEq, Clone, Default)]
pub struct RecipeListState {
    recipes: Vec<ladle::models::RecipeIndex>,
    labels: HashSet<ladle::models::LabelIndex>,
    pattern: String,
}

fn deserialize_comma_list<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(String::deserialize(deserializer)?
        .split(",")
        .filter_map(|string| match string.len() {
            0 => None,
            _ => Some(String::from(string)),
        })
        .collect())
}

fn serialize_comma_list<S>(v: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(v.join(",").as_str())
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Filters {
    #[serde(default)]
    #[serde(
        deserialize_with = "deserialize_comma_list",
        serialize_with = "serialize_comma_list"
    )]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub restrictions: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
}

#[function_component(RecipeList)]
pub fn recipe_list(_: &RecipeListProps) -> Html {
    let state = use_state(|| RecipeListState::default());
    let context = use_context::<AppContext>().unwrap_or(AppContext::default());

    let location = use_location().unwrap();
    let parameters = location.query::<Filters>().unwrap_or(Filters::default());
    let selected_labels: HashSet<_> = parameters
        .labels
        .iter()
        .filter_map(|string| state.labels.iter().find(|l| &l.name == string))
        .cloned()
        .collect();

    let cloned_state = state.clone();
    let context_cloned = context.clone();
    let labels = selected_labels.clone();
    let refresh_recipes = Callback::from(move |_| {
        let cloned_state = cloned_state.clone();
        let context_cloned = context_cloned.clone();
        let labels = labels.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let mut data = cloned_state.deref().clone();
            let mut fetched_recipes = match labels.len() {
                0 => {
                    fetch_recipes_index(
                        context_cloned.settings.server_url.as_str(),
                        labels.clone(),
                        context_cloned.status,
                    )
                    .await
                }
                _ => {
                    fetch_recipes_label_union(
                        context_cloned.settings.server_url.as_str(),
                        labels.clone(),
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

    let refresh_recipes_cloned = refresh_recipes.clone();
    use_effect_with_deps(move |_| refresh_recipes_cloned.emit(()), selected_labels);

    let cloned_state = state.clone();
    let context_cloned = context.clone();
    let refresh_labels = Callback::from(move |_| {
        let cloned_state = cloned_state.clone();
        let context_cloned = context_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let mut data = cloned_state.deref().clone();
            let fetched_labels =
                ladle::label_index(context_cloned.settings.server_url.as_str(), "").await;

            match fetched_labels {
                Ok(mut index) => {
                    index.sort_by(|lhs, rhs| lhs.name.cmp(&rhs.name));
                    data.labels = HashSet::from_iter(index.iter().cloned());
                }
                Err(message) => {
                    context_cloned
                        .status
                        .emit(Message::Error(message.to_string(), chrono::Utc::now()));
                    data.labels = HashSet::new();
                }
            }

            cloned_state.set(data);
        });
    });

    use_effect_with_deps(
        move |_| refresh_labels.emit(()),
        context.settings.server_url.clone(),
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
                    item={recipe.clone()}
                />
            }
        })
        .collect::<Html>();

    html! {
     <div class="recipe-selection">
         <SearchPane
             labels={state.labels.clone()}
             {change_pattern}
             selected_labels={parameters.labels.iter().cloned().collect::<HashSet<_>>()}
         />
         <ul class="recipe-index">
             {items}
             <RecipeCreateButton
                 refresh_recipes={refresh_recipes}
             />
         </ul>
     </div>
    }
}
