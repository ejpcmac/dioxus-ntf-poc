//! An application that can receive and show notifications.

#![expect(clippy::same_name_method, reason = "generated inside Dioxus macros")]

use dioxus::prelude::*;
use ntf_api::ApiClient;

/// Version info.
const VERSION_WITH_GIT: &str = env!("VERSION_WITH_GIT");
/// CSS for the app.
const CSS: Asset = asset!("/assets/app.css");
/// API endpoint.
const ENDPOINT: &str = "http://localhost:3000";

/// Pages of the application.
#[derive(Debug, Clone, Routable)]
enum Route {
    /// The list of notifications.
    #[route("/")]
    List,
    /// The view for a given notification.
    #[route("/:id")]
    Show {
        /// ID of the notification to show.
        id: usize,
    },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: CSS }

        Version {}
        Router::<Route> {}
    }
}

#[component]
fn Version() -> Element {
    rsx! {
        div { class: "absolute z-1000 top-2 right-2 badge badge-sm badge-ghost", "{VERSION_WITH_GIT}" }
    }
}

#[component]
fn List() -> Element {
    let fetch_notifications = async move || {
        ApiClient::new(ENDPOINT)
            .list_notifications()
            .await
            .unwrap_or_default()
    };

    let mut notifications = use_resource(fetch_notifications);

    let mut update_notifications = async move || {
        notifications.set(Some(fetch_notifications().await));
    };

    let reload_notifications = move |_| async move {
        update_notifications().await;
    };

    let ack_notification = move |id| async move {
        let _ignored = ApiClient::new(ENDPOINT).ack_notification(id).await;
        update_notifications().await;
    };

    let delete_notification = move |id| async move {
        let _ignored = ApiClient::new(ENDPOINT).delete_notification(id).await;
        update_notifications().await;
    };

    rsx! {
        div { class: "navbar bg-base-100 shadow-sm" }

        ul { class: "list bg-base-100 rounded-box shadow-md",
            if let Some(ntfs) = notifications.read().as_deref() {
                for ntf in ntfs {
                    li { class: "list-row",
                        Link {
                            to: Route::Show { id: ntf.id },
                            class: "list-col-grow flex gap-4",
                            h2 { class: "text-4xl", "#{ntf.id}" }
                            p { "{ntf.message}" }
                        }

                        if ntf.ack {
                            button { class: "btn btn-active btn-success", "✓" }
                        } else {
                            button {
                                class: "btn btn-soft btn-success",
                                onclick: {
                                    let id = ntf.id;
                                    move |_| ack_notification(id)
                                },
                                "✓"
                            }
                        }
                        button {
                            class: "btn btn-soft btn-error",
                            onclick: {
                                let id = ntf.id;
                                move |_| delete_notification(id)
                            },
                            "✗"
                        }
                    }
                }

                li { class: "p-4 pb-2 text-xs tracking-wide",
                    button {
                        class: "btn btn-primary",
                        onclick: reload_notifications,
                        "Reload notifications"
                    }
                }
            }
        }
    }
}

#[component]
fn Show(
    /// ID of the notification to show.
    id: usize,
) -> Element {
    let fetch_notification = move || async move {
        ApiClient::new(ENDPOINT).get_notification(id).await
    };

    let mut notification = use_resource(fetch_notification);

    let ack_notification = move |id| async move {
        let _ignored = ApiClient::new(ENDPOINT).ack_notification(id).await;
        notification.set(Some(fetch_notification().await));
    };

    let delete_notification = move |id| async move {
        let _ignored = ApiClient::new(ENDPOINT).delete_notification(id).await;
        navigator().replace(Route::List);
    };

    rsx! {
        div { class: "navbar bg-base-100 shadow-sm",
            Link { to: Route::List, class: "btn btn-primary", "<" }
        }

        if let Some(result) = notification.read().as_ref() {
            div { class: "hero bg-base-100",
                div { class: "hero-content text-center",
                    div { class: "max-w-md",
                        {
                            match result {
                                Ok(ntf) => rsx! {
                                    h1 { class: "text-5xl font-bold", "#{ntf.id}" }
                                    p { class: "py-6", "{ntf.message}" }
                                    div { class: "flex flex-col gap-2 min-w-3xs",
                                        if ntf.ack {
                                            button { class: "btn btn-active btn-success", "✓" }
                                        } else {
                                            button {
                                                class: "btn btn-soft btn-success",
                                                onclick: {
                                                    let id = ntf.id;
                                                    move |_| ack_notification(id)
                                                },
                                                "✓"
                                            }
                                        }
                                        button {
                                            class: "btn btn-soft btn-error",
                                            onclick: {
                                                let id = ntf.id;
                                                move |_| delete_notification(id)
                                            },
                                            "✗"
                                        }
                                    }
                                },
                                Err(error) => rsx! {
                                    div { role: "alert", class: "alert alert-error alert-soft", "Error: {error.to_string()}" }
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}
