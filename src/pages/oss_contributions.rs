use leptos::prelude::*;
use crate::components::{GithubIcon, LinkIcon, MenuIcon, StarIcon, ThemeToggle};

#[component]
pub fn OSSContributions() -> impl IntoView {
    let (show_mobile_menu, set_show_mobile_menu) = signal(false);
    let contributions = vec![
        Contribution {
            title: "Teloxide Contribution".to_string(),
            description: "Added example improving docs".to_string(),
            repo_url: "https://github.com/teloxide/teloxide".to_string(),
            pr_url: "https://github.com/teloxide/teloxide/pull/915".to_string(),
            stars: 3476,
            merged: true,
        },
        // Agrega más contribuciones aquí
    ];

    view! {
        <div class="min-h-screen bg-background">
            <header class="sticky top-0 z-40 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
                <div class="container flex h-16 items-center justify-between px-4 sm:px-6">
                    <div class="flex items-center gap-2">
                        <a href="/" class="font-bold text-lg sm:text-xl">
                            "CrawKatt"
                        </a>
                    </div>

                    <div class="hidden md:flex items-center gap-6">
                        <nav class="flex items-center gap-4 md:gap-6">
                            <a href="#about" class="text-sm font-medium hover:text-primary transition-colors">
                                "Acerca de"
                            </a>
                            <a href="#projects" class="text-sm font-medium hover:text-primary transition-colors">
                                "Proyectos"
                            </a>
                            <a href="#skills" class="text-sm font-medium hover:text-primary transition-colors">
                                "Habilidades"
                            </a>
                            <a href="#contact" class="text-sm font-medium hover:text-primary transition-colors">
                                "Contacto"
                            </a>
                            <a href="/oss" class="text-sm font-medium hover:text-primary transition-colors">
                                "Open Source"
                            </a>
                        </nav>
                        <ThemeToggle />
                    </div>

                    <button
                        class="md:hidden p-2"
                        on:click=move |_| set_show_mobile_menu.update(|v| *v = !*v)
                    >
                        <MenuIcon/>
                    </button>
                </div>

                <Show when=move || show_mobile_menu.get()>
                    <div class="md:hidden absolute w-full bg-background border-b">
                        <nav class="flex flex-col p-4 gap-4">
                            <a href="#about" class="text-sm font-medium hover:text-primary">"Acerca de"</a>
                            <a href="#projects" class="text-sm font-medium hover:text-primary">"Proyectos"</a>
                            <a href="#skills" class="text-sm font-medium hover:text-primary">"Habilidades"</a>
                            <a href="#contact" class="text-sm font-medium hover:text-primary">"Contacto"</a>
                            <div class="pt-4 border-t flex justify-center">
                                <ThemeToggle />
                            </div>
                        </nav>
                    </div>
                </Show>
            </header>
            <main class="container py-8 px-4 sm:px-6">
                <section class="py-12 md:py-20">
                    <h1 class="text-3xl md:text-4xl font-bold text-center mb-8">"Open Source Contributions"</h1>

                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        {contributions.into_iter().map(|contribution| {
                            let title = contribution.title;
                            let merged = contribution.merged;
                            let repo = contribution.repo_url;
                            view! {
                                <div class="bg-card rounded-lg border p-6 shadow-sm">
                                    <div class="flex items-center gap-3 mb-4">
                                        <GithubIcon _class="h-6 w-6"/>
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
                                                fallback=|| view! {
                                                    <span class="px-2 py-1 text-xs bg-blue-100 text-blue-800 rounded-full">
                                                        "Open"
                                                    </span>
                                                }
                                            >
                                                <span class="px-2 py-1 text-xs bg-green-100 text-green-800 rounded-full">
                                                    "Merged"
                                                </span>
                                            </Show>
                                        </div>
                                    </div>

                                    <p class="text-muted-foreground mb-4">{contribution.description.clone()}</p>

                                    <div class="flex items-center gap-4 text-sm">
                                        <a
                                            href=contribution.pr_url.clone()
                                            target="_blank"
                                            rel="noopener noreferrer"
                                            class="inline-flex items-center gap-2 text-primary hover:underline"
                                        >
                                            <LinkIcon _class="h-4 w-4"/>
                                            "View PR"
                                        </a>
                                        <div class="flex items-center gap-1">
                                            <StarIcon _class="h-4 w-4"/>
                                            <span>{contribution.stars}</span>
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </section>
            </main>
        </div>
    }
}

#[derive(Clone)]
struct Contribution {
    title: String,
    description: String,
    repo_url: String,
    pr_url: String,
    stars: i32,
    merged: bool,
}