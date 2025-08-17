use leptos::prelude::*;

#[derive(Clone)]
pub struct ThemeContext {
    pub is_dark: RwSignal<bool>,
}

pub fn provide_theme_context() -> ThemeContext {
    let is_dark = RwSignal::new(false);
    let ctx = ThemeContext { is_dark };
    provide_context(ctx.clone());
    ctx
}

pub fn use_theme_context() -> ThemeContext {
    use_context::<ThemeContext>().expect("ThemeContext not found")
}