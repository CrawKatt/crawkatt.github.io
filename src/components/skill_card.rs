use leptos::prelude::*;

#[component]
pub fn SkillCard(name: String) -> impl IntoView {
    view! {
        <div class="bg-card p-6 rounded-lg border text-center hover:border-primary transition-colors">
            <h3 class="font-medium text-lg">{name}</h3>
        </div>
    }
}

