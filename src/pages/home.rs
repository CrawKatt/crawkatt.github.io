use crate::components::{ContactForm, DiscordIcon, GithubIcon, MailIcon, ProjectCard, SkillCard, ThemeToggle, XIcon};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-background">
            <header class="sticky top-0 z-40 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
                <div class="container flex h-16 items-center justify-between">
                    <div class="flex items-center gap-2">
                        <a href="/" class="font-bold text-xl">
                            "Portafolio"
                        </a>
                    </div>

                    <div class="flex items-center gap-6">
                        <nav class="flex items-center gap-6">
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
                        </nav>
                        <ThemeToggle />
                    </div>
                </div>
            </header>
            <main class="container py-12">
                <section class="py-20 md:py-32 flex flex-col items-center text-center">
                    <div class="relative w-32 h-32 mb-8 rounded-full overflow-hidden border-4 border-primary">
                        <img
                            src="/public/profile.png"
                            alt="Profile"
                            class="object-cover w-full h-full"
                        />
                    </div>
                    <h1 class="text-4xl md:text-6xl font-bold tracking-tight mb-4">
                        "CrawKatt"
                    </h1>
                    <p class="text-xl md:text-2xl text-muted-foreground mb-8 max-w-2xl">
                        "Minecraft Modder & Rust Developer"
                    </p>
                    <div class="flex gap-4">
                        <a
                            href="#contact"
                            class="inline-flex items-center justify-center rounded-md text-sm font-medium h-10 px-4 py-2 bg-primary text-primary-foreground"
                        >
                            "Contact Me"
                        </a>
                        <a
                            href="#projects"
                            class="inline-flex items-center justify-center rounded-md text-sm font-medium h-10 px-4 py-2 border border-input bg-background"
                        >
                            "Ver Proyectos"
                        </a>
                    </div>
                </section>
                <section id="about" class="py-16 scroll-mt-20">
                    <h2 class="text-3xl font-bold mb-8 text-center">"Sobre Mí"</h2>
                    <div class="grid md:grid-cols-2 gap-12 items-center">
                        <div>
                            <p class="text-lg mb-4">
                                "¡Hola! Soy un desarrollador apasionado con experiencia en la creación de mods para Minecraft y programación en Rust."
                            </p>
                            <p class="text-lg mb-4">
                                "Desde que era niño siempre me entusiasmé por la computación y la tecnología, crecí estudiando por mi cuenta siguiendo
                                guías, foros y tutoriales en YouTube. Cursé múltiples cursos de Udemy sobre programación y Modding para Minecraft."
                            </p>
                            <p class="text-lg">
                                "Me dedico principalmente al Modding para Minecraft, BackEnd y Administración de Sistemas.
                                Soy un apasionado usuario de Arch Linux y me encanta aprender y desarrollar proyectos únicos para los usuarios."
                            </p>
                        </div>
                        <div class="relative h-80 rounded-lg overflow-hidden">
                            <img
                                src="/public/about.png"
                                alt="About me"
                                class="object-cover w-full h-full"
                            />
                        </div>
                    </div>
                </section>
                <section id="projects" class="py-16 scroll-mt-20">
                    <h2 class="text-3xl font-bold mb-8 text-center">"My Projects"</h2>
                    <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
                        <ProjectCard
                            title="Meica Mod".to_string()
                            description="Minecraft Mod inspired in Meica05 Vtuber".to_string()
                            image="/public/meica_mod.png".to_string()
                        />
                        <ProjectCard
                            title="Leafy".to_string()
                            description="A Discord Bot developed in Rust".to_string()
                            image="/public/leafy.png".to_string()
                        />
                        <ProjectCard
                            title="Leafy Dashboard".to_string()
                            description="A Leptos and Actix Powered FullStack Web Dashboard for config Leafy".to_string()
                            image="/public/leafy.png".to_string()
                        />
                    </div>
                </section>
                <section id="skills" class="py-16 scroll-mt-20">
                    <h2 class="text-3xl font-bold mb-8 text-center">"Skills"</h2>
                    <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
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
                <section id="contact" class="py-16 scroll-mt-20">
                    <h2 class="text-3xl font-bold mb-8 text-center">"Get In Touch"</h2>
                    <div class="max-w-md mx-auto">
                        <div class="flex justify-center gap-6 mb-8">
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
                <div class="container flex flex-col items-center justify-center gap-4 md:flex-row md:justify-between">
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