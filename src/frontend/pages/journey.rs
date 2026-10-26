use super::contributions::{Contribution, CONTRIBUTIONS, FEATURED};
use crate::frontend::components::{
    ArrowDownIcon, CloseIcon, ContactForm, ExternalIcon, LanguageSelector, Meta, Player3d, Project,
    ThemeToggle, EMAIL, MODS,
};
use crate::frontend::season::{current_season, Season};
use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use leptos_fluent::{move_tr, I18n};
use leptos_router::hooks::use_query_map;

fn parallax(frame: NodeRef<html::Div>) {
    Effect::new(move |_| {
        let reduced = window()
            .match_media("(prefers-reduced-motion: reduce)")
            .ok()
            .flatten()
            .is_some_and(|query| query.matches());
        if reduced {
            return;
        }
        let update = move || {
            let Some(el) = frame.get_untracked() else {
                return;
            };
            let rect = el.get_bounding_client_rect();
            let view_h = window()
                .inner_height()
                .ok()
                .and_then(|h| h.as_f64())
                .unwrap_or(0.0);
            let t = rect.top() + rect.height() / 2.0 - view_h / 2.0;
            let _ = el.set_attribute("style", &format!("--t:{t:.1}px"));
        };
        update();
        let on_scroll = window_event_listener(ev::scroll, move |_| update());
        let on_resize = window_event_listener(ev::resize, move |_| update());
        on_cleanup(move || {
            on_scroll.remove();
            on_resize.remove();
        });
    });
}

const W: u16 = 352;

pub fn layer(scene: &str, name: &'static str, suffix: &str) -> impl IntoView {
    let height = match scene {
        "cave" | "deepslate" | "nether" => "276",
        "farlands" => "228",
        _ => "180",
    };
    image(
        name,
        format!("/public/world/{scene}-{name}{suffix}.png"),
        height,
    )
}

fn image(name: &'static str, src: String, height: &'static str) -> impl IntoView {
    view! { <img class=format!("layer layer-{name}") src=src alt="" width="352" height=height /> }
}

