mod ingredients;
mod recipes;
mod settings;
mod status_bar;

use ingredients::{
    create::IngredientCreateButton,
    edit::{IngredientEdit, IngredientEditButton},
    list::IngredientList,
    show::IngredientView,
};
use ladle::models::IngredientIndex;
use recipes::list::RecipeList;
use recipes::recipe_edit::RecipeEditWindow;
use recipes::recipe_window::RecipeWindow;
use settings::{AppSettings, Settings};
use status_bar::{Message, StatusBar};
use std::collections::HashSet;
use std::ops::Deref;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/recipes")]
    ListRecipes,
    #[at("/recipes/:id")]
    ShowRecipe { id: String },
    #[at("/recipes/:id/edit")]
    EditRecipe { id: String },
    #[at("/ingredients")]
    ListIngredients,
    #[at("/ingredients/:id")]
    ShowIngredient { id: String },
    #[at("/ingredients/:id/edit")]
    EditIngredient { id: String },
    #[at("/settings")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(PartialEq, Clone)]
struct AppState {
    last_error: Message,
    edition: bool,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            last_error: Message::None,
            edition: false,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
struct AppContext {
    settings: AppSettings,
    status: Callback<Message>,
    ingredient_cache: HashSet<IngredientIndex>,
}

impl Default for AppContext {
    fn default() -> Self {
        AppContext {
            settings: AppSettings::default(),
            status: Callback::from(|_| ()),
            ingredient_cache: HashSet::new(),
        }
    }
}

pub fn set_title(title: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            document.set_title(title)
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state_eq(|| AppState::default());

    // Callback to display a banner on the main window for the user
    let state_cloned = state.clone();
    let display_status = Callback::from(move |status: Message| {
        let mut data = state_cloned.deref().clone();
        data.last_error = status;
        state_cloned.set(data);
    });

    // Application settings stored on client disk
    let persistent_settings = use_local_storage::<AppSettings>("persistent_settings".to_string());
    let ingredient_cache = use_local_storage::<HashSet<ladle::models::IngredientIndex>>(
        "ingredient_cache".to_string(),
    );

    // Data accessible by all children
    let context = use_state(|| AppContext {
        settings: (*persistent_settings).clone().unwrap_or_default(),
        status: display_status,
        ingredient_cache: (*ingredient_cache).clone().unwrap_or_default(),
    });

    // Callback to trigger a refresh of the ingredient cache
    let context_cloned = context.clone();
    let update_ingredient_cache = Callback::from(move |_| {
        let context_cloned = context_cloned.clone();
        let ingredient_cache = ingredient_cache.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let mut data = context_cloned.deref().clone();
            match ladle::ingredient_index(&context_cloned.settings.server_url, "").await {
                Ok(ingredients) => {
                    let set: HashSet<_> = ingredients.iter().cloned().collect();
                    data.ingredient_cache = set.clone();
                    // Make it available to child components
                    context_cloned.set(data);
                    // Store it on disk
                    ingredient_cache.set(set);
                }
                Err(error) => context_cloned
                    .status
                    .emit(Message::Error(error.to_string(), chrono::Utc::now())),
            }
        });
    });

    // On change of the 'update' field of the state, fetch and cache ingredients
    let effect = update_ingredient_cache.clone();
    use_effect_with_deps(
        move |_| effect.emit(()),
        context.settings.server_url.clone(),
    );

    // Callback to update settings to the value passed as an argument
    let context_cloned = context.clone();
    let update_settings = Callback::from(move |settings: AppSettings| {
        let mut data = context_cloned.deref().clone();
        data.settings = settings.clone();
        context_cloned.set(data);

        persistent_settings.set(settings);
    });

    let context_cloned = context.clone();
    html! {
        <main>
            <StatusBar current={state.last_error.clone()} />
            <ContextProvider<AppContext> context={(*context).clone()}>
                <BrowserRouter>
                    <div class="header">
                        <div class="left">
                            <Link<Route> to={Route::Settings}>
                                {"Settings"}
                            </Link<Route>>
                            <Link<Route> to={Route::ListRecipes}>
                                {"Recipes"}
                            </Link<Route>>
                            <Link<Route> to={Route::ListIngredients}>
                                {"Ingredients"}
                            </Link<Route>>
                        </div>
                        <div class="logo">
                            {format!("spoon v{}", env!("CARGO_PKG_VERSION"))}
                        </div>
                        <div class="right">
                        </div>
                    </div>
                    <Switch<Route>
                        render={Callback::from(move |switch: Route| {
                            let context_cloned = context_cloned.clone();
                            let update_settings = update_settings.clone();
                            let update_ingredient_cache = update_ingredient_cache.clone();
                            match switch {
                                Route::ListRecipes => html! {
                                    <RecipeList/>
                                },
                                Route::ShowRecipe { id } => html! {
                                    <RecipeWindow recipe_id={Some(id)}/>
                                },
                                Route::EditRecipe { id } => html! {
                                    <RecipeEditWindow recipe_id={id}/>
                                },
                                Route::ListIngredients => html! {
                                    <div class={"ingredient-main"}>
                                        <IngredientList />
                                        <IngredientView
                                            ingredient_id={Option::<String>::None}
                                        />
                                        <div class={"options"}>
                                            <IngredientCreateButton
                                                ingredient_cache_refresh={update_ingredient_cache}
                                            />
                                        </div>
                                    </div>
                                },
                                Route::ShowIngredient { id } => html! {
                                    <div class={"ingredient-main"}>
                                        <IngredientList />
                                        <IngredientView ingredient_id={Some(id.clone())}/>
                                        <div class={"options"}>
                                            <IngredientCreateButton
                                                ingredient_cache_refresh={update_ingredient_cache}
                                            />
                                            <IngredientEditButton
                                                ingredient_id={id}
                                            />
                                        </div>
                                    </div>
                                },
                                Route::EditIngredient { id } => html! {
                                    <div class={"ingredient-main"}>
                                        <IngredientList />
                                        <IngredientEdit
                                            ingredient_id={Some(id)}
                                            ingredient_cache_refresh={update_ingredient_cache}
                                        />
                                    </div>
                                },
                                Route::Settings => html! {
                                    <Settings
                                        current={context_cloned.settings.clone()}
                                        update_settings={update_settings}
                                        />
                                },
                                Route::NotFound => html! {"404"},
                            }
                        })}
                    />
                </BrowserRouter>
            </ContextProvider<AppContext>>
        </main>
    }
}
