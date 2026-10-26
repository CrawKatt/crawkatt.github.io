use leptos::prelude::*;

fn pixel_icon(rows: [&'static str; 8], class: &'static str) -> impl IntoView {
    let rects = rows
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.bytes()
                .enumerate()
                .filter(|(_, b)| *b == b'#')
                .map(move |(x, _)| {
                    view! { <rect x=x.to_string() y=y.to_string() width="1" height="1" /> }
                })
        })
        .collect::<Vec<_>>();
    view! {
        <svg
            viewBox="0 0 8 8"
            class=class
            fill="currentColor"
            shape-rendering="crispEdges"
            aria-hidden="true"
        >
            {rects}
        </svg>
    }
}

#[component]
pub fn CloseIcon(#[prop(default = "h-4 w-4")] class: &'static str) -> impl IntoView {
    pixel_icon(
        [
            "##....##", ".##..##.", "..####..", "...##...", "..####..", ".##..##.", "##....##",
            "........",
        ],
        class,
    )
}

#[component]
pub fn ArrowDownIcon(#[prop(default = "h-4 w-4")] class: &'static str) -> impl IntoView {
    pixel_icon(
        [
            "...##...", "...##...", "...##...", "...##...", "##.##.##", ".######.", "..####..",
            "...##...",
        ],
        class,
    )
}

#[component]
pub fn ExternalIcon(#[prop(default = "h-4 w-4")] class: &'static str) -> impl IntoView {
    pixel_icon(
        [
            "...#####", "......##", ".....###", "....##.#", "...##..#", "..##....", ".##.....",
            "##......",
        ],
        class,
    )
}

#[component]
pub fn SunIcon(#[prop(default = "h-4 w-4")] class: &'static str) -> impl IntoView {
    pixel_icon(
        [
            "...##...", ".#....#.", "..####..", "#.####.#", "#.####.#", "..####..", ".#....#.",
            "...##...",
        ],
        class,
    )
}

#[component]
pub fn MoonIcon(#[prop(default = "h-4 w-4")] class: &'static str) -> impl IntoView {
    pixel_icon(
        [
            "..####..", ".###....", "###.....", "###.....", "###.....", "###....#", ".######.",
            "..####..",
        ],
        class,
    )
}
