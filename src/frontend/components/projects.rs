pub enum Meta {
    Mod(&'static str, &'static str),
}

pub struct Project {
    pub name: &'static str,
    pub icon: Option<&'static str>,
    pub desc: &'static str,
    pub meta: Meta,
    pub tags: &'static [&'static str],
    pub link: Option<(&'static str, &'static str)>,
    pub source: Option<&'static str>,
    pub video: Option<&'static str>,
}

impl Project {
    pub fn initials(&self) -> String {
        self.name
            .split_whitespace()
            .take(2)
            .filter_map(|word| word.chars().next())
            .collect()
    }
}

pub static MODS: [Project; 5] = [
    Project {
        name: "Subtitle+",
        icon: Some("/public/subtitle-icon.png"),
        desc: "proj-subtitle",
        meta: Meta::Mod("23k+", "1.20.5-1.21.1"),
        tags: &["Fabric", "Forge"],
        link: Some(("Modrinth", "https://modrinth.com/mod/subtitle")),
        source: None,
        video: None,
    },
    Project {
        name: "Superb Warfare ReFabricated",
        icon: Some("/public/superb_warfare.png"),
        desc: "proj-warfare",
        meta: Meta::Mod("23k+", "1.20.1, 1.21.1"),
        tags: &["Fabric", "Kotlin"],
        link: Some((
            "Modrinth",
            "https://modrinth.com/mod/superb-warfare-refabricated",
        )),
        source: Some("https://github.com/CrawKatt/SuperbWarfareReFabricated"),
        video: None,
    },
    Project {
        name: "Simplest Hammers Refabricated",
        icon: Some("/public/simplest-hammers.webp"),
        desc: "proj-hammers",
        meta: Meta::Mod("2.7k+", "1.21.1-1.21.3"),
        tags: &["Fabric", "Quilt", "Java"],
        link: Some((
            "Modrinth",
            "https://modrinth.com/mod/simplest-hammers-refabricated",
        )),
        source: Some("https://github.com/CrawKatt/Simplest-Hammers"),
        video: None,
    },
    Project {
        name: "[SBW] Dedsafío Starships",
        icon: Some("/public/dedsafio-starships.webp"),
        desc: "proj-starships",
        meta: Meta::Mod("40+", "1.21.1"),
        tags: &["Fabric"],
        link: Some((
            "CurseForge",
            "https://www.curseforge.com/minecraft/mc-mods/sbw-dedsafio-starships",
        )),
        source: None,
        video: None,
    },
    Project {
        name: "Conditional Barriers",
        icon: None,
        desc: "proj-barriers",
        meta: Meta::Mod("1.4k+", "1.21.1"),
        tags: &["Fabric", "Java"],
        link: Some(("Modrinth", "https://modrinth.com/mod/conditional-barriers")),
        source: Some("https://github.com/CrawKatt/ConditionalBarriers"),
        video: None,
    },
];
