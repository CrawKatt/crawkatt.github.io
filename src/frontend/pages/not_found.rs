use super::journey::{layer, TripBar};
use leptos::prelude::*;
use leptos_fluent::move_tr;

#[component]
pub fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    if let Some(response) = use_context::<leptos_actix::ResponseOptions>() {
        response.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    let scenery = |suffix: &'static str| {
        view! {
            <div class=if suffix.is_empty() {
                "scene-day"
            } else {
                "scene-night"
            }>
                {layer("farlands", "back", suffix)} {layer("farlands", "far", suffix)}
                {layer("farlands", "main", suffix)}
            </div>
        }
    };

    view! {
        <TripBar />
        <main id="main">
            <section class="scene scene-surface scene-farlands" aria-labelledby="notfound-title">
                <div class="p-6 sm:p-8 surface-copy end-panel">
                    <p class="text-5xl font-bold font-pixel">"404"</p>
                    <h1
                        id="notfound-title"
                        class="mt-3 font-bold font-pixel text-[clamp(2.25rem,4.1vw,3.6rem)] leading-[.95]"
                    >
                        {move_tr!("notfound-title")}
                    </h1>
                    <p class="mt-5 text-lg leading-relaxed max-w-[32rem]">
                        {move_tr!("notfound-text")}
                    </p>
                    <a href="/" class="mt-7 btn btn-primary">
                        {move_tr!("notfound-back")}
                    </a>
                </div>
                <div class="scene-frame">
                    <div class="stage" role="img" aria-label=move_tr!("notfound-scene")>
                        {scenery("")}
                        {scenery("-night")}
                    </div>
                </div>
            </section>
        </main>
    }
}
