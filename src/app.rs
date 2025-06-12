use fluent_templates::static_loader;
use leptos::prelude::*;
use leptos_fluent::leptos_fluent;
use leptos_meta::*;
use leptos_router::{components::{Route, Router, Routes}, path};
use crate::context::provide_theme_context;
use crate::pages::*;

static_loader! {
    pub static ENGLISH = {
        locales: "./locales",
        fallback_language: "en",
    };
}

#[component]
pub fn I18n(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        translations: [ENGLISH],
        locales: "./locales",
        initial_language_from_navigator: true,
        initial_language_from_navigator_to_localstorage: true,
        sync_html_tag_lang: true,
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_theme_context();

    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <I18n>
            <Router>
                <Routes fallback=|| "Page not found.">
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/terms-and-conditions") view=TermsAndConditions/>
                    <Route path=path!("/contributions") view=OSSContributions/>
                </Routes>
            </Router>
        </I18n>
    }
}