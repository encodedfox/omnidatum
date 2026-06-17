use anyhow::Result;
use od_core::{Book, CanonicalData, Collection, ManualProject, Repository, WebReference};

/// Sort field for repository queries
#[derive(Debug, Default, Clone, PartialEq)]
pub enum SortField {
    #[default]
    Stars,
    Name,
    Updated,
    Created,
    Quality,
}

/// Sort direction
#[derive(Debug, Default, Clone, PartialEq)]
pub enum SortOrder {
    #[default]
    Desc,
    Asc,
}

/// Abstraction over persistence backends.
pub trait RepoStore {
    fn load_all(&self) -> Result<CanonicalData>;
    fn save_all(&self, data: &CanonicalData) -> Result<()>;
    fn get_repo(&self, id: &str) -> Result<Option<Repository>>;
    fn upsert_repo(&self, repo: &Repository) -> Result<()>;
    fn list_repos(&self, filter: &RepoFilter) -> Result<Vec<Repository>>;
    fn count_repos(&self, filter: &RepoFilter) -> Result<usize>;
    fn delete_repo(&self, id: &str) -> Result<bool>;

    fn list_collections(&self) -> Result<Vec<Collection>>;
    fn get_collection(&self, id: &str) -> Result<Option<Collection>>;
    fn save_collection(&self, collection: &Collection) -> Result<()>;
    fn delete_collection(&self, id: &str) -> Result<bool>;
}

/// Filter criteria for repository queries.
#[derive(Debug, Default, Clone)]
pub struct RepoFilter {
    pub language: Option<String>,
    pub archived: Option<bool>,
    pub min_stars: Option<u32>,
    pub max_stars: Option<u32>,
    pub source: Option<String>,
    pub tag: Option<String>,
    pub owner: Option<String>,
    pub license: Option<String>,
    pub topic: Option<String>,
    pub updated_after: Option<String>,
    pub created_before: Option<String>,
    pub sort: SortField,
    pub order: SortOrder,
    pub limit: Option<usize>,
    pub search_query: Option<String>,
}

// Suppress unused-import warnings — these are part of the public trait surface.
const _: fn() = || {
    let _: Option<Book> = None;
    let _: Option<Collection> = None;
    let _: Option<ManualProject> = None;
    let _: Option<WebReference> = None;
};
