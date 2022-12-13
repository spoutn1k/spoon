mod recipe_list;
mod recipe_window;
use recipe_list::RecipeList;
use recipe_window::RecipeWindow;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let server_handle = use_state_eq(|| String::from("http://localhost:8000"));
    let server = (*server_handle).clone();

    let selected_recipe_id_handle = use_state_eq(|| Option::<String>::None);
    let selected_recipe_id = (*selected_recipe_id_handle).clone();

    let on_server_change = Callback::from(move |e: Event| {
        let target: EventTarget = e.target().expect("");
        let url = target.unchecked_into::<HtmlInputElement>().value();
        log::info!("New server: {}", &url);
        server_handle.set(url);
    });

    let on_recipe_select = {
        let selected_recipe = selected_recipe_id_handle.clone();
        Callback::from(move |recipe_id: String| selected_recipe.set(Some(recipe_id)))
    };

    html! {
        <main>
            <input type="text"
                onchange={on_server_change}
                value={server.clone()}
            />
            <RecipeList url={server.clone()} on_click={on_recipe_select} />
            <RecipeWindow url={server} recipe_id={selected_recipe_id}/>
        </main>
    }
}
