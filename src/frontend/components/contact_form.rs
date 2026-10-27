use crate::server::send_contact;
use leptos::ev;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_fluent::move_tr;

#[derive(Clone, Copy, PartialEq)]
enum Status {
    Idle,
    Missing,
    Sending,
    Sent,
    Failed,
}

#[component]
pub fn ContactForm() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (discord, set_discord) = signal(String::new());
    let (message, set_message) = signal(String::new());
    let (accepted_terms, set_accepted_terms) = signal(false);
    let status = RwSignal::new(Status::Idle);

    let handle_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let name_val = name.get_untracked().trim().to_string();
        let discord_val = discord.get_untracked().trim().to_string();
        let message_val = message.get_untracked().trim().to_string();

        if name_val.is_empty()
            || discord_val.is_empty()
            || message_val.is_empty()
            || !accepted_terms.get_untracked()
        {
            status.set(Status::Missing);
            return;
        }

        status.set(Status::Sending);
        spawn_local(async move {
            match send_contact(name_val, discord_val, message_val).await {
                Ok(()) => {
                    set_name.set(String::new());
                    set_discord.set(String::new());
                    set_message.set(String::new());
                    set_accepted_terms.set(false);
                    status.set(Status::Sent);
                }
                Err(_) => status.set(Status::Failed),
            }
        });
    };

    let feedback = move || match status.get() {
        Status::Idle => None,
        Status::Missing => Some(("text-danger", move_tr!("form-missing"))),
        Status::Sending => Some(("text-fg-muted", move_tr!("form-sending"))),
        Status::Sent => Some(("text-mc", move_tr!("form-sent"))),
        Status::Failed => Some(("text-danger", move_tr!("form-error"))),
    };

    view! {
        <form
            on:submit=handle_submit
            class="grid gap-6 content-start self-start p-6 border-2 sm:p-8 border-line bg-surface"
        >
            <div class="grid gap-6 sm:grid-cols-2">
                <div>
                    <label for="name" class="field-label">
                        {move_tr!("form-name")}
                    </label>
                    <input
                        id="name"
                        name="name"
                        autocomplete="name"
                        required
                        class="field"
                        placeholder=move_tr!("form-name-placeholder")
                        on:input=move |ev| set_name.set(event_target_value(&ev))
                        prop:value=name
                    />
                </div>
                <div>
                    <label for="discord" class="field-label">
                        {move_tr!("form-discord")}
                    </label>
                    <input
                        id="discord"
                        name="discord"
                        autocomplete="off"
                        spellcheck="false"
                        required
                        class="field"
                        placeholder=move_tr!("form-discord-placeholder")
                        on:input=move |ev| set_discord.set(event_target_value(&ev))
                        prop:value=discord
                    />
                </div>
            </div>
            <div>
                <label for="message" class="field-label">
                    {move_tr!("form-message")}
                </label>
                <textarea
                    id="message"
                    name="message"
                    required
                    rows="6"
                    class="resize-y field"
                    placeholder=move_tr!("form-message-placeholder")
                    on:input=move |ev| set_message.set(event_target_value(&ev))
                    prop:value=message
                ></textarea>
            </div>
            <label
                for="terms"
                class="flex gap-3 items-start cursor-pointer text-[15px] text-fg-muted"
            >
                <input
                    id="terms"
                    type="checkbox"
                    required
                    class="mt-0.5 w-5 h-5 cursor-pointer shrink-0 accent-gold"
                    on:change=move |ev| set_accepted_terms.set(event_target_checked(&ev))
                    prop:checked=accepted_terms
                />
                <span>
                    {move_tr!("form-terms")} " "
                    <a href="/terms-and-conditions" class="link-u text-fg">
                        {move_tr!("form-terms-link")}
                    </a>
                </span>
            </label>
            <div class="flex flex-col gap-4 sm:flex-row sm:items-center">
                <button
                    type="submit"
                    class="sm:w-auto btn btn-primary"
                    disabled=move || !accepted_terms.get() || status.get() == Status::Sending
                >
                    {move_tr!("form-submit")}
                </button>
                <p role="status" aria-live="polite" class="leading-snug text-[15px]">
                    {move || {
                        feedback().map(|(tone, text)| view! { <span class=tone>{text}</span> })
                    }}
                </p>
            </div>
        </form>
    }
}
