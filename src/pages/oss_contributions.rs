use bon::Builder;
use crate::components::{GithubIcon, Header, LinkIcon, StarIcon};
use leptos::prelude::*;
use crate::context::provide_theme_context;

#[component]
pub fn OSSContributions() -> impl IntoView {
    let _ = provide_theme_context();
    let contributions = vec![
        Contribution::builder()
            .title("Teloxide Contribution")
            .description("Added example improving docs")
            .repo_url("https://github.com/teloxide/teloxide")
            .pr_url("https://github.com/teloxide/teloxide/pull/915")
            .stars(3476)
            .merged(true)
            .build(),

        Contribution::builder()
            .title("RustLangEs Website Contribution")
            .description("Frases añadidas al sitio, añadido rust-toolchain.toml para aplicar nightly por defecto y añadido apartado de Salamandra Devs en comunidades")
            .repo_url("https://github.com/RustLangES/RustLangES.github.io")
            .pr_url("https://github.com/RustLangES/RustLangES.github.io/pull/25")
            .stars(18)
            .merged(true)
            .build(),

        Contribution::builder()
            .title("Traducción del Libro de Rust de la comunidad de RustLangEs")
            .description("Añadiendo traducción de los Slices")
            .repo_url("https://github.com/RustLangES/rust-book-es")
            .pr_url("https://github.com/RustLangES/rust-book-es/pull/2")
            .stars(96)
            .merged(true)
            .build()
    ];

    view! {
        <div class="min-h-screen bg-background">
            <Header/>
            <main class="container py-8 px-4 sm:px-6">
                <section class="py-12 md:py-20">
                    <h1 class="text-3xl md:text-4xl font-bold text-center mb-8">
                        "Open Source Contributions"
                    </h1>

                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        {contributions
                            .into_iter()
                            .map(|contribution| {
                                let title = contribution.title;
                                let merged = contribution.merged;
                                let repo = contribution.repo_url;
                                view! {
                                    <div class="bg-card rounded-lg border p-6 shadow-sm">
                                        <div class="flex items-center gap-3 mb-4">
                                            <GithubIcon _class="h-6 w-6" />
                                            <a
                                                href=repo
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                class="text-xl font-semibold hover:text-primary hover:underline"
                                            >
                                                {title}
                                            </a>
                                            <div class="ml-auto">
                                                <Show
                                                    when=move || merged
                                                    fallback=|| {
                                                        view! {
                                                            <span class="px-2 py-1 text-xs bg-blue-100 text-blue-800 rounded-full">
                                                                "Open"
                                                            </span>
                                                        }
                                                    }
                                                >
                                                    <span class="px-2 py-1 text-xs bg-green-100 text-green-800 rounded-full">
                                                        "Merged"
                                                    </span>
                                                </Show>
                                            </div>
                                        </div>

                                        <p class="text-muted-foreground mb-4">
                                            {contribution.description.clone()}
                                        </p>

                                        <div class="flex items-center gap-4 text-sm">
                                            <a
                                                href=contribution.pr_url.clone()
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                class="inline-flex items-center gap-2 text-primary hover:underline"
                                            >
                                                <LinkIcon _class="h-4 w-4" />
                                                "View PR"
                                            </a>
                                            <div class="flex items-center gap-1">
                                                <StarIcon _class="h-4 w-4" />
                                                <span>{contribution.stars}</span>
                                            </div>
                                        </div>
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </div>
                </section>
            </main>
        </div>
    }
}

#[derive(Clone, Builder)]
#[builder(on(String, into))]
struct Contribution {
    title: String,
    description: String,
    repo_url: String,
    pr_url: String,
    stars: i32,
    merged: bool,
}