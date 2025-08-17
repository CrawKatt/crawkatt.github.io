use leptos::prelude::*;
use leptos_fluent::move_tr;
use crate::frontend::components::{MenuIcon, ThemeToggle, LanguageSelector};

#[component]
pub fn Header() -> impl IntoView {
    let (show_mobile_menu, set_show_mobile_menu) = signal(false);

    view! {
        <header class="sticky top-0 z-40 w-full border-b bg-background/95 supports-[backdrop-filter]:bg-background">
            <div class="container flex h-[88px] items-center justify-between px-4 sm:px-6">
                <div class="flex items-center gap-2">
                    <a href="/" class="font-bold text-lg sm:text-xl">"CrawKatt"</a>
                </div>
                <div class="hidden md:flex items-center gap-6 text-sm font-medium transition-colors">
                    <nav class="flex items-center gap-4 md:gap-6">
                        <a href="#about" class="hover:text-primary">{move_tr!("header-about")}</a>
                        <a href="#projects" class="hover:text-primary">{move_tr!("header-projects")}</a>
                        <a href="#skills" class="hover:text-primary">{move_tr!("header-skills")}</a>
                        <a href="#contact" class="hover:text-primary">{move_tr!("header-contact")}</a>
                        <a href="/contributions" class="hover:text-primary">{move_tr!("header-contributions")}</a>
                        <a href="/terms-and-conditions" class="hover:text-primary">{move_tr!("header-terms")}</a>
                    </nav>
                    <div class="flex items-center gap-4">
                        <LanguageSelector/>
                        <ThemeToggle />
                    </div>
                </div>
                <button
                    class="md:hidden p-2"
                    on:click=move |_| set_show_mobile_menu.update(|v| *v = !*v)
                >
                    <MenuIcon />
                </button>
            </div>

            <Show when=move || show_mobile_menu.get()>
                <div class="md:hidden absolute w-full bg-background border-b text-sm font-medium">
                    <nav class="flex flex-col p-4 gap-4">
                        <a href="#about" class="hover:text-primary">{move_tr!("header-about")}</a>
                        <a href="#projects" class="hover:text-primary">{move_tr!("header-projects")}</a>
                        <a href="#skills" class="hover:text-primary">{move_tr!("header-skills")}</a>
                        <a href="#contact" class="hover:text-primary">{move_tr!("header-contact")}</a>
                        <div class="pt-4 border-t flex flex-col gap-4 justify-center">
                            <LanguageSelector/>
                            <ThemeToggle />
                        </div>
                    </nav>
                </div>
            </Show>
        </header>
    }
}