use crate::frontend::components::{MoonIcon, SunIcon};
use leptos::prelude::*;
use leptos_fluent::move_tr;

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let toggle = move |_| {
        let Some(html) = document().document_element() else {
            return;
        };
        let dark = !html.class_list().contains("dark");
        let _ = html.class_list().toggle_with_force("dark", dark);
        if let Ok(Some(storage)) = window().local_storage() {
            let _ = storage.set_item("theme", if dark { "dark" } else { "light" });
        }
    };

    view! {
        <button
            type="button"
            class="grid place-items-center w-11 h-11 border-2 transition-colors border-line-strong text-fg-muted hover:border-fg hover:text-fg"
            on:click=toggle
            aria-label=move_tr!("a11y-theme")
        >
            <span class="dark:hidden">
                <MoonIcon />
            </span>
            <span class="hidden dark:block">
                <SunIcon />
            </span>
        </button>
    }
}
