// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod management_plugin_enabled
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;pub struct SerdejsonValueQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> postgres_types::Json<& serde_json::value::RawValue>,
    mapper: fn(postgres_types::Json<& serde_json::value::RawValue>) -> T,
} impl<'a, C, T:'a, const N: usize> SerdejsonValueQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(postgres_types::Json<& serde_json::value::RawValue>) -> R) ->
    SerdejsonValueQuery<'a,C,R,N>
    {
        SerdejsonValueQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}pub fn management_plugin_enabled() -> ManagementPluginEnabledStmt
{ ManagementPluginEnabledStmt(cornucopia_async::private::Stmt::new("SELECT
    configuration -> 'plugins' -> 'management' -> 'enabled'
FROM
    \"Nightly\".\"GuildConfigurations\"
WHERE
    guild_id = $1")) } pub struct
ManagementPluginEnabledStmt(cornucopia_async::private::Stmt); impl ManagementPluginEnabledStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
guild_id: &'a T1,) -> SerdejsonValueQuery<'a,C,
serde_json::Value, 1>
{
    SerdejsonValueQuery
    {
        client, params: [guild_id,], stmt: &mut self.0, extractor:
        |row| { row.get(0) }, mapper: |it| { serde_json::from_str(it.0.get()).unwrap() },
    }
} }}pub mod plugin_enabled
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct PluginEnabledParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,> { pub plugin: T1,pub guild_id: T2,}pub struct SerdejsonValueQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> postgres_types::Json<& serde_json::value::RawValue>,
    mapper: fn(postgres_types::Json<& serde_json::value::RawValue>) -> T,
} impl<'a, C, T:'a, const N: usize> SerdejsonValueQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(postgres_types::Json<& serde_json::value::RawValue>) -> R) ->
    SerdejsonValueQuery<'a,C,R,N>
    {
        SerdejsonValueQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}pub fn plugin_enabled() -> PluginEnabledStmt
{ PluginEnabledStmt(cornucopia_async::private::Stmt::new("SELECT
    configuration -> 'plugins' -> $1 -> 'enabled'
FROM
    \"Nightly\".\"GuildConfigurations\"
WHERE
    guild_id = $2")) } pub struct
PluginEnabledStmt(cornucopia_async::private::Stmt); impl PluginEnabledStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
plugin: &'a T1,guild_id: &'a T2,) -> SerdejsonValueQuery<'a,C,
serde_json::Value, 2>
{
    SerdejsonValueQuery
    {
        client, params: [plugin,guild_id,], stmt: &mut self.0, extractor:
        |row| { row.get(0) }, mapper: |it| { serde_json::from_str(it.0.get()).unwrap() },
    }
} }impl <'a, C: GenericClient,T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,> cornucopia_async::Params<'a,
PluginEnabledParams<T1,T2,>, SerdejsonValueQuery<'a, C,
serde_json::Value, 2>, C> for PluginEnabledStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    PluginEnabledParams<T1,T2,>) -> SerdejsonValueQuery<'a, C,
    serde_json::Value, 2>
    { self.bind(client, &params.plugin,&params.guild_id,) }
}}pub mod utilities_plugin_enabled
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;pub struct SerdejsonValueQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> postgres_types::Json<& serde_json::value::RawValue>,
    mapper: fn(postgres_types::Json<& serde_json::value::RawValue>) -> T,
} impl<'a, C, T:'a, const N: usize> SerdejsonValueQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(postgres_types::Json<& serde_json::value::RawValue>) -> R) ->
    SerdejsonValueQuery<'a,C,R,N>
    {
        SerdejsonValueQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}pub fn utilities_plugin_enabled() -> UtilitiesPluginEnabledStmt
{ UtilitiesPluginEnabledStmt(cornucopia_async::private::Stmt::new("SELECT
    configuration -> 'plugins' -> 'utilities' -> 'enabled'
FROM
    \"Nightly\".\"GuildConfigurations\"
WHERE
    guild_id = $1")) } pub struct
UtilitiesPluginEnabledStmt(cornucopia_async::private::Stmt); impl UtilitiesPluginEnabledStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
guild_id: &'a T1,) -> SerdejsonValueQuery<'a,C,
serde_json::Value, 1>
{
    SerdejsonValueQuery
    {
        client, params: [guild_id,], stmt: &mut self.0, extractor:
        |row| { row.get(0) }, mapper: |it| { serde_json::from_str(it.0.get()).unwrap() },
    }
} }}}