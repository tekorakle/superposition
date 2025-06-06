use leptos::*;

use chrono::DateTime;
use leptos_router::A;
use serde_json::{json, Map, Value};
use superposition_macros::box_params;
use superposition_types::{
    custom_query::{CustomQuery, PaginationParams, Query},
    database::models::cac::ConfigVersion,
    PaginatedResponse,
};

use crate::components::skeleton::Skeleton;
use crate::components::stat::Stat;
use crate::components::table::types::{ColumnSortable, Expandable, TablePaginationProps};
use crate::components::table::{types::Column, Table};
use crate::query_updater::{use_param_updater, use_signal_from_query};
use crate::types::{OrganisationId, Tenant};
use crate::utils::use_url_base;
use crate::{api::fetch_snapshots, components::table::types::default_column_formatter};

#[component]
pub fn config_version_list() -> impl IntoView {
    let tenant_rws = use_context::<RwSignal<Tenant>>().unwrap();
    let org_rws = use_context::<RwSignal<OrganisationId>>().unwrap();

    let pagination_params_rws = use_signal_from_query(move |query_string| {
        Query::<PaginationParams>::extract_non_empty(&query_string).into_inner()
    });

    use_param_updater(move || box_params!(pagination_params_rws.get()));

    let table_columns =
        create_memo(move |_| snapshot_table_columns(tenant_rws.get().0, org_rws.get().0));

    let snapshots_resource: Resource<
        (String, PaginationParams, String),
        PaginatedResponse<ConfigVersion>,
    > = create_blocking_resource(
        move || {
            (
                tenant_rws.get().0,
                pagination_params_rws.get(),
                org_rws.get().0,
            )
        },
        |(current_tenant, pagination_params, org_id)| async move {
            fetch_snapshots(&pagination_params, current_tenant.to_string(), org_id)
                .await
                .unwrap_or_default()
        },
    );

    let handle_next_click = Callback::new(move |next_page: i64| {
        pagination_params_rws.update(|f| f.page = Some(next_page));
    });

    let handle_prev_click = Callback::new(move |prev_page: i64| {
        pagination_params_rws.update(|f| f.page = Some(prev_page));
    });

    view! {
        <div class="p-8">
            <Suspense fallback=move || view! { <Skeleton /> }>
                <div class="pb-4">
                    {move || {
                        let snapshot_res = snapshots_resource.get();
                        let total_items = match snapshot_res {
                            Some(snapshot_resp) => snapshot_resp.total_items.to_string(),
                            _ => "0".to_string(),
                        };
                        view! {
                            <Stat
                                heading="Config Versions"
                                icon="ri-camera-lens-fill"
                                number=total_items
                            />
                        }
                    }}

                </div>
                <div class="card rounded-xl w-full bg-base-100 shadow">
                    <div class="card-body">
                        <div class="flex justify-between">
                            <h2 class="card-title">Config Versions</h2>
                        </div>
                        <div>
                            {move || {
                                let value = snapshots_resource.get();
                                let pagination_params = pagination_params_rws.get();
                                match value {
                                    Some(snapshot_resp) => {
                                        let page = pagination_params.page.unwrap_or(1);
                                        let count = pagination_params.count.unwrap_or(10);
                                        let total_pages = snapshot_resp.clone().total_pages;
                                        let resp = snapshot_resp
                                            .data
                                            .into_iter()
                                            .map(|config_version| {
                                                let mut map = Map::new();
                                                map.insert(
                                                    "id".to_string(),
                                                    Value::String(config_version.id.to_string()),
                                                );
                                                map.insert(
                                                    "created_at".to_string(),
                                                    json!(config_version.created_at.format("%v %T").to_string()),
                                                );
                                                map.insert("tags".to_string(), json!(config_version.tags));
                                                map
                                            })
                                            .collect();
                                        let pagination_props = TablePaginationProps {
                                            enabled: true,
                                            count,
                                            current_page: page,
                                            total_pages,
                                            on_next: handle_next_click,
                                            on_prev: handle_prev_click,
                                        };
                                        view! {
                                            <Table
                                                rows=resp
                                                key_column="id".to_string()
                                                columns=table_columns.get()
                                                pagination=pagination_props
                                            />
                                        }
                                    }
                                    None => {
                                        view! { <Skeleton /> }
                                    }
                                }
                            }}

                        </div>
                    </div>
                </div>
            </Suspense>
        </div>
    }
}

pub fn snapshot_table_columns(tenant: String, org_id: String) -> Vec<Column> {
    vec![
        Column::new(
            "id".to_string(),
            false,
            move |value: &str, _row: &Map<String, Value>| {
                let id = value.to_string();
                let base = use_url_base();
                let href = format!(
                    "{}/admin/{}/{}/config/versions/{}",
                    base,
                    org_id.clone(),
                    tenant.clone(),
                    id
                );
                view! {
                    <div class="w-24">
                        <A href=href class="btn-link">{id}</A>
                    </div>
                }
                .into_view()
            },
            ColumnSortable::No,
            Expandable::Disabled,
            default_column_formatter,
        ),
        Column::new(
            "created_at".to_string(),
            false,
            |value: &str, _row: &Map<String, Value>| {
                let formatted_date =
                    match DateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S%.f") {
                        Ok(dt) => dt.format("%d-%b-%Y").to_string(),
                        Err(_) => {
                            logging::log!("Failed to parse date: {}", value);
                            value.to_string()
                        }
                    };
                view! { <span class="w-24">{formatted_date}</span> }.into_view()
            },
            ColumnSortable::No,
            Expandable::Enabled(100),
            default_column_formatter,
        ),
        Column::new(
            "tags".to_string(),
            false,
            |_value: &str, row: &Map<String, Value>| {
                let tags = row.get("tags").and_then(|v| v.as_array());
                match tags {
                    Some(arr) => {
                        let tags_str = arr
                            .iter()
                            .map(|v| v.as_str().unwrap_or(""))
                            .collect::<Vec<&str>>()
                            .join(", ");
                        view! { <span class="w-24">{tags_str}</span> }
                    }
                    None => view! { <span class="w-24">"-"</span> },
                }
                .into_view()
            },
            ColumnSortable::No,
            Expandable::Enabled(100),
            default_column_formatter,
        ),
    ]
}
