use crate::components::{Header, Hero, About, Projects, Skills, Contact, Footer};
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Header />
        <div class="relative">
            <Hero />
            <div class="h-86 z-20 relative overflow-hidden">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1440 320"><path fill="hsl(222.2, 84%, 4.9%)" fill-opacity="1" d="M0,224L80,234.7C160,245,320,267,480,266.7C640,267,800,245,960,224C1120,203,1280,181,1360,170.7L1440,160L1440,320L1360,320C1280,320,1120,320,960,320C800,320,640,320,480,320C320,320,160,320,80,320L0,320Z"></path></svg>
            </div>
            <main class="relative z-30 min-h-screen w-full bg-background">
                <div class="container mx-auto py-8 px-4 sm:px-6">
                    <About />
                    <Projects />
                    <Skills />
                    <Contact />
                </div>
            </main>
        </div>
        <Footer />
    }
}