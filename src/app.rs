use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::{Route, Router, Routes}, path};
use crate::pages::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=path!("/") view=Home/>
                <Route path=path!("/terms-and-conditions") view=TermsAndConditions/>
                <Route path=path!("/contributions") view=OSSContributions/>
            </Routes>
        </Router>
    }
}