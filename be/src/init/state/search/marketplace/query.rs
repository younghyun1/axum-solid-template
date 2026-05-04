use chrono::{DateTime, Utc};
use serde_json::Value;
use tantivy::{Document, TantivyDocument, collector::TopDocs, query::QueryParser};

use crate::{
    domain::marketplace::search::{MarketplaceSearchHit, MarketplaceSearchResultKind},
    init::state::search::marketplace::{
        error::{MarketplaceSearchError, index_error},
        index::MarketplaceSearchInner,
    },
};

pub fn search_index(
    inner: &MarketplaceSearchInner,
    query_text: &str,
    kind: Option<MarketplaceSearchResultKind>,
    limit: usize,
) -> Result<Vec<MarketplaceSearchHit>, MarketplaceSearchError> {
    let reader = match inner.index.reader() {
        Ok(reader) => reader,
        Err(error) => return Err(index_error(error)),
    };
    let searcher = reader.searcher();
    let parser = QueryParser::for_index(
        &inner.index,
        vec![inner.fields.title, inner.fields.subtitle, inner.fields.body],
    );
    let query = match parser.parse_query(query_text) {
        Ok(query) => query,
        Err(error) => return Err(index_error(error)),
    };
    let top_docs = match searcher.search(&query, &TopDocs::with_limit(limit).order_by_score()) {
        Ok(top_docs) => top_docs,
        Err(error) => return Err(index_error(error)),
    };
    let mut hits = Vec::with_capacity(top_docs.len());
    for (score, address) in top_docs {
        let document = match searcher.doc::<TantivyDocument>(address) {
            Ok(document) => document,
            Err(error) => return Err(index_error(error)),
        };
        match hit_from_document(&document, inner, kind, score) {
            Ok(Some(hit)) => hits.push(hit),
            Ok(None) => {}
            Err(error) => return Err(error),
        }
    }

    Ok(hits)
}

fn hit_from_document(
    document: &TantivyDocument,
    inner: &MarketplaceSearchInner,
    kind: Option<MarketplaceSearchResultKind>,
    score: f32,
) -> Result<Option<MarketplaceSearchHit>, MarketplaceSearchError> {
    let json = document.to_json(&inner.index.schema());
    let value: Value = match serde_json::from_str(&json) {
        Ok(value) => value,
        Err(error) => return Err(index_error(error)),
    };
    let hit_kind = match MarketplaceSearchResultKind::from_wire_value(&json_text(&value, "kind")) {
        Some(kind) => kind,
        None => return Ok(None),
    };
    match kind {
        Some(expected_kind) if expected_kind != hit_kind => return Ok(None),
        Some(_) | None => {}
    }
    let body = json_text(&value, "body");
    Ok(Some(MarketplaceSearchHit {
        kind: hit_kind,
        title: json_text(&value, "title"),
        subtitle: json_text(&value, "subtitle"),
        slug: json_text(&value, "slug"),
        url_path: json_text(&value, "url_path"),
        snippet: snippet(&body),
        score,
        updated_at: parse_datetime(&json_text(&value, "updated_at")),
    }))
}

fn json_text(value: &Value, key: &str) -> String {
    match value.get(key) {
        Some(Value::String(text)) => text.clone(),
        Some(Value::Array(values)) => {
            for value in values {
                if let Value::String(text) = value {
                    return text.clone();
                }
            }
            String::new()
        }
        Some(_) | None => String::new(),
    }
}

fn snippet(body: &str) -> String {
    let mut snippet = body.chars().take(220).collect::<String>();
    if body.chars().count() > 220 {
        snippet.push_str("...");
    }
    snippet
}

fn parse_datetime(value: &str) -> Option<DateTime<Utc>> {
    match DateTime::parse_from_rfc3339(value) {
        Ok(datetime) => Some(datetime.with_timezone(&Utc)),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_datetime, snippet};

    #[test]
    fn snippet_limits_long_body() {
        let body = "a".repeat(230);

        let value = snippet(&body);

        assert_eq!(value.chars().count(), 223);
        assert!(value.ends_with("..."));
    }

    #[test]
    fn parse_datetime_accepts_rfc3339() {
        let parsed = parse_datetime("2026-05-04T12:00:00Z");

        assert!(parsed.is_some());
    }

    #[test]
    fn parse_datetime_rejects_invalid_text() {
        let parsed = parse_datetime("not-a-date");

        assert!(parsed.is_none());
    }
}
