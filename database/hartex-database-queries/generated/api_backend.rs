// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod types {}
#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod queries {
    pub mod start_timestamp_select_by_component {
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug, Clone, PartialEq)]
        pub struct SelectStartTimestampByComponent {
            pub component: String,
            pub timestamp: time::OffsetDateTime,
        }
        pub struct SelectStartTimestampByComponentBorrowed<'a> {
            pub component: &'a str,
            pub timestamp: time::OffsetDateTime,
        }
        impl<'a> From<SelectStartTimestampByComponentBorrowed<'a>> for SelectStartTimestampByComponent {
            fn from(
                SelectStartTimestampByComponentBorrowed {
                    component,
                    timestamp,
                }: SelectStartTimestampByComponentBorrowed<'a>,
            ) -> Self {
                Self {
                    component: component.into(),
                    timestamp,
                }
            }
        }
        pub struct SelectStartTimestampByComponentQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> SelectStartTimestampByComponentBorrowed,
            mapper: fn(SelectStartTimestampByComponentBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> SelectStartTimestampByComponentQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(SelectStartTimestampByComponentBorrowed) -> R,
            ) -> SelectStartTimestampByComponentQuery<'a, C, R, N> {
                SelectStartTimestampByComponentQuery {
                    client: self.client,
                    params: self.params,
                    stmt: self.stmt,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                let row = self.client.query_one(stmt, &self.params).await?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.iter().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)
                    .await?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub async fn iter(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt.prepare(self.client).await?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn select_start_timestamp_by_component() -> SelectStartTimestampByComponentStmt {
            SelectStartTimestampByComponentStmt(cornucopia_async::private::Stmt::new(
                "SELECT
    *
FROM
    \"APIBackend\".public.\"StartTimestamps\"
WHERE
    \"component\" = $1",
            ))
        }
        pub struct SelectStartTimestampByComponentStmt(cornucopia_async::private::Stmt);
        impl SelectStartTimestampByComponentStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
                &'a mut self,
                client: &'a C,
                component: &'a T1,
            ) -> SelectStartTimestampByComponentQuery<'a, C, SelectStartTimestampByComponent, 1>
            {
                SelectStartTimestampByComponentQuery {
                    client,
                    params: [component],
                    stmt: &mut self.0,
                    extractor: |row| SelectStartTimestampByComponentBorrowed {
                        component: row.get(0),
                        timestamp: row.get(1),
                    },
                    mapper: |it| <SelectStartTimestampByComponent>::from(it),
                }
            }
        }
    }
    pub mod start_timestamp_upsert {
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug)]
        pub struct StartTimestampUpsertParams<T1: cornucopia_async::StringSql> {
            pub component: T1,
            pub timestamp: time::OffsetDateTime,
        }
        pub fn start_timestamp_upsert() -> StartTimestampUpsertStmt {
            StartTimestampUpsertStmt(cornucopia_async::private::Stmt::new(
                "INSERT INTO
    \"APIBackend\".public.\"StartTimestamps\" (\"component\", \"timestamp\")
VALUES ($1, $2)
ON CONFLICT (\"component\") DO UPDATE
    SET
        \"timestamp\" = $2",
            ))
        }
        pub struct StartTimestampUpsertStmt(cornucopia_async::private::Stmt);
        impl StartTimestampUpsertStmt {
            pub async fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
                &'a mut self,
                client: &'a C,
                component: &'a T1,
                timestamp: &'a time::OffsetDateTime,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[component, timestamp]).await
            }
        }
        impl<'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql>
            cornucopia_async::Params<
                'a,
                StartTimestampUpsertParams<T1>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                >,
                C,
            > for StartTimestampUpsertStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a StartTimestampUpsertParams<T1>,
            ) -> std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            > {
                Box::pin(self.bind(client, &params.component, &params.timestamp))
            }
        }
    }
}
