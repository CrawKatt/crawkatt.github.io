use leptos::prelude::*;
use crate::components::{GithubIcon, MailIcon};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t py-6">
            <div class="container flex flex-col items-center justify-center gap-4 md:flex-row md:justify-between px-4 sm:px-6">
                <p class="text-center text-sm leading-loose text-muted-foreground md:text-left">
                    "© 2025 CrawKatt. All rights reserved."
                </p>
                <div class="flex items-center gap-4">
                    <a
                        href="https://github.com"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="inline-flex items-center justify-center rounded-md text-sm font-medium h-8 w-8"
                    >
                        <div class="h-4 w-4">
                            <GithubIcon />
                        </div>
                        <span class="sr-only">"GitHub"</span>
                    </a>
                    <a
                        href="mailto:contact@example.com"
                        class="inline-flex items-center justify-center rounded-md text-sm font-medium h-8 w-8"
                    >
                        <div class="h-4 w-4">
                            <MailIcon />
                        </div>
                        <span class="sr-only">"Email"</span>
                    </a>
                    <a
                        href="/terms-and-conditions"
                        class="text-center text-sm leading-loose text-muted-foreground md:text-right hover:text-primary"
                    >
                        "Términos y Condiciones."
                    </a>
                </div>
            </div>
        </footer>
    }
}