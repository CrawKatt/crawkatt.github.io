pub struct Contribution {
    pub title: &'static str,
    pub repo: &'static str,
    pub description: &'static str,
    pub role: &'static str,
    pub downloads: Option<&'static str>,
    pub links: Option<(&'static str, &'static str)>,
    pub poster: Option<&'static str>,
}

const fn crown(
    title: &'static str,
    description: &'static str,
    role: &'static str,
    poster: Option<&'static str>,
) -> Contribution {
    Contribution {
        title,
        repo: "Crown Studio",
        description,
        role,
        downloads: None,
        links: None,
        poster,
    }
}

pub static CONTRIBUTIONS: [Contribution; 6] = [
    Contribution {
        title: "The Immersive Music Mod",
        repo: "sf-inc/timm",
        description: "oss-immersive",
        role: "oss-team",
        downloads: Some("670k+"),
        links: Some((
            "https://modrinth.com/mod/immersivemusicmod",
            "https://github.com/sf-inc/timm",
        )),
        poster: None,
    },
    crown(
        "Operation Springfield",
        "crown-springfield",
        "crown-event",
        Some("/public/crown/operation-springfield.webp"),
    ),
    crown(
        "Road to the Crown",
        "crown-road",
        "crown-event",
        Some("/public/crown/road-to-the-crown.webp"),
    ),
    crown("CrownCore", "crown-core", "crown-mod", None),
    crown(
        "Squid Crown Games",
        "crown-squid",
        "crown-event",
        Some("/public/crown/squid-crown-games.webp"),
    ),
    crown(
        "La Caja Navideña",
        "crown-xmas",
        "crown-event",
        Some("/public/crown/la-caja-navidena.webp"),
    ),
];

pub static FEATURED: Contribution = crown("Crown Studio", "crown-studio", "crown-dev", None);
