// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod cached_guild_select_by_id
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug, Clone, PartialEq,)] pub struct CachedGuildSelectById
{ pub default_message_notifications : i32,pub explicit_content_filter : i32,pub features : Vec<String>,pub icon : Option<String>,pub large : bool,pub name : String,pub owner_id : String,pub id : String,}pub struct CachedGuildSelectByIdBorrowed<'a> { pub default_message_notifications : i32,pub explicit_content_filter : i32,pub features : cornucopia_async::ArrayIterator<'a, &'a str>,pub icon : Option<&'a str>,pub large : bool,pub name : &'a str,pub owner_id : &'a str,pub id : &'a str,}
impl<'a> From<CachedGuildSelectByIdBorrowed<'a>> for CachedGuildSelectById
{
    fn from(CachedGuildSelectByIdBorrowed { default_message_notifications,explicit_content_filter,features,icon,large,name,owner_id,id,}: CachedGuildSelectByIdBorrowed<'a>) ->
    Self { Self { default_message_notifications,explicit_content_filter,features: features.map(|v| v.into()).collect(),icon: icon.map(|v| v.into()),large,name: name.into(),owner_id: owner_id.into(),id: id.into(),} }
}pub struct CachedGuildSelectByIdQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> CachedGuildSelectByIdBorrowed,
    mapper: fn(CachedGuildSelectByIdBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> CachedGuildSelectByIdQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(CachedGuildSelectByIdBorrowed) -> R) ->
    CachedGuildSelectByIdQuery<'a,C,R,N>
    {
        CachedGuildSelectByIdQuery
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
        Ok(self.client.query_opt(stmt, &self.params)
        .await?.map(|row| (self.mapper)((self.extractor)(&row))))
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
}pub fn cached_guild_select_by_id() -> CachedGuildSelectByIdStmt
{ CachedGuildSelectByIdStmt(cornucopia_async::private::Stmt::new("SELECT 
    *
FROM
    \"DiscordFrontendNightly\".public.\"CachedGuilds\"
WHERE
    \"id\" = $1")) } pub struct
CachedGuildSelectByIdStmt(cornucopia_async::private::Stmt); impl CachedGuildSelectByIdStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
id: &'a T1,) -> CachedGuildSelectByIdQuery<'a,C,
CachedGuildSelectById, 1>
{
    CachedGuildSelectByIdQuery
    {
        client, params: [id,], stmt: &mut self.0, extractor:
        |row| { CachedGuildSelectByIdBorrowed { default_message_notifications: row.get(0),explicit_content_filter: row.get(1),features: row.get(2),icon: row.get(3),large: row.get(4),name: row.get(5),owner_id: row.get(6),id: row.get(7),} }, mapper: |it| { <CachedGuildSelectById>::from(it) },
    }
} }}pub mod cached_guild_upsert
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct CachedGuildUpsertParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::ArraySql<Item = T1>,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,> { pub default_message_notifications: i32,pub explicit_content_filter: i32,pub features: T2,pub icon: Option<T3>,pub large: bool,pub name: T4,pub owner_id: T5,pub id: T6,}pub fn cached_guild_upsert() -> CachedGuildUpsertStmt
{ CachedGuildUpsertStmt(cornucopia_async::private::Stmt::new("INSERT INTO
    \"DiscordFrontendNightly\".public.\"CachedGuilds\" (\"default_message_notifications\", \"explicit_content_filter\", \"features\", \"icon\", \"large\", \"name\", \"owner_id\", \"id\")
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
ON CONFLICT (\"id\") DO UPDATE
    SET
        \"default_message_notifications\" = $1,
        \"explicit_content_filter\" = $2,
        \"features\" = $3,
        \"icon\" = $4,
        \"large\" = $5,
        \"name\" = $6,
        \"owner_id\" = $7")) } pub struct
CachedGuildUpsertStmt(cornucopia_async::private::Stmt); impl CachedGuildUpsertStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::ArraySql<Item = T1>,T3:
cornucopia_async::StringSql,T4:
cornucopia_async::StringSql,T5:
cornucopia_async::StringSql,T6:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
default_message_notifications: &'a i32,explicit_content_filter: &'a i32,features: &'a T2,icon: &'a Option<T3>,large: &'a bool,name: &'a T4,owner_id: &'a T5,id: &'a T6,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[default_message_notifications,explicit_content_filter,features,icon,large,name,owner_id,id,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,T2: cornucopia_async::ArraySql<Item = T1>,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,>
cornucopia_async::Params<'a, CachedGuildUpsertParams<T1,T2,T3,T4,T5,T6,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for CachedGuildUpsertStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    CachedGuildUpsertParams<T1,T2,T3,T4,T5,T6,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.default_message_notifications,&params.explicit_content_filter,&params.features,&params.icon,&params.large,&params.name,&params.owner_id,&params.id,)) }
}}pub mod cached_member_select_by_user_id_and_guild_id
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct CachedMemberSelectByUserIdAndGuildIdParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,> { pub user_id: T1,pub guild_id: T2,}#[derive( Debug, Clone, PartialEq,)] pub struct CachedMemberSelectByUserIdAndGuildId
{ pub user_id : String,pub guild_id : String,pub roles : Vec<String>,}pub struct CachedMemberSelectByUserIdAndGuildIdBorrowed<'a> { pub user_id : &'a str,pub guild_id : &'a str,pub roles : cornucopia_async::ArrayIterator<'a, &'a str>,}
impl<'a> From<CachedMemberSelectByUserIdAndGuildIdBorrowed<'a>> for CachedMemberSelectByUserIdAndGuildId
{
    fn from(CachedMemberSelectByUserIdAndGuildIdBorrowed { user_id,guild_id,roles,}: CachedMemberSelectByUserIdAndGuildIdBorrowed<'a>) ->
    Self { Self { user_id: user_id.into(),guild_id: guild_id.into(),roles: roles.map(|v| v.into()).collect(),} }
}pub struct CachedMemberSelectByUserIdAndGuildIdQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> CachedMemberSelectByUserIdAndGuildIdBorrowed,
    mapper: fn(CachedMemberSelectByUserIdAndGuildIdBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> CachedMemberSelectByUserIdAndGuildIdQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(CachedMemberSelectByUserIdAndGuildIdBorrowed) -> R) ->
    CachedMemberSelectByUserIdAndGuildIdQuery<'a,C,R,N>
    {
        CachedMemberSelectByUserIdAndGuildIdQuery
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
        Ok(self.client.query_opt(stmt, &self.params)
        .await?.map(|row| (self.mapper)((self.extractor)(&row))))
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
}pub fn cached_member_select_by_user_id_and_guild_id() -> CachedMemberSelectByUserIdAndGuildIdStmt
{ CachedMemberSelectByUserIdAndGuildIdStmt(cornucopia_async::private::Stmt::new("SELECT
    *
FROM
    \"DiscordFrontendNightly\".public.\"CachedMembers\"
WHERE
    user_id = $1 AND
    guild_id = $2")) } pub struct
CachedMemberSelectByUserIdAndGuildIdStmt(cornucopia_async::private::Stmt); impl CachedMemberSelectByUserIdAndGuildIdStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
user_id: &'a T1,guild_id: &'a T2,) -> CachedMemberSelectByUserIdAndGuildIdQuery<'a,C,
CachedMemberSelectByUserIdAndGuildId, 2>
{
    CachedMemberSelectByUserIdAndGuildIdQuery
    {
        client, params: [user_id,guild_id,], stmt: &mut self.0, extractor:
        |row| { CachedMemberSelectByUserIdAndGuildIdBorrowed { user_id: row.get(0),guild_id: row.get(1),roles: row.get(2),} }, mapper: |it| { <CachedMemberSelectByUserIdAndGuildId>::from(it) },
    }
} }impl <'a, C: GenericClient,T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,> cornucopia_async::Params<'a,
CachedMemberSelectByUserIdAndGuildIdParams<T1,T2,>, CachedMemberSelectByUserIdAndGuildIdQuery<'a, C,
CachedMemberSelectByUserIdAndGuildId, 2>, C> for CachedMemberSelectByUserIdAndGuildIdStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    CachedMemberSelectByUserIdAndGuildIdParams<T1,T2,>) -> CachedMemberSelectByUserIdAndGuildIdQuery<'a, C,
    CachedMemberSelectByUserIdAndGuildId, 2>
    { self.bind(client, &params.user_id,&params.guild_id,) }
}}pub mod cached_member_upsert
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct CachedMemberUpsertParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::ArraySql<Item = T3>,> { pub user_id: T1,pub guild_id: T2,pub roles: T4,}pub fn cached_member_upsert() -> CachedMemberUpsertStmt
{ CachedMemberUpsertStmt(cornucopia_async::private::Stmt::new("INSERT INTO \"DiscordFrontendNightly\".public.\"CachedMembers\" (\"user_id\", \"guild_id\", \"roles\")
VALUES ($1, $2, $3)
ON CONFLICT (\"user_id\", \"guild_id\") DO UPDATE
    SET
        \"roles\" = $3")) } pub struct
CachedMemberUpsertStmt(cornucopia_async::private::Stmt); impl CachedMemberUpsertStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,T3:
cornucopia_async::StringSql,T4:
cornucopia_async::ArraySql<Item = T3>,>(&'a mut self, client: &'a  C,
user_id: &'a T1,guild_id: &'a T2,roles: &'a T4,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[user_id,guild_id,roles,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::ArraySql<Item = T3>,>
cornucopia_async::Params<'a, CachedMemberUpsertParams<T1,T2,T3,T4,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for CachedMemberUpsertStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    CachedMemberUpsertParams<T1,T2,T3,T4,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.user_id,&params.guild_id,&params.roles,)) }
}}}