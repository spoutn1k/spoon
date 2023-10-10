mod ingredients;
mod recipes;
mod settings;
mod status_bar;

use ingredients::list::IngredientList;
use ingredients::show::IngredientView;
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
    #[at("/settings")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(PartialEq, Clone)]
struct AppState {
    update: u32,
    last_error: Message,
    edition: bool,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            update: 0,
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

    // On change of the 'update' field of the state, fetch and cache ingredients
    let context_cloned = context.clone();
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let mut data = context_cloned.deref().clone();
                match ladle::ingredient_index(&context_cloned.settings.server_url, "").await {
                    Ok(ingredients) => {
                        let set: HashSet<_> = ingredients.iter().cloned().collect();
                        data.ingredient_cache = set.clone();
                        context_cloned.set(data);
                        ingredient_cache.set(set);
                    }
                    Err(_) => (),
                }
            });
        },
        state.update,
    );

    // Callback to update settings to the value passed as an argument
    let state_cloned = state.clone();
    let context_cloned = context.clone();
    let update_settings = Callback::from(move |settings: AppSettings| {
        let mut data = context_cloned.deref().clone();
        data.settings = settings.clone();
        context_cloned.set(data);

        persistent_settings.set(settings);

        let mut data = state_cloned.deref().clone();
        data.update = data.update + 1;
        state_cloned.set(data);
    });

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
                            let update_settings = update_settings.clone();
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
                                    <IngredientList />
                                },
                                Route::ShowIngredient { id } => html! {
                                    <>
                                        <IngredientList />
                                        <IngredientView ingredient_id={Some(id)}/>
                                    </>
                                },
                                Route::Settings => html! {
                                    <Settings update_settings={update_settings}/>
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
