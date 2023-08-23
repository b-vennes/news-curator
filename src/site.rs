use crate::types;
use tera::{Context, Tera};

impl types::site::Item {
    pub fn new(
        title: String,
        link: String,
        source: String,
        source_id: String,
        published_at: String,
        timestamp: i64,
    ) -> types::site::Item {
        types::site::Item {
            title,
            link,
            source,
            source_id,
            published_at,
            timestamp,
        }
    }

    pub fn from_state_item(item: types::state::Item, source: types::Title) -> types::site::Item {
        Self::new(
            item.title,
            item.link,
            source.clone(),
            types::title_to_id(source),
            item.published_at
                .map(|p| p.date().to_string())
                .unwrap_or(String::from("")),
            item.published_at.map(|p| p.timestamp()).unwrap_or(0),
        )
    }
}

impl types::site::Category {
    pub fn make(
        id: String,
        title: String,
        sources: Vec<types::state::Source>,
        other_categories: Vec<types::Title>,
    ) -> types::site::Category {
        let mut items = sources
            .iter()
            .flat_map(|s| {
                s.items
                    .iter()
                    .map(|i| types::site::Item::from_state_item(i.clone(), s.clone().title))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        items.sort_by_key(|i| -1 * i.timestamp);

        types::site::Category {
            id,
            title,
            items,
            other_categories: other_categories
                .iter()
                .map(|c| types::site::CategoryReference {
                    id: types::title_to_id(c.clone()),
                    title: c.clone(),
                })
                .collect(),
        }
    }

    pub fn render(self, tera: &Tera) -> Result<types::site::CategoryPage, String> {
        let context = Context::from_serialize(&self).map_err(|e| e.to_string())?;
        let rendering = tera
            .render("category_page.html", &context)
            .map_err(|e| e.to_string())?;

        Ok(types::site::CategoryPage {
            id: self.id,
            rendered: rendering,
        })
    }
}

impl types::site::Source {
    pub fn from_source_state(source: types::state::Source) -> types::site::Source {
        let mut items = source
            .items
            .iter()
            .map(|i| types::site::Item::from_state_item(i.clone(), source.title.clone()))
            .collect::<Vec<_>>();

        // need to sort in place
        items.sort_by_key(|i| -1 * i.timestamp);

        types::site::Source {
            id: types::title_to_id(source.title.clone()),
            title: source.title,
            items,
            category_id: types::title_to_id(source.category.title.clone()),
            category_name: source.category.title,
        }
    }

    pub fn render(self, tera: &Tera) -> Result<types::site::SourcePage, String> {
        let context = Context::from_serialize(&self).map_err(|e| e.to_string())?;
        let rendering = tera
            .render("source_page.html", &context)
            .map_err(|e| e.to_string())?;

        Ok(types::site::SourcePage {
            id: self.id,
            rendered: rendering,
        })
    }
}

impl types::site::Index {
    pub fn render(self, tera: &Tera) -> Result<String, String> {
        let context = Context::from_serialize(&self).map_err(|e| e.to_string())?;
        let rendering = tera
            .render("index.html", &context)
            .map_err(|e| e.to_string())?;
        Ok(rendering)
    }
}
