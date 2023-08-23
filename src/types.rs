use serde::{Deserialize, Serialize};

pub type Link = String;
pub type Title = String;

pub fn string_to_title(s: String) -> Title {
    s
}

pub fn title_to_id(title: Title) -> String {
    title.to_lowercase().replace(" ", "-")
}

pub mod config {

    #[derive(Debug, super::Serialize, super::Deserialize, Clone)]
    pub enum SourceType {
        RSS,
        Atom,
    }

    #[derive(Debug, super::Serialize, super::Deserialize, Clone)]
    pub struct Source {
        pub title: super::Title,
        pub link: super::Link,
        pub category: super::Title,
        pub s_type: SourceType,
    }

    #[derive(Debug, super::Serialize, super::Deserialize, Clone)]
    pub struct Config {
        pub sources: Vec<Source>,
    }
}

pub mod state {

    use chrono::NaiveDateTime;

    #[derive(Debug, Clone)]
    pub struct Category {
        pub title: super::Title,
    }

    #[derive(Debug, Clone)]
    pub struct Item {
        pub title: super::Title,
        pub link: super::Link,
        pub published_at: Option<NaiveDateTime>,
    }

    #[derive(Debug, Clone)]
    pub struct Source {
        pub title: super::Title,
        pub items: Vec<Item>,
        pub category: Category,
        pub link: super::Link,
    }

    pub struct State {
        pub sources: Vec<Source>,
    }
}

pub mod site {

    #[derive(Debug, Clone, super::Serialize)]
    pub struct Item {
        pub title: String,
        pub link: String,
        pub source: String,
        pub source_id: String,
        pub published_at: String,
        pub timestamp: i64,
    }

    #[derive(Debug, Clone, super::Serialize)]
    pub struct CategoryReference {
        pub id: String,
        pub title: String,
    }

    #[derive(Debug, Clone, super::Serialize)]
    pub struct Category {
        pub id: String,
        pub title: String,
        pub items: Vec<Item>,
        pub other_categories: Vec<CategoryReference>,
    }

    pub struct CategoryPage {
        pub id: String,
        pub rendered: String,
    }

    #[derive(Debug, Clone, super::Serialize)]
    pub struct Source {
        pub id: String,
        pub title: String,
        pub items: Vec<Item>,
        pub category_id: String,
        pub category_name: String,
    }

    pub struct SourcePage {
        pub id: String,
        pub rendered: String,
    }

    #[derive(Debug, Clone, super::Serialize)]
    pub struct Index {
        pub categories: Vec<CategoryReference>,
    }

    pub struct Site {
        pub index: String,
        pub categories: Vec<CategoryPage>,
        pub sources: Vec<SourcePage>,
    }
}

pub mod program {
    pub trait Program {
        fn get_state(&self, config: super::config::Config) -> Result<super::state::State, String>;

        fn make_site(&self, state: super::state::State) -> Result<super::site::Site, String>;

        fn write_site(&self, site: super::site::Site) -> Result<(), String>;
    }
}
