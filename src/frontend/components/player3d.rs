use leptos::prelude::*;

type Faces = [(u8, u8); 6];

struct Part {
    class: &'static str,
    size: (u8, u8, u8),
    at: (u8, u8),
    base: Faces,
    outer: Faces,
}

const fn limb(u: u8, v: u8) -> Faces {
    [
        (u + 4, v + 4),
        (u + 12, v + 4),
        (u, v + 4),
        (u + 8, v + 4),
        (u + 4, v),
        (u + 8, v),
    ]
}

const PARTS: [Part; 6] = [
    Part {
        class: "mc-arm mc-right",
        size: (4, 12, 4),
        at: (0, 8),
        base: limb(40, 16),
        outer: limb(40, 32),
    },
    Part {
        class: "mc-leg mc-right",
        size: (4, 12, 4),
        at: (4, 20),
        base: limb(0, 16),
        outer: limb(0, 32),
    },
    Part {
        class: "mc-body",
        size: (8, 12, 4),
        at: (4, 8),
        base: [(20, 20), (32, 20), (16, 20), (28, 20), (20, 16), (28, 16)],
        outer: [(20, 36), (32, 36), (16, 36), (28, 36), (20, 32), (28, 32)],
    },
    Part {
        class: "mc-leg mc-left",
        size: (4, 12, 4),
        at: (8, 20),
        base: limb(16, 48),
        outer: limb(0, 48),
    },
    Part {
        class: "mc-head",
        size: (8, 8, 8),
        at: (4, 0),
        base: [(8, 8), (24, 8), (0, 8), (16, 8), (8, 0), (16, 0)],
        outer: [(40, 8), (56, 8), (32, 8), (48, 8), (40, 0), (48, 0)],
    },
    Part {
        class: "mc-arm mc-left",
        size: (4, 12, 4),
        at: (12, 8),
        base: limb(32, 48),
        outer: limb(48, 48),
    },
];

const FACE_CLASSES: [&str; 6] = [
    "mc-front",
    "mc-back",
    "mc-right-side",
    "mc-left-side",
    "mc-top",
    "mc-bottom",
];

fn faces(size: (u8, u8, u8), uv: Faces) -> impl IntoView {
    let (w, h, d) = size;
    let dims = [(w, h), (w, h), (d, h), (d, h), (w, d), (w, d)];
    FACE_CLASSES
        .iter()
        .zip(dims)
        .zip(uv)
        .map(|((class, (fw, fh)), (u, v))| {
            view! {
                <div
                    class=format!("mc-face {class}")
                    style=format!("--fw:{fw};--fh:{fh};--u:{u};--v:{v}")
                ></div>
            }
        })
        .collect::<Vec<_>>()
}

#[component]
pub fn Player3d(#[prop(optional)] class: &'static str) -> impl IntoView {
    view! {
        <div class=format!("mc {class}") aria-hidden="true">
            <div class="mc-rig">
                {PARTS
                    .iter()
                    .map(|p| {
                        let (w, h, d) = p.size;
                        let style = format!(
                            "--w:{w};--h:{h};--d:{d};--x:{};--y:{}",
                            p.at.0,
                            p.at.1,
                        );
                        view! {
                            <div class=format!("mc-part {}", p.class) style=style>
                                <div class="mc-box">{faces(p.size, p.base)}</div>
                                <div class="mc-box mc-outer">{faces(p.size, p.outer)}</div>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_face_stays_inside_the_skin() {
        for part in &PARTS {
            let (w, h, d) = part.size;
            let dims = [(w, h), (w, h), (d, h), (d, h), (w, d), (w, d)];
            for uv in [part.base, part.outer] {
                for ((u, v), (fw, fh)) in uv.into_iter().zip(dims) {
                    assert!(
                        u + fw <= 64 && v + fh <= 64,
                        "{} face at ({u}, {v}) leaves the skin",
                        part.class
                    );
                }
            }
        }
    }
}
