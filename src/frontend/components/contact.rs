use leptos::prelude::*;
use leptos_fluent::move_tr;
use crate::frontend::components::{ContactForm, DiscordIcon, GithubIcon, MailIcon, XIcon};

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <section id="contact" class="py-12 md:py-16 scroll-mt-20">
            <h2 class="text-2xl md:text-3xl font-bold mb-6 md:mb-8 text-center">{move_tr!("contact-title")}</h2>
            <div class="w-full md:max-w-md mx-auto">
                <div class="flex justify-center gap-4 mb-6">
                    <a
                        href="https://github.com/CrawKatt"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="inline-flex items-center justify-center rounded-md text-sm font-medium h-10 w-10 border border-input bg-background"
                    >
                        <GithubIcon />
                        <span class="sr-only">"GitHub"</span>
                    </a>
                    <a
                        href="https://x.com/CrawKatt"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="inline-flex items-center justify-center rounded-md text-sm font-medium h-10 w-10 border border-input bg-background"
                    >
                        <XIcon />
                        <span class="sr-only">"GitHub"</span>
                    </a>
                    <a
                        href="https://www.discord.com/users/395631548629516298"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="inline-flex items-center justify-center rounded-md text-sm font-medium h-10 w-10 border border-input bg-background"
                    >
                        <DiscordIcon />
                        <span class="sr-only">"GitHub"</span>
                    </a>
                    <a
                        href="mailto:man849916@gmail.com"
                        class="inline-flex items-center justify-center rounded-md text-sm font-medium h-10 w-10 border border-input bg-background"
                    >
                        <MailIcon />
                        <span class="sr-only">"Email"</span>
                    </a>
                </div>
                <ContactForm />
            </div>
        </section>
    }
}