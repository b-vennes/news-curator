use chrono::{DateTime, NaiveDateTime};

use crate::types;

impl types::state::Source {
    pub fn load(config_source: types::config::Source) -> Result<types::state::Source, String> {
        match config_source.s_type {
            types::config::SourceType::RSS => Self::load_rss(
                config_source.link,
                config_source.title,
                config_source.category,
            ),
            types::config::SourceType::Atom => Self::load_atom(
                config_source.link,
                config_source.title,
                config_source.category,
            ),
        }
    }

    pub fn load_rss(
        feed_link: types::Link,
        feed_title: types::Title,
        category: types::Title,
    ) -> Result<types::state::Source, String> {
        let link_content = reqwest::blocking::get(feed_link.clone())
            .and_then(|r| r.bytes())
            .map_err(|e| e.to_string())?;

        let feed = rss::Channel::read_from(&link_content[..]).map_err(|e| e.to_string())?;

        let items = feed
            .items()
            .iter()
            .map(|i| types::state::Item {
                title: i.title().map(|s| s.to_string()).unwrap_or(String::from("")),
                link: i.link().map(|s| s.to_string()).unwrap_or(String::from("")),
                published_at: crate::result_ops::traverse_option::<NaiveDateTime, String>(
                    i.pub_date().map(|p| {
                        DateTime::parse_from_rfc2822(p)
                            .map(|d| d.naive_utc())
                            .map_err(|e| e.to_string())
                    }),
                )
                .unwrap_or(None),
            })
            .collect::<Vec<_>>();

        Ok(types::state::Source {
            title: feed_title,
            items,
            category: types::state::Category { title: category },
            link: feed_link,
        })
    }

    pub fn load_atom(
        feed_link: String,
        feed_title: String,
        category_name: String,
    ) -> Result<types::state::Source, String> {
        let link_content = reqwest::blocking::get(feed_link.clone())
            .and_then(|r| r.bytes())
            .map_err(|e| e.to_string())?;

        let feed =
            atom_syndication::Feed::read_from(&link_content[..]).map_err(|e| e.to_string())?;

        let items = feed
            .entries()
            .iter()
            .map(|e| types::state::Item {
                title: e.title().clone().value,
                link: e
                    .links
                    .get(0)
                    .map(|l| l.href.to_string())
                    .unwrap_or(String::from("")),
                published_at: e.published().map(|p| p.naive_utc()),
            })
            .collect::<Vec<_>>();

        Ok(types::state::Source {
            title: feed_title,
            items,
            category: types::state::Category {
                title: category_name,
            },
            link: feed_link,
        })
    }
}
