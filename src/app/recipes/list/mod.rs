mod create_button;
mod element;
mod filters;
mod search_pane;

use crate::app::recipes::list::create_button::RecipeCreateButton;
use crate::app::recipes::list::element::RecipeElement;
use crate::app::recipes::list::filters::Filters;
use crate::app::recipes::list::search_pane::SearchPane;
use crate::app::set_title;
use crate::app::status_bar::Message;
use crate::app::AppContext;
use futures::future::join_all;
use std::collections::HashSet;
use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;

fn simplify_name(original: &str) -> String {
    unidecode::unidecode(original).to_lowercase()
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

pub enum RecipeListAction {
    UpdateRecipes(Vec<ladle::models::RecipeIndex>),
    UpdateLabels(HashSet<ladle::models::LabelIndex>),
    UpdatePattern(String),
}

#[derive(Properties, PartialEq, Clone, Default, Debug)]
pub struct RecipeListState {
    recipes: Vec<ladle::models::RecipeIndex>,
    labels: HashSet<ladle::models::LabelIndex>,
    pattern: String,
}

impl Reducible for RecipeListState {
    type Action = RecipeListAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            RecipeListAction::UpdateRecipes(list) => Self {
                recipes: list,
                labels: self.labels.clone(),
                pattern: self.pattern.clone(),
            },
            RecipeListAction::UpdateLabels(set) => Self {
                recipes: self.recipes.clone(),
                labels: set,
                pattern: self.pattern.clone(),
            },
            RecipeListAction::UpdatePattern(string) => Self {
                recipes: self.recipes.clone(),
                labels: self.labels.clone(),
                pattern: string,
            },
        }
        .into()
    }
}

#[function_component(RecipeList)]
pub fn recipe_list(_: &RecipeListProps) -> Html {
    let state = use_reducer(RecipeListState::default);
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
    let change_pattern = Callback::from(move |pattern: String| {
        cloned_state.dispatch(RecipeListAction::UpdatePattern(pattern))
    });

    set_title("Recipes - spoon");

    let cloned_state = state.clone();
    let context_cloned = context.clone();
    let labels = selected_labels.clone();
    let refresh_recipes = Callback::from(move |_| {
        let cloned_state = cloned_state.clone();
        let context_cloned = context_cloned.clone();
        let labels = labels.clone();
        wasm_bindgen_futures::spawn_local(async move {
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

            cloned_state.dispatch(RecipeListAction::UpdateRecipes(fetched_recipes))
        });
    });

    let cloned_state = state.clone();
    let context_cloned = context.clone();
    let refresh_labels = Callback::from(move |_| {
        let cloned_state = cloned_state.clone();
        let context_cloned = context_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_labels =
                ladle::label_index(context_cloned.settings.server_url.as_str(), "").await;

            let labels = match fetched_labels {
                Ok(mut index) => {
                    index.sort_by(|lhs, rhs| lhs.name.cmp(&rhs.name));
                    HashSet::from_iter(index.iter().cloned())
                }
                Err(message) => {
                    context_cloned
                        .status
                        .emit(Message::Error(message.to_string(), chrono::Utc::now()));
                    HashSet::new()
                }
            };

            cloned_state.dispatch(RecipeListAction::UpdateLabels(labels))
        });
    });

    use_effect_with_deps(move |_| refresh_recipes.emit(()), selected_labels);

    use_effect_with_deps(
        move |_| refresh_labels.emit(()),
        context.settings.server_url.clone(),
    );

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
             </ul>
            <RecipeCreateButton />
         </div>
    }
}
