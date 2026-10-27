use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Season {
    #[default]
    Summer,
    Autumn,
    Winter,
    Spring,
    Halloween,
    Christmas,
}

#[cfg(feature = "ssr")]
const UTC_OFFSET_HOURS: i64 = -3;

impl Season {
    pub fn slug(self) -> &'static str {
        match self {
            Self::Summer => "summer",
            Self::Autumn => "autumn",
            Self::Winter => "winter",
            Self::Spring => "spring",
            Self::Halloween => "halloween",
            Self::Christmas => "christmas",
        }
    }

    fn folder(self) -> Option<&'static str> {
        match self {
            Self::Halloween => Some("halloween"),
            Self::Christmas => Some("christmas"),
            Self::Spring => Some("spring"),
            Self::Autumn => Some("autumn"),
            Self::Winter => Some("winter"),
            _ => None,
        }
    }

    pub fn art(self, file: &str) -> String {
        match self.folder() {
            Some(folder) => format!("/public/world/{folder}/{file}.png"),
            None => format!("/public/world/{file}.png"),
        }
    }

    pub fn cave_art(self, file: &str) -> String {
        match self {
            Self::Christmas | Self::Winter => Self::Christmas.art(file),
            Self::Halloween | Self::Spring => self.art(file),
            _ => format!("/public/world/{file}.png"),
        }
    }
}

#[cfg(feature = "ssr")]
impl Season {
    pub fn for_month(month: u8) -> Self {
        match month {
            10 => Self::Halloween,
            12 => Self::Christmas,
            1..=3 => Self::Summer,
            4..=6 => Self::Autumn,
            7 | 8 => Self::Winter,
            _ => Self::Spring,
        }
    }

    pub fn from_slug(slug: &str) -> Option<Self> {
        [
            Self::Summer,
            Self::Autumn,
            Self::Winter,
            Self::Spring,
            Self::Halloween,
            Self::Christmas,
        ]
        .into_iter()
        .find(|season| season.slug() == slug)
    }
}

#[cfg(feature = "ssr")]
fn month_of_day(days: i64) -> u8 {
    let z = days + 719_468;
    let doe = z - z.div_euclid(146_097) * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let mp = (5 * (doe - (365 * yoe + yoe / 4 - yoe / 100)) + 2) / 153;
    (if mp < 10 { mp + 3 } else { mp - 9 }) as u8
}

#[server(CurrentSeason, "/api")]
pub async fn current_season(chosen: Option<String>) -> Result<Season, ServerFnError> {
    if let Some(season) = chosen.as_deref().and_then(Season::from_slug) {
        return Ok(season);
    }
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |since| since.as_secs() as i64);
    Ok(Season::for_month(month_of_day(
        (secs + UTC_OFFSET_HOURS * 3600).div_euclid(86_400),
    )))
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::*;

    #[test]
    fn months_and_calendar() {
        let months: Vec<u8> = [0, 11_017, 20_752, 20_818, 20_147]
            .into_iter()
            .map(month_of_day)
            .collect();
        assert_eq!(months, [1, 3, 10, 12, 2]);
        assert_eq!(Season::for_month(10), Season::Halloween);
        assert_eq!(Season::for_month(12), Season::Christmas);
        assert_eq!(Season::for_month(2), Season::Summer);
        assert_eq!(Season::for_month(5), Season::Autumn);
        assert_eq!(Season::for_month(8), Season::Winter);
        assert_eq!(Season::for_month(9), Season::Spring);
        assert_eq!(Season::for_month(11), Season::Spring);
        assert_eq!(Season::from_slug("halloween"), Some(Season::Halloween));
        assert_eq!(
            Season::Halloween.art("surface-main"),
            "/public/world/halloween/surface-main.png"
        );
        assert_eq!(
            Season::Summer.art("surface-main"),
            "/public/world/surface-main.png"
        );
        assert_eq!(
            Season::Christmas.cave_art("cave-main"),
            "/public/world/christmas/cave-main.png"
        );
        assert_eq!(
            Season::Spring.cave_art("cave-main"),
            "/public/world/spring/cave-main.png"
        );
        assert_eq!(
            Season::Autumn.cave_art("cave-main"),
            "/public/world/cave-main.png"
        );
        assert_eq!(
            Season::Winter.cave_art("cave-main"),
            "/public/world/christmas/cave-main.png"
        );
        assert_eq!(
            Season::Halloween.cave_art("cave-main"),
            "/public/world/halloween/cave-main.png"
        );
    }
}
