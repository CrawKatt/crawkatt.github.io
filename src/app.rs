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
        set_language_to_cookie: true,
        initial_language_from_url_param_to_local_storage: true,
        initial_language_from_url_param_to_cookie: true,
        local_storage_key: "language",
        initial_language_from_local_storage: true,
        initial_language_from_local_storage_to_cookie: true,
        set_language_to_local_storage: true,
        initial_language_from_navigator: true,
        initial_language_from_navigator_to_cookie: true,
        initial_language_from_accept_language_header: true,
        initial_language_from_server_function: initial_language_server_function,
        initial_language_from_server_function_to_cookie: true,
        initial_language_from_server_function_to_local_storage: true,
        initial_language_from_url_path_to_cookie: true,
        initial_language_from_url_path_to_local_storage: true,
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_theme_context();

    view! {
        <I18nProvider>
            <Meta property="og:type" content="website"/>
            <Meta property="og:title" content="CrawKatt - Portfolio"/>
            <Meta property="og:description" content="CrawKatt Software Developer"/>
            <Meta property="og:image" content="https://crawkatt.dev/profile.png"/>
            <Meta property="og:url" content="https://crawkatt.dev/"/>

            <Meta name="twitter:card" content="summary_large_image"/>
            <Meta name="twitter:title" content="CrawKatt - Portfolio"/>
            <Meta name="twitter:description" content="CrawKatt Software Developer"/>
            <Meta name="twitter:image" content="https://crawkatt.dev/profile.png"/>
            <Stylesheet id="leptos" href="/pkg/portfolio.css" />
            <Link rel="shortcut icon" type_="image/png" href="/favicon.ico" />
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
    Ok(Some("es".to_string()))
}