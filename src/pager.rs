use std::pin::Pin;

use cassandra_proto::{
  error,
  frame::frame_result::{RowsMetadata, RowsMetadataFlag},
  query::QueryParamsBuilder,
  types::{rows::Row, CBytes},
};

use crate::{
  query::{ExecExecutor, PreparedQuery, QueryExecutor},
  session::Session,
  transport::CDRSTransport,
};

pub type PageSize = i32;

pub struct SessionPager<T: CDRSTransport + 'static> {
  page_size: i32,
  session: Session<T>,
}

impl<T: CDRSTransport + 'static> SessionPager<T> {
  pub fn new(session: Session<T>, page_size: PageSize) -> SessionPager<T> {
    SessionPager { session, page_size }
  }

  pub fn query_with_pager_state<Q>(
    &mut self,
    query: Q,
    state: PagerState,
  ) -> QueryPager<Q, SessionPager<T>>
  where
    Q: ToString,
  {
    QueryPager {
      pager: self,
      pager_state: state,
      query,
    }
  }

  pub fn query<Q>(&mut self, query: Q) -> QueryPager<Q, SessionPager<T>>
  where
    Q: ToString,
  {
    self.query_with_pager_state(query, PagerState::new())
  }

  pub fn exec_with_pager_state(
    &mut self,
    query: PreparedQuery,
    state: PagerState,
  ) -> ExecPager<SessionPager<T>> {
    ExecPager {
      pager: self,
      pager_state: state,
      query,
    }
  }

  pub fn exec(&mut self, query: PreparedQuery) -> ExecPager<SessionPager<T>> {
    self.exec_with_pager_state(query, PagerState::new())
  }
}

pub struct QueryPager<'a, Q: ToString, P: 'a> {
  pager: &'a mut P,
  pager_state: PagerState,
  query: Q,
}

impl<'a, Q: ToString, T: CDRSTransport + 'static> QueryPager<'a, Q, SessionPager<T>> {
  pub async fn next(&mut self) -> error::Result<Vec<Row>> {
    let mut params = QueryParamsBuilder::new().page_size(self.pager.page_size);
    if self.pager_state.cursor.is_some() {
      params = params.paging_state(self.pager_state.cursor.clone().unwrap());
    }

    let pinned_session = Pin::new(&mut self.pager.session);

    let body = pinned_session
      .query_with_params(self.query.to_string(), params.finalize())
      .await
      .and_then(|frame| frame.get_body())?;

    let metadata_res: error::Result<RowsMetadata> = body
      .as_rows_metadata()
      .ok_or("Pager query should yield a vector of rows".into());
    let metadata = metadata_res?;

    self.pager_state.has_more_pages =
      Some(RowsMetadataFlag::has_has_more_pages(metadata.flags.clone()));
    self.pager_state.cursor = metadata.paging_state.clone();
    body
      .into_rows()
      .ok_or("Pager query should yield a vector of rows".into())
  }

  pub fn has_more(&self) -> bool {
    self.pager_state.has_more_pages.unwrap_or(false)
  }

  /// This method returns a copy of pager state so
  /// the state may be used later for continuing paging.
  pub fn pager_state(&self) -> PagerState {
    self.pager_state.clone()
  }
}

pub struct ExecPager<'a, P: 'a> {
  pager: &'a mut P,
  pager_state: PagerState,
  query: PreparedQuery,
}

impl<'a, T: CDRSTransport + 'static> ExecPager<'a, SessionPager<T>> {
  pub async fn next(&mut self) -> error::Result<Vec<Row>> {
    let mut params = QueryParamsBuilder::new().page_size(self.pager.page_size);
    if self.pager_state.cursor.is_some() {
      params = params.paging_state(self.pager_state.cursor.clone().unwrap());
    }
    let pinned_session = Pin::new(&mut self.pager.session);

    let body = pinned_session
      .exec_with_params(&self.query, params.finalize())
      .await
      .and_then(|frame| frame.get_body())?;

    let metadata_res: error::Result<RowsMetadata> = body
      .as_rows_metadata()
      .ok_or("Pager query should yield a vector of rows".into());
    let metadata = metadata_res?;

    self.pager_state.has_more_pages =
      Some(RowsMetadataFlag::has_has_more_pages(metadata.flags.clone()));
    self.pager_state.cursor = metadata.paging_state.clone();
    body
      .into_rows()
      .ok_or("Pager query should yield a vector of rows".into())
  }

  pub fn has_more(&self) -> bool {
    self.pager_state.has_more_pages.unwrap_or(false)
  }

  /// This method returns a copy of pager state so
  /// the state may be used later for continuing paging.
  pub fn pager_state(&self) -> PagerState {
    self.pager_state.clone()
  }
}

#[derive(Clone, PartialEq, Debug)]
pub struct PagerState {
  cursor: Option<CBytes>,
  has_more_pages: Option<bool>,
}

impl PagerState {
  pub fn new() -> Self {
    PagerState {
      cursor: None,
      has_more_pages: None,
    }
  }

  pub fn with_cursor(cursor: CBytes) -> Self {
    PagerState {
      cursor: Some(cursor),
      has_more_pages: None,
    }
  }

  pub fn with_cursor_and_more_flag(cursor: CBytes, has_more: bool) -> Self {
    PagerState {
      cursor: Some(cursor),
      has_more_pages: Some(has_more),
    }
  }

  pub fn has_more(&self) -> bool {
    self.has_more_pages.unwrap_or(false)
  }

  pub fn get_cursor(&self) -> Option<CBytes> {
    self.cursor.clone()
  }
}
