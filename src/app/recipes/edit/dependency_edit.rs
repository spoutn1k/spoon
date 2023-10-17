use crate::app::recipes::edit::EditionContext;
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
    let edition_context = use_context::<EditionContext>().unwrap_or(EditionContext::default());

    let props_cloned = props.clone();
    let context_cloned = context.clone();
    let recipe_id = edition_context.recipe_id;
    let on_dependency_delete = Callback::from(move |_| {
        let props_cloned = props_cloned.clone();
        let context_cloned = context_cloned.clone();
        let recipe_id = recipe_id.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match ladle::dependency_delete(
                context_cloned.settings.server_url.as_str(),
                recipe_id.as_str(),
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
        <tr key={props.dependency.recipe.id.as_str()}>
            <td>{props.dependency.recipe.name.as_str()}</td>
            <td><button onclick={on_dependency_delete}>{"Delete"}</button></td>
        </tr>
    }
}