fn layers(scene: &'static str, names: &'static [&'static str]) -> impl IntoView {
    names
        .iter()
        .map(|name| layer(scene, name, ""))
        .collect_view()
}

type Sway = (&'static str, u16, u16, u16, u16, u16, u16, f32);

const MIDTREE: Sway = ("midtree", 136, 78, 64, 32, 0, 64, 1.2);
const SWAYING: [Sway; 12] = [
    ("oak", 280, 36, 80, 48, 0, 80, 1.6),
    ("tall", 0, 96, 16, 32, 16, 32, 5.0),
    ("tall", 16, 96, 16, 32, 0, 32, 6.0),
    ("tall", 318, 112, 16, 32, 0, 32, 6.0),
    ("tall", 334, 112, 16, 32, 16, 32, 5.0),
    ("plants", 36, 128, 16, 16, 0, 96, 7.0),
    ("plants", 54, 128, 16, 16, 16, 96, 7.0),
    ("plants", 70, 128, 16, 16, 32, 96, 8.0),
    ("plants", 88, 128, 16, 16, 80, 96, 5.0),
    ("plants", 112, 128, 16, 16, 48, 96, 7.0),
    ("plants", 128, 128, 16, 16, 64, 96, 6.0),
    ("plants", 300, 128, 16, 16, 0, 96, 7.0),
];

const SPRUCE: Sway = ("oak", 280, 40, 80, 88, 0, 80, 1.6);
const TREE_LIGHTS: Sway = ("lights", 280, 40, 80, 88, 0, 80, 1.6);

fn sway(
    (file, x, y, w, h, frame, sheet, amp): Sway,
    suffix: &str,
    season: Season,
) -> impl IntoView {
    let delay = f32::from(W - x) / f32::from(W) * 1.6;
    let style = format!(
        "--x:{x};--y:{y};--w:{w};--h:{h};--f:{frame};--sheet:{sheet};--amp:{amp}deg;--delay:{delay:.2}s;\
         background-image:url({})",
        season.art(&format!("{file}{suffix}"))
    );
    view! { <span class="sway" style=style></span> }
}

fn scenery(night: bool, season: Season) -> impl IntoView {
    let suffix = if night { "-night" } else { "" };
    let layer = move |name| image(name, season.art(&format!("surface-{name}{suffix}")), "180");
    view! {
        <div class=if night {
            "scene-night"
        } else {
            "scene-day"
        }>
            {layer("back")} <div class="layer layer-back">
                <div
                    class="clouds"
                    style=format!(
                        "background-image:url({})",
                        season.art(&format!("clouds{suffix}")),
                    )
                ></div>
            </div> {layer("far")} {layer("mid")}
            <div class="layer layer-mid">{sway(MIDTREE, suffix, season)}</div> {layer("main")}
            {SWAYING
                .into_iter()
                .map(|s| if season == Season::Christmas && s.0 == "oak" { SPRUCE } else { s })
                .map(|s| sway(s, suffix, season))
                .collect_view()} {night.then(|| layer("light"))}
            {(night && season == Season::Christmas)
                .then(|| {
                    view! {
                        <div class="twinkle">
                            {layer("twinkle")} {sway(TREE_LIGHTS, "", season)}
                        </div>
                    }
                })}
        </div>
    }
}

const GUSTS: [(u16, u16, f32); 2] = [(214, 26, 0.2), (150, 98, 4.2)];

const FIREFLIES: [(u16, u16, u8); 5] = [
    (84, 124, 0),
    (98, 120, 3),
    (108, 132, 5),
    (90, 136, 2),
    (114, 122, 7),
];

const BEES: [(u16, u16, u8, u8, u8); 3] = [
    (44, 120, 7, 0, 34),
    (92, 118, 9, 4, 26),
    (270, 92, 6, 2, 18),
];

const CHICKEN: (u16, u16, u8) = (322, 129, 14);

const CREAKING: (u16, u16) = (288, 98);
const WALKERS: [(&str, u16, u16, u8, u8, u8); 2] = [
    ("skeleton", 96, 112, 34, 16, 0),
    ("zombie", 40, 112, 30, 20, 9),
];
const PHANTOMS: [(u16, u16, u8); 2] = [(188, 92, 0), (262, 22, 7)];

const SNOW_GOLEM: (u16, u16, u8) = (100, 113, 40);

const PIG: (&str, u16, u16, u8) = ("pig", 96, 126, 40);
const FOX: (&str, u16, u16, u8) = ("fox", 96, 130, 40);
const SNOWFALL: [(u16, u16, i16, u16, u8, u8); 8] = [
    (250, 0, -14, 150, 12, 0),
    (272, 4, -10, 146, 10, 5),
    (296, 0, -16, 150, 13, 2),
    (318, 6, -12, 144, 11, 8),
    (340, 0, -18, 150, 14, 4),
    (262, 10, -8, 140, 9, 7),
    (308, 2, -14, 148, 12, 10),
    (332, 8, -10, 142, 10, 1),
];

const LEAVES: [(u16, u16, i16, u16, u8, u8); 7] = [
    (300, 72, -96, 74, 11, 0),
    (330, 64, -120, 82, 13, 4),
    (288, 80, -70, 66, 9, 7),
    (346, 76, -104, 70, 12, 2),
    (312, 58, -132, 88, 14, 9),
    (322, 82, -60, 64, 10, 5),
    (296, 66, -150, 80, 15, 12),
];

#[component]
fn Surface(season: Season) -> impl IntoView {
    let halloween = season == Season::Halloween;
    let snowy = matches!(season, Season::Christmas | Season::Winter);
    let spring = season == Season::Spring;
    let autumn = season == Season::Autumn;
    let frame = NodeRef::<html::Div>::new();
    parallax(frame);

    let leaves = LEAVES
        .iter()
        .chain(if snowy { &SNOWFALL[..] } else { &[] })
        .enumerate()
        .map(|(i, (x, y, dx, dy, secs, phase))| {
            let shape = i % 4;
            let style = format!(
                "--x:{x};--y:{y};--dx:{dx};--dy:{dy};--dur:{secs}s;--delay:-{phase}s;--v:{shape}"
            );
            view! { <span class="leaf" style=style></span> }
        })
        .collect_view();
    let smoke = (0..4u8)
        .map(|i| view! { <span class="smoke" style=format!("--delay:-{}s", f32::from(i) * 1.75)></span> })
        .collect_view();
    let fireflies = FIREFLIES
        .iter()
        .map(|(x, y, phase)| view! { <span class="firefly" style=format!("--x:{x};--y:{y};--delay:-{phase}s")></span> })
        .collect_view();
    let bees = BEES
        .iter()
        .map(|(x, y, secs, phase, range)| {
            view! {
                <span
                    class="bee"
                    style=format!("--x:{x};--y:{y};--dur:{secs}s;--delay:-{phase}s;--range:{range}")
                ></span>
            }
        })
        .collect_view();
    let (chicken_x, chicken_y, range) = CHICKEN;
    let (golem_x, golem_y, golem_range) = SNOW_GOLEM;
    let roamer = match season {
        Season::Spring => Some(PIG),
        Season::Autumn => Some(FOX),
        _ => None,
    }
    .map(|(mob, x, y, range)| {
        view! {
            <span
                class=format!("roamer {mob}")
                style=format!("--x:{x};--y:{y};--range:{range}")
            ></span>
        }
    });
    let mobs = halloween.then(|| {
        let (x, y) = CREAKING;
        view! {
            <span class="creaking night-mob" style=format!("--x:{x};--y:{y}")></span>
            {WALKERS
                .iter()
                .map(|(mob, x, y, range, secs, phase)| {
                    let style = format!(
                        "--x:{x};--y:{y};--range:{range};--dur:{secs}s;--delay:-{phase}s",
                    );
                    view! { <span class=format!("walker {mob} night-mob") style=style></span> }
                })
                .collect_view()}
            {PHANTOMS
                .iter()
                .map(|(x, y, phase)| {
                    view! {
                        <span
                            class="phantom night-mob"
                            style=format!("--x:{x};--y:{y};--delay:-{phase}s")
                        ></span>
                    }
                })
                .collect_view()}
        }
    });

    view! {
        <section
            id="inicio"
            class=format!("scene scene-surface season-{}", season.slug())
            aria-labelledby="trip-title"
        >
            <div class="surface-copy">
                <h1
                    id="trip-title"
                    class="font-bold font-pixel text-[clamp(2.25rem,4.1vw,3.6rem)] leading-[.95]"
                >
                    {move_tr!("trip-title")}
                </h1>
                <p class="mt-5 text-lg leading-relaxed max-w-[32rem]">{move_tr!("trip-lede")}</p>
                <div class="flex flex-wrap gap-3 mt-7">
                    <a href="#mods" class="btn btn-primary">
                        {move_tr!("trip-cta-mods")}
                        <ArrowDownIcon />
                    </a>
                    <a href="#contacto" class="btn trip-btn-ghost">
                        {move_tr!("trip-cta-contact")}
                    </a>
                </div>
                <a
                    href="https://modrinth.com/user/CrawKatt"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="inline-flex gap-3 items-baseline mt-6 min-h-[44px]"
                >
                    <span class="text-3xl font-bold font-pixel">"50k+"</span>
                    <span class="font-mono text-sm underline decoration-2 underline-offset-4">
                        {move_tr!("hero-proof")}
                    </span>
                </a>
            </div>
            <div class="scene-frame" node_ref=frame>
                <div class="stage" role="img" aria-label=move_tr!("trip-scene-plains")>
                    {scenery(false, season)}
                    {scenery(true, season)}
                    {smoke}
                    {(!halloween && !spring && !autumn)
                        .then(|| {
                            view! {
                                <span
                                    class="chicken"
                                    style=format!("--x:{chicken_x};--y:{chicken_y};--range:{range}")
                                ></span>
                            }
                        })}
                    {roamer}
                    {mobs}
                    {snowy
                        .then(|| {
                            view! {
                                <span
                                    class="snow-golem"
                                    style=format!(
                                        "--x:{golem_x};--y:{golem_y};--range:{golem_range}",
                                    )
                                ></span>
                            }
                        })}
                    <span class="player-shadow"></span>
                    <Player3d class="surface-player" />
                    {leaves}
                    {GUSTS
                        .iter()
                        .map(|(x, y, at)| {
                            view! {
                                <span
                                    class="gust"
                                    style=format!("--x:{x};--y:{y};--delay:{at}s")
                                ></span>
                            }
                        })
                        .collect_view()}
                    {fireflies}
                    {(!halloween && !snowy && !autumn).then_some(bees)}
                </div>
            </div>
        </section>
    }
}

#[component]
fn ModFrame(
    project: &'static Project,
    open: RwSignal<Option<&'static Project>>,
    dialog: NodeRef<html::Dialog>,
) -> impl IntoView {
    let open_it = move |_| {
        open.set(Some(project));
        show(dialog);
    };

    view! {
        <button
            type="button"
            class="mod-frame"
            aria-haspopup="dialog"
            aria-label=project.name
            on:click=open_it
        >
            {match project.icon {
                Some(icon) => {
                    view! { <img class="mod-item" src=icon alt="" loading="lazy" /> }.into_any()
                }
                None => {
                    view! { <span class="mod-item mod-initials">{project.initials()}</span> }
                        .into_any()
                }
            }}
            <span class="nametag" aria-hidden="true">
                {project.name}
            </span>
        </button>
    }
}

fn show(dialog: NodeRef<html::Dialog>) {
    if let Some(dialog) = dialog.get() {
        let _ = dialog.show_modal();
    }
}

fn close(dialog: NodeRef<html::Dialog>) {
    if let Some(dialog) = dialog.get() {
        dialog.close();
    }
}

#[component]
fn GameDialog(
    dialog: NodeRef<html::Dialog>,
    panel: &'static str,
    title_id: &'static str,
    on_close: Callback<()>,
    children: Children,
) -> impl IntoView {
    view! {
        <dialog
            node_ref=dialog
            class="mod-dialog"
            aria-labelledby=title_id
            on:close=move |_| on_close.run(())
            on:click=move |ev| {
                if ev.target() == ev.current_target() {
                    close(dialog);
                }
            }
        >
            <div class=panel>
                <button
                    type="button"
                    class="mod-dialog-close"
                    autofocus
                    aria-label=move_tr!("mod-close")
                    on:click=move |_| close(dialog)
                >
                    <CloseIcon />
                </button>
                {children()}
            </div>
        </dialog>
    }
}

#[component]
fn ModDialog(
    dialog: NodeRef<html::Dialog>,
    open: RwSignal<Option<&'static Project>>,
) -> impl IntoView {
    let i18n = expect_context::<I18n>();
    let body = move || {
        open.get().map(|project| {
            let Meta::Mod(downloads, versions) = project.meta;
            let meta = move || format!("{downloads} {} · Minecraft {versions}", i18n.tr("proj-downloads"));
            view! {
                <div class="flex flex-col gap-3 items-start sm:flex-row sm:gap-4 sm:items-center sm:pr-12">
                    <div class="mc-slot">
                        {match project.icon {
                            Some(icon) => view! { <img src=icon alt="" /> }.into_any(),
                            None => {
                                view! { <span class="mod-initials">{project.initials()}</span> }
                                    .into_any()
                            }
                        }}
                    </div>
                    <div class="min-w-0">
                        <h2
                            id="mod-dialog-title"
                            class="text-2xl font-bold leading-tight font-pixel sm:text-[1.75rem]"
                        >
                            {project.name}
                        </h2>
                        <p class="mt-1 font-mono text-[13px]">{meta}</p>
                    </div>
                </div>
                <p class="mt-5 leading-relaxed text-[1.0625rem]">{move || i18n.tr(project.desc)}</p>
                <ul class="flex flex-wrap gap-1.5 mt-4" aria-label=move_tr!("mod-tags")>
                    {project
                        .tags
                        .iter()
                        .map(|tag| view! { <li class="mc-tag">{*tag}</li> })
                        .collect_view()}
                </ul>
                {project
                    .video
                    .map(|src| {
                        view! {
                            <figure class="mt-5">
                                <video
                                    class="w-full bg-black"
                                    src=src
                                    controls
                                    preload="metadata"
                                    playsinline
                                ></video>
                                <figcaption class="mt-2 font-mono text-[13px]">
                                    {move_tr!("mod-video")}
                                </figcaption>
                            </figure>
                        }
                    })}
                <div class="flex flex-wrap gap-3 mt-6">
                    {project
                        .link
                        .map(|(label, url)| {
                            view! {
                                <a
                                    href=url
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="mc-button"
                                >
                                    {label}
                                    <ExternalIcon class="w-2.5 h-2.5" />
                                </a>
                            }
                        })}
                    {project
                        .source
                        .map(|url| {
                            view! {
                                <a
                                    href=url
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="mc-button"
                                >
                                    {move_tr!("proj-source")}
                                    <ExternalIcon class="w-2.5 h-2.5" />
                                </a>
                            }
                        })}
                </div>
            }
        })
    };

    view! {
        <GameDialog
            dialog
            panel="mc-panel"
            title_id="mod-dialog-title"
            on_close=Callback::new(move |_| open.set(None))
        >
            {body}
        </GameDialog>
    }
}

const LUSH_SPORES: [(u16, u16, i8, i8, u8, f32); 24] = [
    (8, 36, -8, -6, 10, 0.0),
    (69, 73, 15, -11, 13, 2.7),
    (130, 110, -9, 16, 16, 5.4),
    (191, 147, 16, 10, 12, 8.1),
    (252, 44, -10, -15, 15, 10.8),
    (313, 81, 17, -9, 11, 2.5),
    (38, 118, -11, 14, 14, 2.2),
    (99, 155, 18, 8, 10, 8.9),
    (160, 52, -12, -13, 13, 8.6),
    (221, 89, 19, -7, 16, 8.3),
    (282, 126, -13, 12, 12, 3.0),
    (343, 163, 20, 6, 15, 14.7),
    (68, 60, -14, -11, 11, 10.4),
    (129, 97, 8, -16, 14, 7.1),
    (190, 134, -15, 10, 10, 7.8),
    (251, 171, 9, 15, 13, 1.5),
    (312, 68, -16, -9, 16, 11.2),
    (37, 105, 10, -14, 12, 9.9),
    (98, 142, -17, 8, 15, 3.6),
    (159, 39, 11, 13, 11, 7.3),
    (220, 76, -18, -7, 14, 12.0),
    (281, 113, 12, -12, 10, 6.7),
    (342, 150, -19, 6, 13, 7.4),
    (67, 47, 13, 11, 16, 14.1),
];

const BATS: [(u16, u16, u8); 3] = [(24, 76, 0), (60, 104, 3), (272, 84, 5)];

const DRIPS: [(u16, u16, u16, f32); 2] = [(52, 91, 92, 0.0), (292, 91, 92, 2.6)];

#[component]
fn Cave(season: Season) -> impl IntoView {
    let frame = NodeRef::<html::Div>::new();
    parallax(frame);
    let dialog = NodeRef::<html::Dialog>::new();
    let open = RwSignal::new(None::<&'static Project>);
    let icy = matches!(season, Season::Christmas | Season::Winter);
    let dripstone = !icy && season != Season::Spring;
    let cave_layer = move |name| image(name, season.cave_art(&format!("cave-{name}")), "276");

    view! {
        <section id="mods" class="scene scene-cave" aria-labelledby="mods-title">
            <div class="scene-frame" node_ref=frame>
                <div class="stage">
                    {cave_layer("far")} {cave_layer("mid")}
                    {(!icy)
                        .then(|| view! { <span class="waterfall" style="--x:322;--y:0"></span> })}
                    {cave_layer("main")} {cave_layer("light")}
                    {dripstone
                        .then(|| {
                            DRIPS
                                .iter()
                                .map(|(x, y, fall, phase)| {
                                    let style = format!(
                                        "--x:{x};--y:{y};--fall:{fall};--delay:-{phase}s",
                                    );
                                    view! { <span class="drip" style=style></span> }
                                })
                                .collect_view()
                        })}
                    {(season == Season::Halloween)
                        .then(|| {
                            BATS.iter()
                                .map(|(x, y, phase)| {
                                    view! {
                                        <span
                                            class="bat"
                                            style=format!("--x:{x};--y:{y};--delay:-{phase}s")
                                        ></span>
                                    }
                                })
                                .collect_view()
                        })}
                    {(season == Season::Spring)
                        .then(|| {
                            LUSH_SPORES
                                .iter()
                                .map(|(x, y, dx, dy, secs, phase)| {
                                    let style = format!(
                                        "--x:{x};--y:{y};--dx:{dx};--dy:{dy};--dur:{secs}s;--delay:-{phase}s",
                                    );
                                    view! { <span class="lush-spore" style=style></span> }
                                })
                                .collect_view()
                        })} <span class="minecart"></span> <div class="cave-copy">
                        <h2
                            id="mods-title"
                            class="font-bold leading-none font-pixel text-[clamp(1.875rem,3.4vw,3rem)]"
                        >
                            {move_tr!("cave-title")}
                        </h2>
                        <p class="mx-auto mt-3 leading-relaxed max-w-[34rem]">
                            {move_tr!("cave-lede")}
                        </p>
                    </div>
                    <ul class="frames">
                        {MODS
                            .iter()
                            .map(|project| {
                                view! {
                                    <li>
                                        <ModFrame project open dialog />
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ul>
                </div>
            </div>
            <ModDialog dialog open />
        </section>
    }
}

fn strata() -> impl IntoView {
    view! {
        <div class="strata" aria-hidden="true">
            <div class="strata-stage">
                <img class="layer" src="/public/world/strata.png" alt="" width="352" height="128" />
                <img
                    class="layer strata-night"
                    src="/public/world/strata-night.png"
                    alt=""
                    width="352"
                    height="128"
                />
            </div>
        </div>
    }
}

fn deep_strata() -> impl IntoView {
    view! {
        <div class="strata strata-deep" aria-hidden="true">
            <div class="strata-stage">
                <img class="layer" src="/public/world/deep.png" alt="" width="352" height="96" />
            </div>
        </div>
    }
}

fn hit(
    name: impl Into<Signal<String>>,
    (x, y, w, h): (u16, u16, u16, u16),
    on_open: impl Fn() + 'static,
) -> impl IntoView {
    let name = name.into();
    view! {
        <button
            type="button"
            class="hit"
            style=format!("--x:{x};--y:{y};--w:{w};--h:{h}")
            aria-haspopup="dialog"
            aria-label=name
            on:click=move |_| on_open()
        >
            <span class="nametag" aria-hidden="true">
                {name}
            </span>
        </button>
    }
}

const BOOKSHELVES: [(u16, u16); 6] = [
    (120, 144),
    (120, 160),
    (120, 176),
    (216, 144),
    (216, 160),
    (216, 176),
];
const ENCHANTING: (u16, u16) = (168, 160);

const GLYPHS: [(u16, u16, i16, i16, f32); 4] = [
    (136, 148, 34, 14, 0.0),
    (136, 172, 36, -4, 1.6),
    (214, 152, -36, 10, 0.8),
    (214, 174, -34, -6, 2.4),
];

#[component]
fn Deepslate() -> impl IntoView {
    let frame = NodeRef::<html::Div>::new();
    parallax(frame);
    let dialog = NodeRef::<html::Dialog>::new();
    let open = RwSignal::new(None::<&'static Contribution>);

    view! {
        <section id="contribuciones" class="scene scene-deep" aria-labelledby="deep-title">
            <div class="scene-copy">
                <h2
                    id="deep-title"
                    class="font-bold leading-none font-pixel text-[clamp(1.875rem,3.4vw,3rem)]"
                >
                    {move_tr!("oss-title")}
                </h2>
                <p class="mx-auto mt-3 leading-relaxed max-w-[34rem]">
                    {move_tr!("oss-intro")} " " {move_tr!("deep-hint")}
                </p>
            </div>
            <div class="scene-frame" node_ref=frame>
                <div class="stage">
                    {layers("deepslate", &["far"])}
                    {[20u16, 324]
                        .iter()
                        .map(|x| view! { <span class="lava-far" style=format!("--x:{x}")></span> })
                        .collect_view()} {layers("deepslate", &["mid", "main", "light"])}
                    {CONTRIBUTIONS
                        .iter()
                        .zip(BOOKSHELVES)
                        .map(|(c, (x, y))| hit(
                            c.title,
                            (x, y, 16, 16),
                            move || {
                                open.set(Some(c));
                                show(dialog);
                            },
                        ))
                        .collect_view()}
                    <span
                        class="enchanting-book"
                        style=format!("--x:{};--y:{}", ENCHANTING.0 + 2, ENCHANTING.1 + 2)
                    ></span>
                    {(0..3u8)
                        .map(|i| view! { <span class="shriek" style=format!("--i:{i}")></span> })
                        .collect_view()}
                    {[(25u16, 112u16), (313, 112)]
                        .iter()
                        .flat_map(|&(x, y)| (0..2u8).map(move |i| (x, y, i)))
                        .map(|(x, y, i)| {
                            view! {
                                <span
                                    class="shriek shriek-echo"
                                    style=format!("--x:{x};--y:{y};--i:{i}")
                                ></span>
                            }
                        })
                        .collect_view()}
                    <span class="lavafall" style="--x:104;--y:32;--to:112"></span>
                    <span class="warden-peek">
                        <span></span>
                    </span>
                    {GLYPHS
                        .iter()
                        .map(|(x, y, dx, dy, phase)| {
                            view! {
                                <span
                                    class="mote glyph"
                                    style=format!(
                                        "--x:{x};--y:{y};--dx:{dx};--dy:{dy};--delay:-{phase}s",
                                    )
                                ></span>
                            }
                        })
                        .collect_view()}
                    {hit(
                        FEATURED.title,
                        (ENCHANTING.0, ENCHANTING.1, 16, 32),
                        move || {
                            open.set(Some(&FEATURED));
                            show(dialog);
                        },
                    )}
                </div>
            </div>
            <BookDialog dialog open />
        </section>
    }
}

#[component]
fn BookDialog(
    dialog: NodeRef<html::Dialog>,
    open: RwSignal<Option<&'static Contribution>>,
) -> impl IntoView {
    let i18n = expect_context::<I18n>();
    let body = move || {
        open.get().map(|c| {
            view! {
                {(c.repo != c.title)
                    .then(|| view! { <p class="font-mono text-[13px]">{c.repo}</p> })}
                <h2 id="book-title" class="mt-1 text-2xl font-bold leading-tight font-pixel">
                    {c.title}
                </h2>
                <p class="mt-1 font-pixel text-[15px]">
                    {move || i18n.tr(c.role)}
                    {c
                        .downloads
                        .map(|d| {
                            view! {
                                " · "
                                {d}
                                " "
                                {move_tr!("oss-downloads")}
                            }
                        })}
                </p>
                <p class="mt-4 leading-relaxed">{move || i18n.tr(c.description)}</p>
                {c
                    .poster
                    .map(|src| {
                        view! {
                            <img
                                class="book-poster"
                                src=src
                                alt=c.title
                                width="400"
                                loading="lazy"
                            />
                        }
                    })}
                {c
                    .links
                    .map(|(project, repo)| {
                        view! {
                            <p class="flex flex-wrap gap-y-2 gap-x-5 mt-5">
                                <a
                                    href=project
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="book-link"
                                >
                                    {move_tr!("oss-view-project")}
                                </a>
                                <a
                                    href=repo
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class="book-link"
                                >
                                    {move_tr!("oss-repo")}
                                </a>
                            </p>
                        }
                    })}
            }
        })
    };

    view! {
        <GameDialog
            dialog
            panel="book-panel"
            title_id="book-title"
            on_close=Callback::new(move |_| open.set(None))
        >
            <div class="book-page">{body}</div>
        </GameDialog>
    }
}

fn bedrock(to: &'static str, rows: u16) -> impl IntoView {
    view! {
        <div
            class=format!("strata strata-bedrock strata-{to}")
            style=format!("--rows:{rows}")
            aria-hidden="true"
        >
            <div class="strata-stage">
                <img
                    class="layer"
                    src=format!("/public/world/bedrock-{to}.png")
                    alt=""
                    width="352"
                    height=rows
                />
            </div>
        </div>
    }
}

struct Plan {
    name: &'static str,
    price: &'static str,
    summary: &'static str,
    includes: &'static [&'static str],
}

const PLANS: [Plan; 3] = [
    Plan {
        name: "plan-quartz",
        price: "plan-quartz-price",
        summary: "plan-quartz-summary",
        includes: &[
            "plan-quartz-1",
            "plan-quartz-2",
            "plan-quartz-3",
            "plan-quartz-4",
        ],
    },
    Plan {
        name: "plan-gold",
        price: "plan-gold-price",
        summary: "plan-gold-summary",
        includes: &["plan-gold-1", "plan-gold-2", "plan-gold-3", "plan-gold-4"],
    },
    Plan {
        name: "plan-netherite",
        price: "plan-netherite-price",
        summary: "plan-netherite-summary",
        includes: &[
            "plan-netherite-1",
            "plan-netherite-2",
            "plan-netherite-3",
            "plan-netherite-4",
        ],
    },
];

const CHESTS: [(u16, u16); 3] = [(129, 161), (169, 161), (209, 161)];
const LAVAFALLS: [(u16, u16); 2] = [(32, 64), (304, 64)];

const GUARD: (u16, u16, u16) = (85, 144, 128);

const SPORES: [(u16, u16, f32); 8] = [
    (60, 188, 0.0),
    (110, 168, 1.7),
    (150, 183, 3.1),
    (200, 176, 4.4),
    (236, 188, 2.2),
    (270, 166, 5.0),
    (300, 180, 0.9),
    (330, 172, 3.7),
];
const SPARKS: [(u16, u16, f32); 4] = [
    (58, 128, 0.0),
    (58, 128, 2.1),
    (294, 128, 1.0),
    (294, 128, 3.2),
];

fn motes(kind: &'static str, at: &'static [(u16, u16, f32)]) -> impl IntoView {
    at.iter()
        .map(|(x, y, phase)| {
            view! {
                <span
                    class=format!("mote {kind}")
                    style=format!("--x:{x};--y:{y};--delay:-{phase}s")
                ></span>
            }
        })
        .collect_view()
}

#[component]
fn Nether() -> impl IntoView {
    let frame = NodeRef::<html::Div>::new();
    parallax(frame);
    let dialog = NodeRef::<html::Dialog>::new();
    let open = RwSignal::new(None::<&'static Plan>);
    let i18n = expect_context::<I18n>();

    view! {
        <section id="comisiones" class="scene scene-nether" aria-labelledby="nether-title">
            <div class="scene-copy">
                <h2
                    id="nether-title"
                    class="font-bold leading-none font-pixel text-[clamp(1.875rem,3.4vw,3rem)]"
                >
                    {move_tr!("nether-title")}
                </h2>
                <p class="mx-auto mt-3 leading-relaxed max-w-[34rem]">{move_tr!("nether-lede")}</p>
            </div>
            <div class="scene-frame" node_ref=frame>
                <div class="stage">
                    {layers("nether", &["far", "mid"])}
                    <span class="ghast" style="--gx:58;--gy:70"></span>
                    <span
                        class="ghast"
                        style="--gx:236;--gy:82;animation-delay:-16s,-16s,0s"
                    ></span> {layers("nether", &["main", "light"])}
                    <span
                        class="guard"
                        style=format!("--x:{};--y:{};--range:{}", GUARD.0, GUARD.1, GUARD.2)
                    ></span>
                    {LAVAFALLS
                        .iter()
                        .map(|(x, y)| {
                            view! {
                                <span class="lavafall" style=format!("--x:{x};--y:{y}")></span>
                            }
                        })
                        .collect_view()} {motes("spore", &SPORES)}
                    {CHESTS
                        .iter()
                        .zip(&PLANS)
                        .map(|(&(x, y), plan)| {
                            let name = Signal::derive(move || i18n.tr(plan.name));
                            hit(
                                name,
                                (x - 1, y - 1, 16, 16),
                                move || {
                                    open.set(Some(plan));
                                    show(dialog);
                                },
                            )
                        })
                        .collect_view()}
                </div>
            </div>
            <ChestDialog dialog open />
        </section>
    }
}

#[component]
fn ChestDialog(
    dialog: NodeRef<html::Dialog>,
    open: RwSignal<Option<&'static Plan>>,
) -> impl IntoView {
    let i18n = expect_context::<I18n>();
    let form = expect_context::<ContactDialog>().0;
    let body = move || {
        open.get().map(|plan| {
            view! {
                <h2
                    id="chest-title"
                    class="text-2xl font-bold leading-tight font-pixel sm:text-[1.75rem]"
                >
                    {move || i18n.tr(plan.name)}
                </h2>
                <p class="mt-2 font-mono text-[13px]">{move || i18n.tr(plan.price)}</p>
                <p class="mt-4 leading-relaxed text-[1.0625rem]">{move || i18n.tr(plan.summary)}</p>
                <ul class="pl-5 mt-3 space-y-1 leading-relaxed list-disc">
                    {plan
                        .includes
                        .iter()
                        .map(|key| view! { <li>{move || i18n.tr(key)}</li> })
                        .collect_view()}
                </ul>
                <p class="mt-4 font-mono text-[13px]">{move_tr!("plan-versions")}</p>
                <button
                    type="button"
                    class="mt-6 mc-button"
                    on:click=move |_| {
                        close(dialog);
                        show(form);
                    }
                >
                    {move_tr!("plan-cta")}
                </button>
            }
        })
    };

    view! {
        <GameDialog
            dialog
            panel="mc-panel"
            title_id="chest-title"
            on_close=Callback::new(move |_| open.set(None))
        >
            {body}
        </GameDialog>
    }
}

#[derive(Clone, Copy)]
struct ContactDialog(NodeRef<html::Dialog>);

const DRAGON_EGG: (u16, u16) = (168, 112);

#[component]
fn End() -> impl IntoView {
    let frame = NodeRef::<html::Div>::new();
    parallax(frame);
    let form = expect_context::<ContactDialog>().0;
    let i18n = expect_context::<I18n>();
    let channel = "flex min-h-[44px] items-center gap-3 font-semibold";

    view! {
        <section id="contacto" class="scene scene-end" aria-labelledby="end-title">
            <div class="scene-copy">
                <h2
                    id="end-title"
                    class="font-bold leading-none font-pixel text-[clamp(1.875rem,3.4vw,3rem)]"
                >
                    {move_tr!("contact-title")}
                </h2>
                <p class="mx-auto mt-3 leading-relaxed max-w-[34rem]">{move_tr!("contact-text")}</p>
            </div>
            <div class="scene-frame" node_ref=frame>
                <div class="stage">
                    <div role="img" aria-label=move_tr!("end-scene")>
                        {layers("end", &["far", "main", "light"])}
                    </div>
                    {motes("spark", &SPARKS)}
                    {hit(
                        Signal::derive(move || i18n.tr("contact-open")),
                        (DRAGON_EGG.0, DRAGON_EGG.1, 16, 16),
                        move || show(form),
                    )}
                </div>
            </div>
            <GameDialog
                dialog=form
                panel="mc-panel egg-panel"
                title_id="form-title"
                on_close=Callback::new(|_| ())
            >
                <h2 id="form-title" class="pr-12 mb-5 text-2xl font-bold leading-tight font-pixel">
                    {move_tr!("contact-title")}
                </h2>
                <ContactForm />
                <div class="pt-6 mt-8 egg-more">
                    <div>
                        <p class="text-sm font-pixel">{move_tr!("contact-elsewhere")}</p>
                        <ul class="grid gap-1 mt-2">
                            <li>
                                <a
                                    href="https://www.discord.com/users/395631548629516298"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class=channel
                                >
                                    "Discord"
                                </a>
                            </li>
                            <li>
                                <a href=format!("mailto:{EMAIL}") class=channel>
                                    "Email"
                                    <span class="font-mono text-sm font-normal">{EMAIL}</span>
                                </a>
                            </li>
                            <li>
                                <a
                                    href="https://github.com/CrawKatt"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class=channel
                                >
                                    "GitHub"
                                    <span class="font-mono text-sm font-normal">"@CrawKatt"</span>
                                </a>
                            </li>
                            <li>
                                <a
                                    href="https://x.com/CrawKatt"
                                    target="_blank"
                                    rel="noopener noreferrer"
                                    class=channel
                                >
                                    "X"
                                    <span class="font-mono text-sm font-normal">"@CrawKatt"</span>
                                </a>
                            </li>
                        </ul>
                    </div>
                </div>
                <footer class="flex flex-wrap gap-y-1 gap-x-6 items-center pt-3 mt-6 font-mono egg-more text-[13px]">
                    <span>{move_tr!("foot-rights")}</span>
                    <span>{move_tr!("foot-made")}</span>
                </footer>
            </GameDialog>
        </section>
    }
}

#[component]
pub fn TripBar() -> impl IntoView {
    view! {
        <header class="trip-bar">
            <a href="/" class="inline-flex items-center font-bold tracking-tight min-h-[44px]">
                "CrawKatt"
            </a>
            <div class="flex gap-3 items-center">
                <ThemeToggle />
                <LanguageSelector />
            </div>
        </header>
    }
}

#[component]
pub fn Journey() -> impl IntoView {
    let query = use_query_map();
    let season = Resource::new_blocking(move || query.read().get("theme"), current_season);
    provide_context(ContactDialog(NodeRef::new()));
    view! {
        <TripBar />
        <main id="main">
            <Suspense>
                {move || Suspend::new(async move {
                    let season = season.await.unwrap_or_default();
                    view! {
                        <Surface season />
                        {strata()}
                        <Cave season />
                    }
                })}
            </Suspense>
            {deep_strata()}
            <Deepslate />
            {bedrock("nether", 156)}
            <Nether />
            {bedrock("end", 92)}
            <End />
        </main>
    }
}
