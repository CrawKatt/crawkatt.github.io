use leptos::prelude::*;
use leptos_fluent::{move_tr, I18n};

#[component]
pub fn LanguageSelector() -> impl IntoView {
    let i18n = expect_context::<I18n>();

    view! {
        <div
            role="group"
            aria-label=move_tr!("a11y-language")
            class="flex border-2 border-line-strong"
        >
            {i18n
                .languages
                .iter()
                .map(|&lang| {
                    let code = lang.id.to_string();
                    let lang_attr = code.clone();
                    view! {
                        <button
                            type="button"
                            lang=lang_attr
                            aria-label=lang.name
                            aria-pressed=move || (i18n.language.get() == lang).to_string()
                            class="px-2.5 h-8 font-mono font-medium uppercase transition-colors text-[13px] text-fg-subtle aria-pressed:bg-fg aria-pressed:text-bg hover:text-fg"
                            on:click=move |_| i18n.language.set(lang)
                        >
                            {code}
                        </button>
                    }
                })
                .collect_view()}
        </div>
    }
}
