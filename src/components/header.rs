use leptos::prelude::*;
use crate::components::{MenuIcon, ThemeToggle};

#[component]
pub fn Header() -> impl IntoView {
    let (show_mobile_menu, set_show_mobile_menu) = signal(false);
    view! {
        <header class="sticky top-0 z-40 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
            <div class="container flex h-16 items-center justify-between px-4 sm:px-6">
                <div class="flex items-center gap-2">
                    <a href="/" class="font-bold text-lg sm:text-xl">
                        "CrawKatt"
                    </a>
                </div>
                <div class="hidden md:flex items-center gap-6">
                    <nav class="flex items-center gap-4 md:gap-6">
                        <a
                            href="#about"
                            class="text-sm font-medium hover:text-primary transition-colors"
                        >
                            "Acerca de"
                        </a>
                        <a
                            href="#projects"
                            class="text-sm font-medium hover:text-primary transition-colors"
                        >
                            "Proyectos"
                        </a>
                        <a
                            href="#skills"
                            class="text-sm font-medium hover:text-primary transition-colors"
                        >
                            "Habilidades"
                        </a>
                        <a
                            href="#contact"
                            class="text-sm font-medium hover:text-primary transition-colors"
                        >
                            "Contacto"
                        </a>
                        <a
                            href="/contributions"
                            class="text-sm font-medium hover:text-primary transition-colors"
                        >
                            "Open Source"
                        </a>
                    </nav>
                    <ThemeToggle />
                </div>
                <button
                    class="md:hidden p-2"
                    on:click=move |_| set_show_mobile_menu.update(|v| *v = !*v)
                >
                    <MenuIcon />
                </button>
            </div>

            <Show when=move || show_mobile_menu.get()>
                <div class="md:hidden absolute w-full bg-background border-b">
                    <nav class="flex flex-col p-4 gap-4">
                        <a href="#about" class="text-sm font-medium hover:text-primary">
                            "Acerca de"
                        </a>
                        <a href="#projects" class="text-sm font-medium hover:text-primary">
                            "Proyectos"
                        </a>
                        <a href="#skills" class="text-sm font-medium hover:text-primary">
                            "Habilidades"
                        </a>
                        <a href="#contact" class="text-sm font-medium hover:text-primary">
                            "Contacto"
                        </a>
                        <div class="pt-4 border-t flex justify-center">
                            <ThemeToggle />
                        </div>
                    </nav>
                </div>
            </Show>
        </header>
    }
}