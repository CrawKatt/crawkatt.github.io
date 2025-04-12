use leptos::prelude::*;
use leptos_fluent::{expect_i18n, Language};

#[component]
pub fn LanguageSelector() -> impl IntoView {
    let i18n = expect_i18n();
    let (show_options, set_show_options) = signal(false);

    view! {
        <div class="relative">
            <button
                on:click=move |_| set_show_options.update(|v| *v = !*v)
                class="flex items-center gap-2 px-3 py-2 rounded-md hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors"
            >
                <span>{move || i18n.language.get().name}</span>
                <svg
                    class="w-4 h-4 transition-transform"
                    class=("rotate-180", move || show_options.get())
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                >
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                </svg>
            </button>

            <Show when=move || show_options.get()>
                <div class="absolute right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-white dark:bg-gray-800 shadow-lg border border-gray-200 dark:border-gray-700">
                    <div class="p-2 space-y-1">
                        {move || {
                            i18n.languages.iter()
                                .map(|lang| render_language(lang, set_show_options))
                                .collect::<Vec<_>>()
                        }}
                    </div>
                </div>
            </Show>
        </div>
    }
}

fn render_language(lang: &'static Language, set_show: WriteSignal<bool>) -> impl IntoView {
    let i18n = expect_i18n();

    view! {
        <button
            on:click=move |_| {
                i18n.language.set(lang);
                set_show.set(false);
            }
            class="w-full px-3 py-2 text-left rounded-md text-sm hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
            class=("bg-gray-100 dark:bg-gray-700", move || lang.is_active())
        >
            {lang.name}
        </button>
    }
}