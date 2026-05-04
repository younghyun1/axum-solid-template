use std::{path::PathBuf, sync::Arc};

use tantivy::{Index, doc};
use tokio::task;
use tracing::info;

use crate::{
    domain::marketplace::search::{MarketplaceSearchDocument, MarketplaceSearchHit},
    init::{
        db_pool::DbPool,
        state::search::marketplace::{
            document_source::load_documents,
            error::{MarketplaceSearchError, index_error},
            query::search_index,
            schema::{MarketplaceSearchFields, search_schema},
        },
    },
};

#[derive(Clone, Debug)]
pub struct MarketplaceSearchIndex {
    inner: Arc<MarketplaceSearchInner>,
}

#[derive(Debug)]
pub struct MarketplaceSearchInner {
    pub index: Index,
    pub fields: MarketplaceSearchFields,
}

impl MarketplaceSearchIndex {
    pub async fn load(db_pool: &DbPool, index_path: &str) -> Result<Self, MarketplaceSearchError> {
        let documents = match load_documents(db_pool).await {
            Ok(documents) => documents,
            Err(error) => return Err(error),
        };
        let path = PathBuf::from(index_path);
        let document_count = documents.len();
        let result = task::spawn_blocking(move || build_index(path, documents)).await;
        let index = match result {
            Ok(index_result) => match index_result {
                Ok(index) => index,
                Err(error) => return Err(error),
            },
            Err(error) => {
                return Err(MarketplaceSearchError::TaskJoin {
                    error: error.to_string(),
                });
            }
        };

        info!(
            document_count,
            index_path, "Loaded marketplace search index"
        );
        Ok(index)
    }

    pub async fn rebuild(&self, db_pool: &DbPool) -> Result<usize, MarketplaceSearchError> {
        let documents = match load_documents(db_pool).await {
            Ok(documents) => documents,
            Err(error) => return Err(error),
        };
        let document_count = documents.len();
        let inner = Arc::clone(&self.inner);
        let result = task::spawn_blocking(move || rebuild_index(&inner, documents)).await;

        match result {
            Ok(index_result) => match index_result {
                Ok(()) => {
                    info!(document_count, "Rebuilt marketplace search index");
                    Ok(document_count)
                }
                Err(error) => Err(error),
            },
            Err(error) => Err(MarketplaceSearchError::TaskJoin {
                error: error.to_string(),
            }),
        }
    }

    pub async fn search(
        &self,
        query_text: String,
        kind: Option<crate::domain::marketplace::search::MarketplaceSearchResultKind>,
        limit: usize,
    ) -> Result<Vec<MarketplaceSearchHit>, MarketplaceSearchError> {
        let inner = Arc::clone(&self.inner);
        let result =
            task::spawn_blocking(move || search_index(&inner, &query_text, kind, limit)).await;

        match result {
            Ok(search_result) => search_result,
            Err(error) => Err(MarketplaceSearchError::TaskJoin {
                error: error.to_string(),
            }),
        }
    }
}

fn build_index(
    index_path: PathBuf,
    documents: Vec<MarketplaceSearchDocument>,
) -> Result<MarketplaceSearchIndex, MarketplaceSearchError> {
    if index_path.exists() {
        match std::fs::remove_dir_all(&index_path) {
            Ok(()) => {}
            Err(error) => return Err(index_error(error)),
        }
    }
    match std::fs::create_dir_all(&index_path) {
        Ok(()) => {}
        Err(error) => return Err(index_error(error)),
    }

    let (schema, fields) = search_schema();
    let index = match Index::create_in_dir(&index_path, schema) {
        Ok(index) => index,
        Err(error) => return Err(index_error(error)),
    };
    let inner = MarketplaceSearchInner { index, fields };
    match rebuild_index(&inner, documents) {
        Ok(()) => Ok(MarketplaceSearchIndex {
            inner: Arc::new(inner),
        }),
        Err(error) => Err(error),
    }
}

fn rebuild_index(
    inner: &MarketplaceSearchInner,
    documents: Vec<MarketplaceSearchDocument>,
) -> Result<(), MarketplaceSearchError> {
    let mut writer = match inner.index.writer(50_000_000) {
        Ok(writer) => writer,
        Err(error) => return Err(index_error(error)),
    };
    match writer.delete_all_documents() {
        Ok(_) => {}
        Err(error) => return Err(index_error(error)),
    }
    for document in documents {
        let tantivy_document = doc!(
            inner.fields.kind => document.kind.as_str(),
            inner.fields.title => document.title,
            inner.fields.subtitle => document.subtitle,
            inner.fields.body => document.body,
            inner.fields.slug => document.slug,
            inner.fields.url_path => document.url_path,
            inner.fields.updated_at => document.updated_at.to_rfc3339(),
        );
        match writer.add_document(tantivy_document) {
            Ok(_) => {}
            Err(error) => return Err(index_error(error)),
        }
    }
    match writer.commit() {
        Ok(_) => Ok(()),
        Err(error) => Err(index_error(error)),
    }
}
