// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod plugin_enabled
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct PluginEnabledParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,> { pub plugin: T1,pub guild_id: T2,}#[derive( Debug, Clone, PartialEq,)] pub struct PluginEnabled
{ pub guild_id : String,pub dashboard_admins : Vec<String>,pub dashboard_editors : Vec<String>,pub dashboard_viewers : Vec<String>,pub appearance_nickname : String,pub appearance_colour : i64,pub enabled_plugins : Vec<String>,}pub struct PluginEnabledBorrowed<'a> { pub guild_id : &'a str,pub dashboard_admins : cornucopia_async::ArrayIterator<'a, &'a str>,pub dashboard_editors : cornucopia_async::ArrayIterator<'a, &'a str>,pub dashboard_viewers : cornucopia_async::ArrayIterator<'a, &'a str>,pub appearance_nickname : &'a str,pub appearance_colour : i64,pub enabled_plugins : cornucopia_async::ArrayIterator<'a, &'a str>,}
impl<'a> From<PluginEnabledBorrowed<'a>> for PluginEnabled
{
    fn from(PluginEnabledBorrowed { guild_id,dashboard_admins,dashboard_editors,dashboard_viewers,appearance_nickname,appearance_colour,enabled_plugins,}: PluginEnabledBorrowed<'a>) ->
    Self { Self { guild_id: guild_id.into(),dashboard_admins: dashboard_admins.map(|v| v.into()).collect(),dashboard_editors: dashboard_editors.map(|v| v.into()).collect(),dashboard_viewers: dashboard_viewers.map(|v| v.into()).collect(),appearance_nickname: appearance_nickname.into(),appearance_colour,enabled_plugins: enabled_plugins.map(|v| v.into()).collect(),} }
}pub struct PluginEnabledQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> PluginEnabledBorrowed,
    mapper: fn(PluginEnabledBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> PluginEnabledQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(PluginEnabledBorrowed) -> R) ->
    PluginEnabledQuery<'a,C,R,N>
    {
        PluginEnabledQuery
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
    *
FROM
    \"Nightly\".\"GuildConfigurations\"
WHERE
    \"enabled_plugins\" @> array[ $1 ] AND
    \"guild_id\" = $2")) } pub struct
PluginEnabledStmt(cornucopia_async::private::Stmt); impl PluginEnabledStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
plugin: &'a T1,guild_id: &'a T2,) -> PluginEnabledQuery<'a,C,
PluginEnabled, 2>
{
    PluginEnabledQuery
    {
        client, params: [plugin,guild_id,], stmt: &mut self.0, extractor:
        |row| { PluginEnabledBorrowed { guild_id: row.get(0),dashboard_admins: row.get(1),dashboard_editors: row.get(2),dashboard_viewers: row.get(3),appearance_nickname: row.get(4),appearance_colour: row.get(5),enabled_plugins: row.get(6),} }, mapper: |it| { <PluginEnabled>::from(it) },
    }
} }impl <'a, C: GenericClient,T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,> cornucopia_async::Params<'a,
PluginEnabledParams<T1,T2,>, PluginEnabledQuery<'a, C,
PluginEnabled, 2>, C> for PluginEnabledStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    PluginEnabledParams<T1,T2,>) -> PluginEnabledQuery<'a, C,
    PluginEnabled, 2>
    { self.bind(client, &params.plugin,&params.guild_id,) }
}}}