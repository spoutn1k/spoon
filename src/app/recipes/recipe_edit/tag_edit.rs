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
    let props_cloned = props.clone();
    let context_cloned = context.clone();
    let on_tag_delete = Callback::from(move |_| {
        let props_cloned = props_cloned.clone();
        let context_cloned = context_cloned.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match ladle::recipe_untag(
                context_cloned.server.as_str(),
                context_cloned.recipe_id.unwrap().as_str(),
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
