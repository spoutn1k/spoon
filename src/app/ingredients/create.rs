use crate::app::status_bar::Message;
use crate::app::AppContext;
use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

static INGREDIENT_NAME_PROMPT: &str = "Ingredient name:";

#[derive(Clone, Default)]
struct IngredientCreateState {}

#[derive(Properties, PartialEq, Clone)]
pub struct IngredientCreateProps {}

#[function_component(IngredientCreateButton)]
pub fn ingredient_create_button(_props: &IngredientCreateProps) -> Html {
    let _state = use_state(IngredientCreateState::default);
    let context = use_context::<AppContext>().unwrap_or(AppContext::default());
    let navigator = use_navigator().unwrap();

    let context_cloned = context.clone();
    let name_submit = Callback::from(move |name: String| {
        let context_cloned = context_cloned.clone();
        let nc = navigator.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match ladle::ingredient_create(
                context_cloned.settings.server_url.as_str(),
                name.as_str(),
                false,
                false,
                false,
                false,
            )
            .await
            {
                Ok(ingredient) => nc.push(&Route::ShowIngredient { id: ingredient.id }),
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
            .prompt_with_message(INGREDIENT_NAME_PROMPT)
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
        <button class="create-item create-ingredient" onclick={name_prompt}>
            {"Add ingredient"}
        </button>
    }
}
