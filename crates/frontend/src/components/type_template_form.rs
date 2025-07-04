pub mod utils;

use crate::components::form::label::Label;
use crate::components::type_template_form::utils::create_type;
use crate::components::{
    alert::AlertType,
    button::Button,
    change_form::ChangeForm,
    input::{Input, InputType},
    type_template_form::utils::update_type,
};
use crate::providers::{alert_provider::enqueue_alert, editor_provider::EditorProvider};
use crate::schema::{JsonSchemaType, SchemaType};
use crate::types::{OrganisationId, Tenant};
use leptos::*;
use serde_json::{json, Value};
use web_sys::MouseEvent;

#[component]
pub fn type_template_form<NF>(
    #[prop(default = false)] edit: bool,
    #[prop(default = String::new())] type_name: String,
    #[prop(default = json!({"type": "number"}))] type_schema: Value,
    handle_submit: NF,
    #[prop(default = String::new())] description: String,
) -> impl IntoView
where
    NF: Fn() + 'static + Clone,
{
    let workspace = use_context::<Signal<Tenant>>().unwrap();
    let org = use_context::<Signal<OrganisationId>>().unwrap();

    let (error_message, set_error_message) = create_signal("".to_string());
    let (type_name_rs, type_name_ws) = create_signal(type_name);
    let (type_schema_rs, type_schema_ws) = create_signal(type_schema);
    let (req_inprogess_rs, req_inprogress_ws) = create_signal(false);

    let (description_rs, description_ws) = create_signal(description);
    let (change_reason_rs, change_reason_ws) = create_signal(String::new());

    let on_submit = move |ev: MouseEvent| {
        req_inprogress_ws.set(true);
        ev.prevent_default();
        let type_name = type_name_rs.get();
        let type_schema = type_schema_rs.get();

        let handle_submit_clone = handle_submit.clone();
        spawn_local({
            let handle_submit = handle_submit_clone;
            async move {
                let result = if edit {
                    let payload = json!({
                        "type_schema": type_schema,
                        "description": description_rs.get(),
                        "change_reason": change_reason_rs.get(),
                    });
                    update_type(workspace.get().0, type_name, payload, org.get().0).await
                } else {
                    let description = description_rs.get();
                    let change_reason = change_reason_rs.get();
                    let payload = json!({
                        "type_name": type_name,
                        "type_schema": type_schema,
                        "description": description,
                        "change_reason": change_reason
                    });
                    create_type(workspace.get().0, payload.clone(), org.get().0).await
                };

                req_inprogress_ws.set(false);
                match result {
                    Ok(_) => {
                        handle_submit();
                        let success_message = if edit {
                            "Type updated successfully!"
                        } else {
                            "New Type created successfully!"
                        };
                        enqueue_alert(
                            String::from(success_message),
                            AlertType::Success,
                            5000,
                        );
                    }
                    Err(e) => {
                        set_error_message.set(e.clone());
                        enqueue_alert(e, AlertType::Error, 5000);
                    }
                }
            }
        });
    };
    view! {
        <form class="w-full flex flex-col gap-5 bg-white text-gray-700">
            <div class="form-control">
                <Label title="Type Name" />
                <input
                    disabled=edit
                    type="text"
                    placeholder="Type name"
                    name="type_name"
                    id="type_name"
                    class="input input-bordered w-full max-w-md"
                    value=move || type_name_rs.get()
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        type_name_ws.set(value);
                    }
                />
            </div>

            <ChangeForm
                title="Description".to_string()
                placeholder="Enter a description".to_string()
                value=description_rs.get_untracked()
                on_change=Callback::new(move |new_description| {
                    description_ws.set(new_description)
                })
            />
            <ChangeForm
                title="Reason for Change".to_string()
                placeholder="Enter a reason for this change".to_string()
                value=change_reason_rs.get_untracked()
                on_change=Callback::new(move |new_change_reason| {
                    change_reason_ws.set(new_change_reason)
                })
            />

            <div class="form-control">
                <Label title="Type Schema" />
                {move || {
                    let schem = type_schema_rs.get();
                    let schema_type = SchemaType::Single(JsonSchemaType::from(&schem));
                    view! {
                        <EditorProvider>
                            <Input
                                id="type-schema"
                                class="w-full max-w-md pt-3 rounded-md resize-y"
                                schema_type
                                value=schem
                                on_change=move |new_type_schema| type_schema_ws.set(new_type_schema)
                                r#type=InputType::Monaco(vec![])
                            />
                        </EditorProvider>
                    }
                }}

            </div>

            {move || {
                let loading = req_inprogess_rs.get();
                view! {
                    <Button
                        class="self-end h-12 w-48"
                        text="Submit"
                        icon_class="ri-send-plane-line"
                        on_click=on_submit.clone()
                        loading
                    />
                }
            }}
            <p class="text-red-500">{move || error_message.get()}</p>

        </form>
    }
}
