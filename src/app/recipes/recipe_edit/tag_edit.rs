use crate::app::recipes::recipe_edit::EditionContext;
use crate::app::status_bar::Message;
use crate::app::AppContext;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TagEditItemProps {
    pub label: ladle::models::LabelIndex,
    pub refresh: Callback<()>,
}

#[function_component(TagEditItem)]
pub fn tag_edit_item(props: &TagEditItemProps) -> Html {
    let context = use_context::<AppContext>().unwrap_or(AppContext::default());
    let edition_context = use_context::<EditionContext>().unwrap_or(EditionContext::default());
    let props_cloned = props.clone();
    let context_cloned = context.clone();
    let recipe_id = edition_context.recipe_id;
    let on_tag_delete = Callback::from(move |_| {
        let props_cloned = props_cloned.clone();
        let context_cloned = context_cloned.clone();
        let recipe_id = recipe_id.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match ladle::recipe_untag(
                context_cloned.settings.server_url.as_str(),
                recipe_id.as_str(),
                props_cloned.label.id.as_str(),
            )
            .await
            {
                Ok(_) => props_cloned.refresh.emit(()),
                Err(message) => context_cloned
                    .status
                    .emit(Message::Error(message.to_string(), chrono::Utc::now())),
            }
        });
    });

    html! {
        <li key={props.label.id.as_str()}>
            <span>{props.label.name.as_str()}</span>
            <button onclick={on_tag_delete}>{"Delete"}</button>
        </li>
    }
}
