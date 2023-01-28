mod recipe_edit;
mod recipe_list;
mod recipe_window;
mod search_pane;
mod status_bar;

use ladle::models::IngredientIndex;
use recipe_edit::RecipeEditWindow;
use recipe_list::RecipeList;
use recipe_window::RecipeWindow;
use status_bar::{Message, StatusBar};
use std::collections::HashSet;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
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
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(PartialEq, Clone)]
struct AppState {
    update: u32,
    last_error: Message,
    settings_open: bool,
    edition: bool,
}

#[derive(Properties, PartialEq, Clone, Debug)]
struct AppContext {
    server: String,
    recipe_id: Option<String>,
    status: Callback<Message>,

    ingredient_cache: HashSet<IngredientIndex>,
}

impl Default for AppContext {
    fn default() -> Self {
        AppContext {
            server: String::default(),
            recipe_id: None,
            status: Callback::from(|_| ()),
            ingredient_cache: HashSet::new(),
        }
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state_eq(|| AppState {
        update: 0,
        last_error: Message::None,
        settings_open: false,
        edition: false,
    });

    let state_cloned = state.clone();
    let display_status = Callback::from(move |status: Message| {
        let mut data = state_cloned.deref().clone();
        data.last_error = status;
        state_cloned.set(data);
    });

    let stored_server = use_local_storage::<String>("server_url".to_string());

    let context = use_state(|| AppContext {
        server: (*stored_server).clone().unwrap_or_default(),
        recipe_id: None,
        status: display_status,
        ingredient_cache: HashSet::new(),
    });

    let context_cloned = context.clone();
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let mut data = context_cloned.deref().clone();
                let ingredients = ladle::ingredient_index(&context_cloned.server, "")
                    .await
                    .unwrap_or(Vec::<_>::new());
                data.ingredient_cache = ingredients.iter().cloned().collect();
                context_cloned.set(data);
            });
        },
        state.update,
    );

    let state_cloned = state.clone();
    let context_cloned = context.clone();
    let on_server_change = Callback::from(move |e: Event| {
        let new_url = e
            .target()
            .expect("")
            .unchecked_into::<HtmlInputElement>()
            .value();
        let mut data = state_cloned.deref().clone();
        data.update = data.update + 1;
        state_cloned.set(data);

        let context_cloned = context_cloned.clone();
        let new_url_cloned = new_url.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let mut data = context_cloned.deref().clone();
            let ingredients = ladle::ingredient_index(&new_url_cloned, "")
                .await
                .unwrap_or(Vec::<_>::new());
            data.server = new_url_cloned;
            data.ingredient_cache = ingredients.iter().cloned().collect();
            context_cloned.set(data);
        });

        stored_server.set(new_url);
    });

    let state_cloned = state.clone();
    let context_cloned = context.clone();
    let on_recipe_select = Callback::from(move |recipe_id: String| {
        let mut data = state_cloned.deref().clone();
        data.edition = false;
        state_cloned.set(data);

        let mut data = context_cloned.deref().clone();
        data.recipe_id = Some(recipe_id);
        context_cloned.set(data);
    });

    let state_cloned = state.clone();
    let set_settings_mode = Callback::from(move |mode: bool| {
        let mut data = state_cloned.deref().clone();
        data.settings_open = mode;
        state_cloned.set(data);
    });

    let state_cloned = state.clone();
    let context_cloned = context.clone();
    let on_delete: Callback<()> = Callback::from(move |_| {
        let mut data = state_cloned.deref().clone();
        data.edition = false;
        data.update = data.update + 1;
        state_cloned.set(data);

        let mut data = context_cloned.deref().clone();
        data.recipe_id = None;
        context_cloned.set(data);
    });

    let switch = move |route: Route| -> Html {
        match route {
            Route::ListRecipes => html! {
                <>
                    <RecipeList/>
                    <RecipeWindow recipe_id={Option::<String>::None}/>
                </>
            },
            Route::ShowRecipe { id } => html! {
                <>
                    <RecipeList />
                    <RecipeWindow recipe_id={Some(id)}/>
                </>
            },
            Route::ListIngredients => html! {},
            Route::EditRecipe { id } => html! {
                <>
                    <RecipeList />
                    <RecipeEditWindow recipe_id={id}/>
                </>
            },
            Route::NotFound => html! {"404"},
        }
    };

    let open_settings = set_settings_mode.clone();

    html! {
        <main>
            <StatusBar current={state.last_error.clone()} />
            <div class={format!("settings {}", if state.settings_open {"open"} else {"close"})}>
                <label for="server">{"Knife server url:"}</label>
                <input type="text"
                    name="server"
                    onchange={on_server_change}
                    value={context.server.clone()}
                />
                <button
                    class="settings-close"
                    onclick={move |_| open_settings.emit(false)}
                >
                    {"Close"}
                </button>
            </div>
            <div class="header">
                <div class="left">
                    <button onclick={move |_| set_settings_mode.clone().emit(true)}>
                        {"Settings"}
                    </button>
                </div>
                <div class="logo">
                    {format!("spoon v{}", env!("CARGO_PKG_VERSION"))}
                </div>
                <div class="right">
                </div>
            </div>
            <ContextProvider<AppContext> context={(*context).clone()}>
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </ContextProvider<AppContext>>
        </main>
    }
}
