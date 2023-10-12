use crate::app::{status_bar::Message, AppContext, Route};
use yew::prelude::*;
use yew_router::prelude::*;

static RECIPE_NAME_PROMPT: &str = "Recipe name:";

#[derive(Clone, Default)]
struct RecipeCreateState {}

#[derive(Properties, PartialEq, Clone)]
pub struct RecipeCreateProps {}

#[function_component(RecipeCreateButton)]
pub fn recipe_create_button(_props: &RecipeCreateProps) -> Html {
    let _state = use_state(RecipeCreateState::default);
    let context = use_context::<AppContext>().unwrap_or(AppContext::default());
    let navigator = use_navigator().unwrap();

    let context_cloned = context.clone();
    let name_submit = Callback::from(move |name: String| {
        let context_cloned = context_cloned.clone();
        let nc = navigator.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match ladle::recipe_create(
                context_cloned.settings.server_url.as_str(),
                name.as_str(),
                "",
                "",
                "",
            )
            .await
            {
                Ok(recipe) => nc.push(&Route::ShowRecipe { id: recipe.id }),
                Err(error) => context_cloned
                    .status
                    .emit(Message::Error(error.to_string(), chrono::Utc::now())),
            }
        });
    });

    let context_cloned = context.clone();
    let name_prompt = Callback::from(move |_| {
        match web_sys::window()
            .unwrap()
            .prompt_with_message(RECIPE_NAME_PROMPT)
        {
            Ok(Some(name)) => name_submit.emit(name),
            Ok(None) => (),
            Err(error) => context_cloned.status.emit(Message::Error(
                error.as_string().unwrap_or(String::default()),
                chrono::Utc::now(),
            )),
        }
    });

    html! {
        <button class="create-item create-recipe" onclick={name_prompt}>
            {"Add recipe"}
        </button>
    }
}
