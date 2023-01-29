use crate::app::status_bar::Message;
use crate::app::AppContext;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct DependencyEditItemProps {
    pub dependency: ladle::models::Dependency,
    pub refresh: Callback<()>,
}

#[function_component(DependencyEditItem)]
pub fn dependency_edit_item(props: &DependencyEditItemProps) -> Html {
    let context = use_context::<AppContext>().unwrap_or(AppContext::default());

    let props_cloned = props.clone();
    let context_cloned = context.clone();
    let on_dependency_delete = Callback::from(move |_| {
        let props_cloned = props_cloned.clone();
        let context_cloned = context_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match ladle::dependency_delete(
                context_cloned.server.as_str(),
                context_cloned.recipe_id.unwrap().as_str(),
                props_cloned.dependency.recipe.id.as_str(),
            )
            .await
            {
                Ok(()) => props_cloned.refresh.emit(()),
                Err(message) => context_cloned
                    .status
                    .emit(Message::Error(message.to_string(), chrono::Utc::now())),
            }
        });
    });

    html! {
        <li key={props.dependency.recipe.id.as_str()}>
            <span>{props.dependency.recipe.name.as_str()}</span>
            <button onclick={on_dependency_delete}>{"Delete"}</button>
        </li>
    }
}
