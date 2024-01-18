// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod cached_guild_select_by_id
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug, Clone, PartialEq,)] pub struct CachedGuildSelectById
{ pub default_message_notifications : i32,pub explicit_content_filter : i32,pub features : Vec<String>,pub icon : Option<String>,pub large : bool,pub name : String,pub owner_id : String,pub id : String,pub premium_subscription_count : Option<i64>,pub premium_tier : i32,pub verification_level : i32,}pub struct CachedGuildSelectByIdBorrowed<'a> { pub default_message_notifications : i32,pub explicit_content_filter : i32,pub features : cornucopia_async::ArrayIterator<'a, &'a str>,pub icon : Option<&'a str>,pub large : bool,pub name : &'a str,pub owner_id : &'a str,pub id : &'a str,pub premium_subscription_count : Option<i64>,pub premium_tier : i32,pub verification_level : i32,}
impl<'a> From<CachedGuildSelectByIdBorrowed<'a>> for CachedGuildSelectById
{
    fn from(CachedGuildSelectByIdBorrowed { default_message_notifications,explicit_content_filter,features,icon,large,name,owner_id,id,premium_subscription_count,premium_tier,verification_level,}: CachedGuildSelectByIdBorrowed<'a>) ->
    Self { Self { default_message_notifications,explicit_content_filter,features: features.map(|v| v.into()).collect(),icon: icon.map(|v| v.into()),large,name: name.into(),owner_id: owner_id.into(),id: id.into(),premium_subscription_count,premium_tier,verification_level,} }
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
    \"DiscordFrontend\".\"Nightly\".\"CachedGuilds\"
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
        |row| { CachedGuildSelectByIdBorrowed { default_message_notifications: row.get(0),explicit_content_filter: row.get(1),features: row.get(2),icon: row.get(3),large: row.get(4),name: row.get(5),owner_id: row.get(6),id: row.get(7),premium_subscription_count: row.get(8),premium_tier: row.get(9),verification_level: row.get(10),} }, mapper: |it| { <CachedGuildSelectById>::from(it) },
    }
} }}pub mod cached_guild_upsert
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct CachedGuildUpsertParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::ArraySql<Item = T1>,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,> { pub default_message_notifications: i32,pub explicit_content_filter: i32,pub features: T2,pub icon: Option<T3>,pub large: bool,pub name: T4,pub owner_id: T5,pub id: T6,pub premium_subscription_count: Option<i64>,pub premium_tier: i32,pub verification_level: i32,}pub fn cached_guild_upsert() -> CachedGuildUpsertStmt
{ CachedGuildUpsertStmt(cornucopia_async::private::Stmt::new("INSERT INTO
    \"DiscordFrontend\".\"Nightly\".\"CachedGuilds\" (\"default_message_notifications\", \"explicit_content_filter\", \"features\", \"icon\", \"large\", \"name\", \"owner_id\", \"id\", \"premium_subscription_count\", \"premium_tier\", \"verification_level\")
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
ON CONFLICT (\"id\") DO UPDATE
    SET
        \"default_message_notifications\" = $1,
        \"explicit_content_filter\" = $2,
        \"features\" = $3,
        \"icon\" = $4,
        \"large\" = $5,
        \"name\" = $6,
        \"owner_id\" = $7,
        \"premium_subscription_count\" = $9,
        \"premium_tier\" = $10,
        \"verification_level\" = $11")) } pub struct
CachedGuildUpsertStmt(cornucopia_async::private::Stmt); impl CachedGuildUpsertStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::ArraySql<Item = T1>,T3:
cornucopia_async::StringSql,T4:
cornucopia_async::StringSql,T5:
cornucopia_async::StringSql,T6:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
default_message_notifications: &'a i32,explicit_content_filter: &'a i32,features: &'a T2,icon: &'a Option<T3>,large: &'a bool,name: &'a T4,owner_id: &'a T5,id: &'a T6,premium_subscription_count: &'a Option<i64>,premium_tier: &'a i32,verification_level: &'a i32,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[default_message_notifications,explicit_content_filter,features,icon,large,name,owner_id,id,premium_subscription_count,premium_tier,verification_level,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,T2: cornucopia_async::ArraySql<Item = T1>,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,>
cornucopia_async::Params<'a, CachedGuildUpsertParams<T1,T2,T3,T4,T5,T6,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for CachedGuildUpsertStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    CachedGuildUpsertParams<T1,T2,T3,T4,T5,T6,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.default_message_notifications,&params.explicit_content_filter,&params.features,&params.icon,&params.large,&params.name,&params.owner_id,&params.id,&params.premium_subscription_count,&params.premium_tier,&params.verification_level,)) }
}}pub mod cached_member_select_by_guild_id
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug, Clone, PartialEq,)] pub struct CachedMemberSelectByGuildId
{ pub user_id : String,pub guild_id : String,pub roles : Vec<String>,}pub struct CachedMemberSelectByGuildIdBorrowed<'a> { pub user_id : &'a str,pub guild_id : &'a str,pub roles : cornucopia_async::ArrayIterator<'a, &'a str>,}
impl<'a> From<CachedMemberSelectByGuildIdBorrowed<'a>> for CachedMemberSelectByGuildId
{
    fn from(CachedMemberSelectByGuildIdBorrowed { user_id,guild_id,roles,}: CachedMemberSelectByGuildIdBorrowed<'a>) ->
    Self { Self { user_id: user_id.into(),guild_id: guild_id.into(),roles: roles.map(|v| v.into()).collect(),} }
}pub struct CachedMemberSelectByGuildIdQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> CachedMemberSelectByGuildIdBorrowed,
    mapper: fn(CachedMemberSelectByGuildIdBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> CachedMemberSelectByGuildIdQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(CachedMemberSelectByGuildIdBorrowed) -> R) ->
    CachedMemberSelectByGuildIdQuery<'a,C,R,N>
    {
        CachedMemberSelectByGuildIdQuery
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
}pub fn cached_member_select_by_guild_id() -> CachedMemberSelectByGuildIdStmt
{ CachedMemberSelectByGuildIdStmt(cornucopia_async::private::Stmt::new("SELECT
    *
FROM
    \"DiscordFrontend\".\"Nightly\".\"CachedMembers\"
WHERE
    \"guild_id\" = $1")) } pub struct
CachedMemberSelectByGuildIdStmt(cornucopia_async::private::Stmt); impl CachedMemberSelectByGuildIdStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
guild_id: &'a T1,) -> CachedMemberSelectByGuildIdQuery<'a,C,
CachedMemberSelectByGuildId, 1>
{
    CachedMemberSelectByGuildIdQuery
    {
        client, params: [guild_id,], stmt: &mut self.0, extractor:
        |row| { CachedMemberSelectByGuildIdBorrowed { user_id: row.get(0),guild_id: row.get(1),roles: row.get(2),} }, mapper: |it| { <CachedMemberSelectByGuildId>::from(it) },
    }
} }}pub mod cached_member_select_by_user_id_and_guild_id
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
    \"DiscordFrontend\".\"Nightly\".\"CachedMembers\"
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
{ CachedMemberUpsertStmt(cornucopia_async::private::Stmt::new("INSERT INTO \"DiscordFrontend\".\"Nightly\".\"CachedMembers\" (\"user_id\", \"guild_id\", \"roles\")
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
}}pub mod cached_role_select_by_guild_id
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug, Clone, PartialEq,)] pub struct CachedRoleSelectByGuildId
{ pub icon : Option<String>,pub guild_id : String,pub id : String,pub flags : i32,pub hoist : bool,pub managed : bool,pub mentionable : bool,pub position : i32,pub color : i64,}pub struct CachedRoleSelectByGuildIdBorrowed<'a> { pub icon : Option<&'a str>,pub guild_id : &'a str,pub id : &'a str,pub flags : i32,pub hoist : bool,pub managed : bool,pub mentionable : bool,pub position : i32,pub color : i64,}
impl<'a> From<CachedRoleSelectByGuildIdBorrowed<'a>> for CachedRoleSelectByGuildId
{
    fn from(CachedRoleSelectByGuildIdBorrowed { icon,guild_id,id,flags,hoist,managed,mentionable,position,color,}: CachedRoleSelectByGuildIdBorrowed<'a>) ->
    Self { Self { icon: icon.map(|v| v.into()),guild_id: guild_id.into(),id: id.into(),flags,hoist,managed,mentionable,position,color,} }
}pub struct CachedRoleSelectByGuildIdQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> CachedRoleSelectByGuildIdBorrowed,
    mapper: fn(CachedRoleSelectByGuildIdBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> CachedRoleSelectByGuildIdQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(CachedRoleSelectByGuildIdBorrowed) -> R) ->
    CachedRoleSelectByGuildIdQuery<'a,C,R,N>
    {
        CachedRoleSelectByGuildIdQuery
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
}pub fn cached_role_select_by_guild_id() -> CachedRoleSelectByGuildIdStmt
{ CachedRoleSelectByGuildIdStmt(cornucopia_async::private::Stmt::new("SELECT
    *
FROM
    \"DiscordFrontend\".\"Nightly\".\"CachedRoles\"
WHERE
    \"guild_id\" = $1")) } pub struct
CachedRoleSelectByGuildIdStmt(cornucopia_async::private::Stmt); impl CachedRoleSelectByGuildIdStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
guild_id: &'a T1,) -> CachedRoleSelectByGuildIdQuery<'a,C,
CachedRoleSelectByGuildId, 1>
{
    CachedRoleSelectByGuildIdQuery
    {
        client, params: [guild_id,], stmt: &mut self.0, extractor:
        |row| { CachedRoleSelectByGuildIdBorrowed { icon: row.get(0),guild_id: row.get(1),id: row.get(2),flags: row.get(3),hoist: row.get(4),managed: row.get(5),mentionable: row.get(6),position: row.get(7),color: row.get(8),} }, mapper: |it| { <CachedRoleSelectByGuildId>::from(it) },
    }
} }}pub mod cached_role_select_by_id_and_guild_id
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct CachedRoleSelectByIdAndGuildIdParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,> { pub id: T1,pub guild_id: T2,}#[derive( Debug, Clone, PartialEq,)] pub struct CachedRoleSelectByIdAndGuildId
{ pub icon : Option<String>,pub guild_id : String,pub id : String,pub flags : i32,pub hoist : bool,pub managed : bool,pub mentionable : bool,pub position : i32,pub color : i64,}pub struct CachedRoleSelectByIdAndGuildIdBorrowed<'a> { pub icon : Option<&'a str>,pub guild_id : &'a str,pub id : &'a str,pub flags : i32,pub hoist : bool,pub managed : bool,pub mentionable : bool,pub position : i32,pub color : i64,}
impl<'a> From<CachedRoleSelectByIdAndGuildIdBorrowed<'a>> for CachedRoleSelectByIdAndGuildId
{
    fn from(CachedRoleSelectByIdAndGuildIdBorrowed { icon,guild_id,id,flags,hoist,managed,mentionable,position,color,}: CachedRoleSelectByIdAndGuildIdBorrowed<'a>) ->
    Self { Self { icon: icon.map(|v| v.into()),guild_id: guild_id.into(),id: id.into(),flags,hoist,managed,mentionable,position,color,} }
}pub struct CachedRoleSelectByIdAndGuildIdQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> CachedRoleSelectByIdAndGuildIdBorrowed,
    mapper: fn(CachedRoleSelectByIdAndGuildIdBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> CachedRoleSelectByIdAndGuildIdQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(CachedRoleSelectByIdAndGuildIdBorrowed) -> R) ->
    CachedRoleSelectByIdAndGuildIdQuery<'a,C,R,N>
    {
        CachedRoleSelectByIdAndGuildIdQuery
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
}pub fn cached_role_select_by_id_and_guild_id() -> CachedRoleSelectByIdAndGuildIdStmt
{ CachedRoleSelectByIdAndGuildIdStmt(cornucopia_async::private::Stmt::new("SELECT
    *
FROM
    \"DiscordFrontend\".\"Nightly\".\"CachedRoles\"
WHERE
    \"id\" = $1 AND
    \"guild_id\" = $2")) } pub struct
CachedRoleSelectByIdAndGuildIdStmt(cornucopia_async::private::Stmt); impl CachedRoleSelectByIdAndGuildIdStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
id: &'a T1,guild_id: &'a T2,) -> CachedRoleSelectByIdAndGuildIdQuery<'a,C,
CachedRoleSelectByIdAndGuildId, 2>
{
    CachedRoleSelectByIdAndGuildIdQuery
    {
        client, params: [id,guild_id,], stmt: &mut self.0, extractor:
        |row| { CachedRoleSelectByIdAndGuildIdBorrowed { icon: row.get(0),guild_id: row.get(1),id: row.get(2),flags: row.get(3),hoist: row.get(4),managed: row.get(5),mentionable: row.get(6),position: row.get(7),color: row.get(8),} }, mapper: |it| { <CachedRoleSelectByIdAndGuildId>::from(it) },
    }
} }impl <'a, C: GenericClient,T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,> cornucopia_async::Params<'a,
CachedRoleSelectByIdAndGuildIdParams<T1,T2,>, CachedRoleSelectByIdAndGuildIdQuery<'a, C,
CachedRoleSelectByIdAndGuildId, 2>, C> for CachedRoleSelectByIdAndGuildIdStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    CachedRoleSelectByIdAndGuildIdParams<T1,T2,>) -> CachedRoleSelectByIdAndGuildIdQuery<'a, C,
    CachedRoleSelectByIdAndGuildId, 2>
    { self.bind(client, &params.id,&params.guild_id,) }
}}pub mod cached_role_upsert
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct CachedRoleUpsertParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,> { pub color: i64,pub icon: Option<T1>,pub id: T2,pub guild_id: T3,pub flags: i32,pub hoist: bool,pub managed: bool,pub mentionable: bool,pub position: i32,}pub fn cached_role_upsert() -> CachedRoleUpsertStmt
{ CachedRoleUpsertStmt(cornucopia_async::private::Stmt::new("INSERT INTO \"DiscordFrontend\".\"Nightly\".\"CachedRoles\" (\"color\", \"icon\", \"id\", \"guild_id\", \"flags\", \"hoist\", \"managed\", \"mentionable\", \"position\")
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
ON CONFLICT (\"id\", \"guild_id\") DO UPDATE
    SET
        \"color\" = $1,
        \"icon\" = $2,
        \"flags\" = $5,
        \"hoist\" = $6,
        \"managed\" = $7,
        \"mentionable\" = $8,
        \"position\" = $9")) } pub struct
CachedRoleUpsertStmt(cornucopia_async::private::Stmt); impl CachedRoleUpsertStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,T3:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
color: &'a i64,icon: &'a Option<T1>,id: &'a T2,guild_id: &'a T3,flags: &'a i32,hoist: &'a bool,managed: &'a bool,mentionable: &'a bool,position: &'a i32,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[color,icon,id,guild_id,flags,hoist,managed,mentionable,position,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,>
cornucopia_async::Params<'a, CachedRoleUpsertParams<T1,T2,T3,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for CachedRoleUpsertStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    CachedRoleUpsertParams<T1,T2,T3,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.color,&params.icon,&params.id,&params.guild_id,&params.flags,&params.hoist,&params.managed,&params.mentionable,&params.position,)) }
}}pub mod cached_user_select_by_id
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug, Clone, PartialEq,)] pub struct CachedUserSelectById
{ pub id : String,pub bot : bool,}pub struct CachedUserSelectByIdBorrowed<'a> { pub id : &'a str,pub bot : bool,}
impl<'a> From<CachedUserSelectByIdBorrowed<'a>> for CachedUserSelectById
{
    fn from(CachedUserSelectByIdBorrowed { id,bot,}: CachedUserSelectByIdBorrowed<'a>) ->
    Self { Self { id: id.into(),bot,} }
}pub struct CachedUserSelectByIdQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> CachedUserSelectByIdBorrowed,
    mapper: fn(CachedUserSelectByIdBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> CachedUserSelectByIdQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(CachedUserSelectByIdBorrowed) -> R) ->
    CachedUserSelectByIdQuery<'a,C,R,N>
    {
        CachedUserSelectByIdQuery
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
}pub fn cached_user_select_by_id() -> CachedUserSelectByIdStmt
{ CachedUserSelectByIdStmt(cornucopia_async::private::Stmt::new("SELECT
    *
FROM
    \"DiscordFrontend\".\"Nightly\".\"CachedUsers\"
WHERE
    \"id\" = $1")) } pub struct
CachedUserSelectByIdStmt(cornucopia_async::private::Stmt); impl CachedUserSelectByIdStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
id: &'a T1,) -> CachedUserSelectByIdQuery<'a,C,
CachedUserSelectById, 1>
{
    CachedUserSelectByIdQuery
    {
        client, params: [id,], stmt: &mut self.0, extractor:
        |row| { CachedUserSelectByIdBorrowed { id: row.get(0),bot: row.get(1),} }, mapper: |it| { <CachedUserSelectById>::from(it) },
    }
} }}pub mod cached_user_upsert
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct CachedUserUpsertParams<T1: cornucopia_async::StringSql,> { pub id: T1,pub bot: bool,}pub fn cached_user_upsert() -> CachedUserUpsertStmt
{ CachedUserUpsertStmt(cornucopia_async::private::Stmt::new("INSERT INTO \"DiscordFrontend\".\"Nightly\".\"CachedUsers\" (id, bot)
VALUES ($1, $2)
ON CONFLICT (\"id\") DO UPDATE
    SET
        \"bot\" = $2")) } pub struct
CachedUserUpsertStmt(cornucopia_async::private::Stmt); impl CachedUserUpsertStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
id: &'a T1,bot: &'a bool,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[id,bot,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,>
cornucopia_async::Params<'a, CachedUserUpsertParams<T1,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for CachedUserUpsertStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    CachedUserUpsertParams<T1,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.id,&params.bot,)) }
}}}