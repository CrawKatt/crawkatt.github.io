use crate::components::{Header, Hero, About, Projects, Skills, Contact, Footer};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-background">
            <Header />
            <main class="container py-8 px-4 sm:px-6">
                <Hero />
                <About />
                <Projects />
                <Skills />
                <Contact />
            </main>
            <Footer />
        </div>
    }
}