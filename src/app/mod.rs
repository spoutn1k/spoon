mod recipes;
use recipes::RecipesList;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <RecipesList url={String::from("http://localhost:8000")} />
        </main>
    }
}
