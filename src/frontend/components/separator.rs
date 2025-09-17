use leptos::prelude::*;
use crate::frontend::context::*;

#[component]
pub fn Separator() -> impl IntoView {
    let theme_ctx = use_theme_context();
    let is_dark = theme_ctx.is_dark;

    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1440 310">
            <Show
                when=move || is_dark.get()
                fallback=|| view! {
                    <path
                        fill="hsl(0, 0%, 100%)"
                        fill-opacity="1"
                        d="M0,224L80,234.7C160,245,320,267,480,266.7C640,267,800,245,960,224C1120,203,1280,181,1360,170.7L1440,160L1440,320L1360,320C1280,320,1120,320,960,320C800,320,640,320,480,320C320,320,160,320,80,320L0,320Z"
                    />
                }
            >
                <path
                    fill="hsl(222.2, 84%, 4.9%)"
                    fill-opacity="1"
                    d="M0,224L80,234.7C160,245,320,267,480,266.7C640,267,800,245,960,224C1120,203,1280,181,1360,170.7L1440,160L1440,320L1360,320C1280,320,1120,320,960,320C800,320,640,320,480,320C320,320,160,320,80,320L0,320Z"
                />
            </Show>
        </svg>
    }
}