use crate::components::{ContactForm, DiscordIcon, GithubIcon, MailIcon, MenuIcon, ProjectCard, SkillCard, ThemeToggle, XIcon};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    let (show_mobile_menu, set_show_mobile_menu) = signal(false);

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
                            <a href="/contributions" class="text-sm font-medium hover:text-primary transition-colors">
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
                <section class="relative py-12 md:py-20 min-h-[500px] flex items-center justify-center overflow-hidden">
                    <video
                        autoplay
                        muted
                        loop
                        class="absolute inset-0 w-full h-full object-cover blur-lg z-0"
                    >
                        <source src="/public/background-video.webm" type="video/webm"/>
                        "Tu navegador no soporta videos HTML5."
                    </video>
                    <div class="absolute inset-0 bg-black/50 z-10"></div>

                    <div class="relative z-20 flex flex-col items-center text-center">
                        <div class="relative w-24 h-24 md:w-32 md:h-32 mb-6 md:mb-8 rounded-full overflow-hidden border-4 border-primary">
                            <img
                                src="/public/profile.png"
                                alt="Profile"
                                class="object-cover w-full h-full"
                            />
                        </div>
                        <h1 class="text-3xl md:text-4xl lg:text-6xl font-bold tracking-tight mb-4 text-white">
                            "CrawKatt"
                        </h1>
                        <p class="text-lg md:text-xl lg:text-2xl mb-6 max-w-2xl text-white/80">
                            "Minecraft Modder & Rust Developer"
                        </p>
                        <div class="flex flex-col sm:flex-row gap-3 w-full sm:w-auto">
                            <a
                                href="#contact"
                                class="inline-flex items-center justify-center rounded-md text-sm font-medium h-10 px-4 py-2 bg-primary text-white hover:bg-primary/90 transition-colors"
                            >
                                "Contact Me"
                            </a>
                            <a
                                href="#projects"
                                class="inline-flex items-center justify-center rounded-md text-sm font-medium h-10 px-4 py-2 bg-white/20 text-white hover:bg-white/30 backdrop-blur-sm border border-white/30 transition-all"
                            >
                                "Ver Proyectos"
                            </a>
                        </div>
                    </div>
                </section>

                <section id="about" class="py-12 md:py-16 scroll-mt-20">
                    <h2 class="text-2xl md:text-3xl font-bold mb-6 md:mb-8 text-center">"Sobre Mí"</h2>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-8 md:gap-12 items-center">
                        <div class="order-2 md:order-1">
                            <p class="text-base md:text-lg mb-4">
                                "¡Hola! Soy un desarrollador apasionado con experiencia en la creación de mods para Minecraft y programación en Rust."
                            </p>
                            <p class="text-base md:text-lg mb-4">
                                "Desde que era niño siempre me entusiasmé por la computación y la tecnología, crecí estudiando por mi cuenta siguiendo
                                guías, foros y tutoriales en YouTube. Cursé múltiples cursos de Udemy sobre programación y Modding para Minecraft."
                            </p>
                            <p class="text-base md:text-lg">
                                "Me dedico principalmente al Modding para Minecraft, BackEnd y Administración de Sistemas.
                                Soy un apasionado usuario de Arch Linux y me encanta aprender y desarrollar proyectos únicos para los usuarios."
                            </p>
                        </div>
                        <div class="relative h-60 md:h-80 rounded-lg overflow-hidden order-1 md:order-2">
                            <img
                                src="/public/about.png"
                                alt="About me"
                                class="object-cover w-full h-full"
                            />
                        </div>
                    </div>
                </section>

                <section id="projects" class="py-12 md:py-16 scroll-mt-20">
                    <h2 class="text-2xl md:text-3xl font-bold mb-6 md:mb-8 text-center">"Mis Proyectos"</h2>
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 md:gap-6">
                        <ProjectCard
                            title="Meica Mod".to_string()
                            description="Mod de Minecraft inspirado en la Vtuber Meica05".to_string()
                            image="/public/meica_mod.png".to_string()
                            demo="https://www.curseforge.com/minecraft/mc-mods/meica-mod".to_string()
                            source_code="https://github.com/CrawKatt/meica_mod".to_string()
                        />
                        <ProjectCard
                            title="Leafy".to_string()
                            description="Bot de Discord hecho en Rust".to_string()
                            image="/public/leafy.png".to_string()
                            demo="#".to_string()
                            source_code="https://github.com/CrawKatt/leafy".to_string()
                        />
                        <ProjectCard
                            title="Leafy Dashboard".to_string()
                            description="Una Dashboard Web hecha en Leptos, Actix y SurrealDB para configurar a Leafy".to_string()
                            image="/public/leafy.png".to_string()
                            demo="#".to_string()
                            source_code="https://github.com/CrawKatt/leafy_dashboard".to_string()
                        />
                    </div>
                </section>

                <section id="skills" class="py-12 md:py-16 scroll-mt-20">
                    <h2 class="text-2xl md:text-3xl font-bold mb-6 md:mb-8 text-center">"Skills"</h2>
                    <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-3 md:gap-4">
                        <SkillCard name="Rust".to_string() />
                        <SkillCard name="Leptos".to_string() />
                        <SkillCard name="TailwindCSS".to_string() />
                        <SkillCard name="Forge".to_string() />
                        <SkillCard name="Fabric".to_string() />
                        <SkillCard name="SurrealDB".to_string() />
                        <SkillCard name="Java".to_string() />
                        <SkillCard name="C#".to_string() />
                    </div>
                </section>

                <section id="contact" class="py-12 md:py-16 scroll-mt-20">
                    <h2 class="text-2xl md:text-3xl font-bold mb-6 md:mb-8 text-center">"Contácteme"</h2>
                    <div class="w-full md:max-w-md mx-auto">
                        <div class="flex justify-center gap-4 mb-6">
                            <a
                                href="https://github.com/CrawKatt"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="inline-flex items-center justify-center rounded-md text-sm font-medium h-10 w-10 border border-input bg-background"
                            >
                                <GithubIcon/>
                                <span class="sr-only">"GitHub"</span>
                            </a>
                            <a
                                href="https://x.com/CrawKatt"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="inline-flex items-center justify-center rounded-md text-sm font-medium h-10 w-10 border border-input bg-background"
                            >
                                <XIcon/>
                                <span class="sr-only">"GitHub"</span>
                            </a>
                            <a
                                href="https://www.discord.com/users/395631548629516298"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="inline-flex items-center justify-center rounded-md text-sm font-medium h-10 w-10 border border-input bg-background"
                            >
                                <DiscordIcon/>
                                <span class="sr-only">"GitHub"</span>
                            </a>
                            <a
                                href="mailto:man849916@gmail.com"
                                class="inline-flex items-center justify-center rounded-md text-sm font-medium h-10 w-10 border border-input bg-background"
                            >
                                <MailIcon/>
                                <span class="sr-only">"Email"</span>
                            </a>
                        </div>
                        <ContactForm/>
                    </div>
                </section>
            </main>

            <footer class="border-t py-6">
                <div class="container flex flex-col items-center justify-center gap-4 md:flex-row md:justify-between px-4 sm:px-6">
                    <p class="text-center text-sm leading-loose text-muted-foreground md:text-left">
                        {"© 2025"}{" CrawKatt. All rights reserved."}
                    </p>
                    <div class="flex items-center gap-4">
                        <a
                            href="https://github.com"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="inline-flex items-center justify-center rounded-md text-sm font-medium h-8 w-8"
                        >
                            <div class="h-4 w-4">
                                <GithubIcon/>
                            </div>
                            <span class="sr-only">"GitHub"</span>
                        </a>
                        <a
                            href="mailto:contact@example.com"
                            class="inline-flex items-center justify-center rounded-md text-sm font-medium h-8 w-8"
                        >
                            <div class="h-4 w-4">
                                <MailIcon/>
                            </div>
                            <span class="sr-only">"Email"</span>
                        </a>
                    </div>
                </div>
            </footer>
        </div>
    }
}