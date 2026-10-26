use super::journey::TripBar;
use leptos::prelude::*;
use leptos_fluent::{move_tr, I18n};

struct Section {
    title: &'static str,
    intro: Option<&'static str>,
    items: &'static [(&'static str, bool)],
    note: Option<&'static str>,
}

const SECTIONS: [Section; 12] = [
    Section {
        title: "terms-section1-title",
        intro: Some("terms-section1-content"),
        items: &[],
        note: None,
    },
    Section {
        title: "terms-section2-title",
        intro: Some("terms-section2-intro"),
        items: &[
            ("terms-section2-item1", true),
            ("terms-section2-item2", true),
            ("terms-section2-item3", true),
            ("terms-section2-item4", true),
            ("terms-section2-item5", true),
            ("terms-section2-item6", true),
        ],
        note: Some("terms-section2-note"),
    },
    Section {
        title: "terms-section2-1-title",
        intro: Some("terms-section2-1-intro"),
        items: &[
            ("terms-section2-1-item1", true),
            ("terms-section2-1-item2", true),
        ],
        note: Some("terms-section2-note"),
    },
    Section {
        title: "terms-section3-title",
        intro: Some("terms-section3-content"),
        items: &[],
        note: None,
    },
    Section {
        title: "terms-section4-title",
        intro: Some("terms-section4-content"),
        items: &[],
        note: None,
    },
    Section {
        title: "terms-section5-title",
        intro: Some("terms-section5-content"),
        items: &[],
        note: None,
    },
    Section {
        title: "terms-section6-title",
        intro: None,
        items: &[
            ("terms-section6-item1", false),
            ("terms-section6-item2", false),
            ("terms-section6-item3", true),
        ],
        note: None,
    },
    Section {
        title: "terms-section7-title",
        intro: None,
        items: &[
            ("terms-section7-item1", false),
            ("terms-section7-item2", false),
        ],
        note: None,
    },
    Section {
        title: "terms-section8-title",
        intro: None,
        items: &[("terms-section8-item1", false)],
        note: None,
    },
    Section {
        title: "terms-section9-title",
        intro: None,
        items: &[
            ("terms-section9-item1", false),
            ("terms-section9-item2", false),
        ],
        note: None,
    },
    Section {
        title: "terms-section10-title",
        intro: None,
        items: &[
            ("terms-section10-item1", false),
            ("terms-section10-item2", false),
        ],
        note: None,
    },
    Section {
        title: "terms-section11-title",
        intro: None,
        items: &[
            ("terms-section11-item1", true),
            ("terms-section11-item2", true),
            ("terms-section11-item3", true),
        ],
        note: None,
    },
];

#[component]
pub fn TermsAndConditions() -> impl IntoView {
    let i18n = expect_context::<I18n>();

    view! {
        <TripBar />
        <main id="main" class="terms-page">
            <article class="terms-map">
                <p class="terms-label">{move_tr!("terms-label")}</p>
                <h1 class="terms-title">{move_tr!("terms-title")}</h1>
                <p class="mt-6 text-lg leading-relaxed">{move_tr!("terms-intro")}</p>
                {SECTIONS
                    .iter()
                    .map(|section| {
                        view! {
                            <section class="terms-section">
                                <h2 class="text-2xl font-bold font-pixel">
                                    {move || i18n.tr(section.title)}
                                </h2>
                                {section
                                    .intro
                                    .map(|key| {
                                        view! {
                                            <p class="mt-4 leading-relaxed">{move || i18n.tr(key)}</p>
                                        }
                                    })}
                                {(!section.items.is_empty())
                                    .then(|| {
                                        view! {
                                            <ul class="mt-4 space-y-3">
                                                {section
                                                    .items
                                                    .iter()
                                                    .map(|&(key, restricted)| {
                                                        view! {
                                                            <li class="flex gap-3 leading-relaxed">
                                                                <span class=if restricted {
                                                                    "terms-x"
                                                                } else {
                                                                    "terms-dot"
                                                                }></span>
                                                                <span>{move || i18n.tr(key)}</span>
                                                            </li>
                                                        }
                                                    })
                                                    .collect_view()}
                                            </ul>
                                        }
                                    })}
                                {section
                                    .note
                                    .map(|key| {
                                        view! { <p class="terms-note">{move || i18n.tr(key)}</p> }
                                    })}
                            </section>
                        }
                    })
                    .collect_view()}
            </article>
        </main>
    }
}
