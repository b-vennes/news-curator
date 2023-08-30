use std::collections::HashMap;
use std::fs;
use std::path::Path;

use tera::Tera;

use crate::result_ops;
use crate::types;

pub struct ProgramSync {
    pub tera: Tera,
}

impl types::program::Program for ProgramSync {
    fn get_state(&self, config: types::config::Config) -> Result<types::state::State, String> {
        println!("Retrieving state");

        let sources = result_ops::traverse_vec::<types::state::Source, String>(
            config
                .sources
                .iter()
                .map(|s| types::state::Source::load(s.clone()))
                .map(|s_res| {
                    s_res.map(|s| {
                        s.filter_items_by_date(
                            chrono::offset::Utc::now().naive_utc() - chrono::Duration::days(3),
                        )
                    })
                })
                .collect(),
        )?;

        Ok(types::state::State { sources })
    }

    fn make_site(&self, state: types::state::State) -> Result<types::site::Site, String> {
        println!("Making site");

        let empty_categories_map: HashMap<types::Title, Vec<types::state::Source>> = HashMap::new();

        let categories_map: HashMap<types::Title, Vec<types::state::Source>> = state
            .sources
            .iter()
            .fold(empty_categories_map, |mut acc, source| {
                acc.entry(source.category.title.clone())
                    .and_modify(|s| s.push(source.clone()))
                    .or_insert(vec![source.clone()]);
                acc
            });

        let mut category_titles: Vec<types::Title> = categories_map.keys().cloned().collect::<Vec<_>>();
        category_titles.sort();

        let category_pages: Vec<types::site::CategoryPage> = result_ops::traverse_vec(
            categories_map
                .iter()
                .map(|(title, sources)| {
                    types::site::Category::make(
                        types::title_to_id(title.clone()),
                        title.clone(),
                        sources.clone().to_vec(),
                        category_titles
                            .iter()
                            .filter(|&t| t != title)
                            .map(|t| types::string_to_title(t.to_string()))
                            .collect(),
                    )
                    .render(&self.tera)
                })
                .collect(),
        )?;

        let mut source_sites: Vec<types::site::Source> = state
            .sources
            .iter()
            .map(|s| types::site::Source::from_source_state(s.clone()))
            .collect();
        source_sites.sort_by_key(|s| s.title.clone());

        let source_pages: Vec<types::site::SourcePage> = result_ops::traverse_vec(
            source_sites
                .iter()
                .map(|s| s.clone().render(&self.tera))
                .collect(),
        )?;

        let index_page: String = types::site::Index {
            categories: category_titles
                .iter()
                .map(|c| types::site::CategoryReference {
                    id: types::title_to_id(c.clone()),
                    title: c.clone(),
                })
                .collect(),
            sources: source_sites,
        }
        .render(&self.tera)?;

        Ok(types::site::Site {
            categories: category_pages,
            sources: source_pages,
            index: index_page,
        })
    }

    fn write_site(&self, site: types::site::Site) -> Result<(), String> {
        println!("Writing site");

        fs::create_dir_all(Path::new("./site/sources/")).map_err(|e| e.to_string())?;
        fs::create_dir_all(Path::new("./site/categories/")).map_err(|e| e.to_string())?;

        site.sources.iter().try_for_each(|s| {
            fs::write(
                Path::new(&format!("./site/sources/{}.html", s.id)),
                s.rendered.clone(),
            )
            .map_err(|e| e.to_string())
        })?;

        site.categories.iter().try_for_each(|c| {
            fs::write(
                Path::new(&format!("./site/categories/{}.html", c.id)),
                c.rendered.clone(),
            )
            .map_err(|e| e.to_string())
        })?;

        fs::write(Path::new("./site/index.html"), site.index).map_err(|e| e.to_string())?;

        Ok(())
    }
}
