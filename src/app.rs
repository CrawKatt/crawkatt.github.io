use leptos::prelude::*;
use leptos_fluent::leptos_fluent;
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    path
};
use crate::frontend::pages::{Home, OSSContributions, TermsAndConditions};
use crate::frontend::context::provide_theme_context;

#[component]
fn I18nProvider(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        locales: "./locales",
        default_language: "en",
        sync_html_tag_lang: true,
        sync_html_tag_dir: true,
        cookie_name: "lang",
        cookie_attrs: "SameSite=Strict; Secure; path=/; max-age=600",
        initial_language_from_cookie: true,
        initial_language_from_cookie_to_local_storage: true,
        initial_language_from_cookie_to_server_function: set_language_server_function,
        set_language_to_cookie: true,
        url_param: "lang",
        initial_language_from_url_param: true,
        initial_language_from_url_param_to_local_storage: true,
        initial_language_from_url_param_to_cookie: true,
        set_language_to_url_param: true,
        local_storage_key: "language",
        initial_language_from_local_storage: true,
        initial_language_from_local_storage_to_cookie: true,
        initial_language_from_local_storage_to_server_function: set_language_server_function,
        set_language_to_local_storage: true,
        initial_language_from_navigator: true,
        initial_language_from_navigator_to_cookie: true,
        initial_language_from_navigator_to_server_function: set_language_server_function,
        initial_language_from_accept_language_header: true,
        initial_language_from_server_function: initial_language_server_function,
        initial_language_from_server_function_to_cookie: true,
        initial_language_from_server_function_to_local_storage: true,
        set_language_to_server_function: set_language_server_function,
        url_path: get_language_from_url_path,
        initial_language_from_url_path: true,
        initial_language_from_url_path_to_cookie: true,
        initial_language_from_url_path_to_local_storage: true,
        initial_language_from_url_path_to_server_function: set_language_server_function,
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_theme_context();

    view! {
        <I18nProvider>
            <Stylesheet id="leptos" href="/pkg/portfolio.css" />
            <Link rel="shortcut icon" type_="image/png" href="/favicon.png" />
            <Router>
                <FlatRoutes fallback=|| "Page not found.">
                    <Route path=path!("") view=Home />
                    <Route path=path!("/terms-and-conditions") view=TermsAndConditions/>
                    <Route path=path!("/contributions") view=OSSContributions/>
                </FlatRoutes>
            </Router>
        </I18nProvider>
    }
}

#[server(InitialLanguage, "/api")]
pub async fn initial_language_server_function(
) -> Result<Option<String>, ServerFnError> {
    // .. replace with your own logic
    Ok(Some("es".to_string()))
}

#[server(ShowHelloWorld, "/api")]
pub async fn show_hello_world(
    translated_hello_world: String,
    language: String,
) -> Result<(), ServerFnError> {
    #[allow(clippy::print_stdout)]
    {
        println!("{translated_hello_world} ({language})");
    };
    Ok(())
}

#[server(SetLanguage, "/api")]
pub async fn set_language_server_function(
    _language: String,
) -> Result<(), ServerFnError> {
    // .. replace with your own logic
    Ok(())
}

fn get_language_from_url_path(path: &str) -> &str {
    if let Some(language) = path.split('/').nth(1) {
        return language;
    }
    ""
}