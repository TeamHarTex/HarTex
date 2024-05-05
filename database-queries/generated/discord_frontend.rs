// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod cached_emoji_select_by_guild_id
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug, Clone, PartialEq,)] pub struct CachedEmojiSelectByGuildId
{ pub id : String,pub guild_id : String,pub name : String,pub animated : bool,pub managed : bool,}pub struct CachedEmojiSelectByGuildIdBorrowed<'a> { pub id : &'a str,pub guild_id : &'a str,pub name : &'a str,pub animated : bool,pub managed : bool,}
impl<'a> From<CachedEmojiSelectByGuildIdBorrowed<'a>> for CachedEmojiSelectByGuildId
{
    fn from(CachedEmojiSelectByGuildIdBorrowed { id,guild_id,name,animated,managed,}: CachedEmojiSelectByGuildIdBorrowed<'a>) ->
    Self { Self { id: id.into(),guild_id: guild_id.into(),name: name.into(),animated,managed,} }
}pub struct CachedEmojiSelectByGuildIdQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> CachedEmojiSelectByGuildIdBorrowed,
    mapper: fn(CachedEmojiSelectByGuildIdBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> CachedEmojiSelectByGuildIdQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(CachedEmojiSelectByGuildIdBorrowed) -> R) ->
    CachedEmojiSelectByGuildIdQuery<'a,C,R,N>
    {
        CachedEmojiSelectByGuildIdQuery
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
}pub fn cached_emoji_select_by_guild_id() -> CachedEmojiSelectByGuildIdStmt
{ CachedEmojiSelectByGuildIdStmt(cornucopia_async::private::Stmt::new("SELECT
    *
FROM
    \"DiscordFrontend\".\"Nightly\".\"CachedEmojis\"
WHERE
    \"guild_id\" = $1")) } pub struct
CachedEmojiSelectByGuildIdStmt(cornucopia_async::private::Stmt); impl CachedEmojiSelectByGuildIdStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
guild_id: &'a T1,) -> CachedEmojiSelectByGuildIdQuery<'a,C,
CachedEmojiSelectByGuildId, 1>
{
    CachedEmojiSelectByGuildIdQuery
    {
        client, params: [guild_id,], stmt: &mut self.0, extractor:
        |row| { CachedEmojiSelectByGuildIdBorrowed { id: row.get(0),guild_id: row.get(1),name: row.get(2),animated: row.get(3),managed: row.get(4),} }, mapper: |it| { <CachedEmojiSelectByGuildId>::from(it) },
    }
} }}pub mod cached_emoji_select_by_id
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug, Clone, PartialEq,)] pub struct CachedEmojiSelectById
{ pub id : String,pub guild_id : String,pub name : String,pub animated : bool,pub managed : bool,}pub struct CachedEmojiSelectByIdBorrowed<'a> { pub id : &'a str,pub guild_id : &'a str,pub name : &'a str,pub animated : bool,pub managed : bool,}
impl<'a> From<CachedEmojiSelectByIdBorrowed<'a>> for CachedEmojiSelectById
{
    fn from(CachedEmojiSelectByIdBorrowed { id,guild_id,name,animated,managed,}: CachedEmojiSelectByIdBorrowed<'a>) ->
    Self { Self { id: id.into(),guild_id: guild_id.into(),name: name.into(),animated,managed,} }
}pub struct CachedEmojiSelectByIdQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> CachedEmojiSelectByIdBorrowed,
    mapper: fn(CachedEmojiSelectByIdBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> CachedEmojiSelectByIdQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(CachedEmojiSelectByIdBorrowed) -> R) ->
    CachedEmojiSelectByIdQuery<'a,C,R,N>
    {
        CachedEmojiSelectByIdQuery
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
}pub fn cached_emoji_select_by_id() -> CachedEmojiSelectByIdStmt
{ CachedEmojiSelectByIdStmt(cornucopia_async::private::Stmt::new("SELECT
    *
FROM
    \"DiscordFrontend\".\"Nightly\".\"CachedEmojis\"
WHERE
    \"id\" = $1")) } pub struct
CachedEmojiSelectByIdStmt(cornucopia_async::private::Stmt); impl CachedEmojiSelectByIdStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
id: &'a T1,) -> CachedEmojiSelectByIdQuery<'a,C,
CachedEmojiSelectById, 1>
{
    CachedEmojiSelectByIdQuery
    {
        client, params: [id,], stmt: &mut self.0, extractor:
        |row| { CachedEmojiSelectByIdBorrowed { id: row.get(0),guild_id: row.get(1),name: row.get(2),animated: row.get(3),managed: row.get(4),} }, mapper: |it| { <CachedEmojiSelectById>::from(it) },
    }
} }}pub mod cached_emoji_upsert
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct CachedEmojiUpsertParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,> { pub animated: bool,pub name: T1,pub id: T2,pub guild_id: T3,pub managed: bool,}pub fn cached_emoji_upsert() -> CachedEmojiUpsertStmt
{ CachedEmojiUpsertStmt(cornucopia_async::private::Stmt::new("INSERT INTO \"DiscordFrontend\".\"Nightly\".\"CachedEmojis\" (\"animated\", \"name\", \"id\", \"guild_id\", \"managed\")
VALUES ($1, $2, $3, $4, $5)
ON CONFLICT (\"id\") DO UPDATE
    SET
        \"guild_id\" = $4,
        \"animated\" = $1,
        \"name\" = $2,
        \"managed\" = $5")) } pub struct
CachedEmojiUpsertStmt(cornucopia_async::private::Stmt); impl CachedEmojiUpsertStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,T3:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
animated: &'a bool,name: &'a T1,id: &'a T2,guild_id: &'a T3,managed: &'a bool,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[animated,name,id,guild_id,managed,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,>
cornucopia_async::Params<'a, CachedEmojiUpsertParams<T1,T2,T3,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for CachedEmojiUpsertStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    CachedEmojiUpsertParams<T1,T2,T3,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.animated,&params.name,&params.id,&params.guild_id,&params.managed,)) }
}}pub mod cached_guild_select_by_id
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug, Clone, PartialEq,)] pub struct CachedGuildSelectById
{ pub default_message_notifications : i16,pub explicit_content_filter : i16,pub features : Vec<String>,pub icon : Option<String>,pub id : String,pub large : bool,pub name : String,pub owner_id : String,pub mfa_level : i16,pub premium_subscription_count : Option<i64>,pub premium_tier : i16,pub verification_level : i16,}pub struct CachedGuildSelectByIdBorrowed<'a> { pub default_message_notifications : i16,pub explicit_content_filter : i16,pub features : cornucopia_async::ArrayIterator<'a, &'a str>,pub icon : Option<&'a str>,pub id : &'a str,pub large : bool,pub name : &'a str,pub owner_id : &'a str,pub mfa_level : i16,pub premium_subscription_count : Option<i64>,pub premium_tier : i16,pub verification_level : i16,}
impl<'a> From<CachedGuildSelectByIdBorrowed<'a>> for CachedGuildSelectById
{
    fn from(CachedGuildSelectByIdBorrowed { default_message_notifications,explicit_content_filter,features,icon,id,large,name,owner_id,mfa_level,premium_subscription_count,premium_tier,verification_level,}: CachedGuildSelectByIdBorrowed<'a>) ->
    Self { Self { default_message_notifications,explicit_content_filter,features: features.map(|v| v.into()).collect(),icon: icon.map(|v| v.into()),id: id.into(),large,name: name.into(),owner_id: owner_id.into(),mfa_level,premium_subscription_count,premium_tier,verification_level,} }
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
        |row| { CachedGuildSelectByIdBorrowed { default_message_notifications: row.get(0),explicit_content_filter: row.get(1),features: row.get(2),icon: row.get(3),id: row.get(4),large: row.get(5),name: row.get(6),owner_id: row.get(7),mfa_level: row.get(8),premium_subscription_count: row.get(9),premium_tier: row.get(10),verification_level: row.get(11),} }, mapper: |it| { <CachedGuildSelectById>::from(it) },
    }
} }}pub mod cached_guild_upsert
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct CachedGuildUpsertParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::ArraySql<Item = T1>,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,> { pub default_message_notifications: i16,pub explicit_content_filter: i16,pub features: T2,pub icon: Option<T3>,pub large: bool,pub name: T4,pub owner_id: T5,pub id: T6,pub mfa_level: i16,pub premium_subscription_count: Option<i64>,pub premium_tier: i16,pub verification_level: i16,}pub fn cached_guild_upsert() -> CachedGuildUpsertStmt
{ CachedGuildUpsertStmt(cornucopia_async::private::Stmt::new("INSERT INTO
    \"DiscordFrontend\".\"Nightly\".\"CachedGuilds\" (\"default_message_notifications\", \"explicit_content_filter\", \"features\", \"icon\", \"large\", \"name\", \"owner_id\", \"id\", \"mfa_level\", \"premium_subscription_count\", \"premium_tier\", \"verification_level\")
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
ON CONFLICT (\"id\") DO UPDATE
    SET
        \"default_message_notifications\" = $1,
        \"explicit_content_filter\" = $2,
        \"features\" = $3,
        \"icon\" = $4,
        \"large\" = $5,
        \"mfa_level\" = $9,
        \"name\" = $6,
        \"owner_id\" = $7,
        \"premium_subscription_count\" = $10,
        \"premium_tier\" = $11,
        \"verification_level\" = $12")) } pub struct
CachedGuildUpsertStmt(cornucopia_async::private::Stmt); impl CachedGuildUpsertStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::ArraySql<Item = T1>,T3:
cornucopia_async::StringSql,T4:
cornucopia_async::StringSql,T5:
cornucopia_async::StringSql,T6:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
default_message_notifications: &'a i16,explicit_content_filter: &'a i16,features: &'a T2,icon: &'a Option<T3>,large: &'a bool,name: &'a T4,owner_id: &'a T5,id: &'a T6,mfa_level: &'a i16,premium_subscription_count: &'a Option<i64>,premium_tier: &'a i16,verification_level: &'a i16,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[default_message_notifications,explicit_content_filter,features,icon,large,name,owner_id,id,mfa_level,premium_subscription_count,premium_tier,verification_level,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,T2: cornucopia_async::ArraySql<Item = T1>,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,>
cornucopia_async::Params<'a, CachedGuildUpsertParams<T1,T2,T3,T4,T5,T6,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for CachedGuildUpsertStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    CachedGuildUpsertParams<T1,T2,T3,T4,T5,T6,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.default_message_notifications,&params.explicit_content_filter,&params.features,&params.icon,&params.large,&params.name,&params.owner_id,&params.id,&params.mfa_level,&params.premium_subscription_count,&params.premium_tier,&params.verification_level,)) }
}}pub mod cached_member_select_by_guild_id
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug, Clone, PartialEq,)] pub struct CachedMemberSelectByGuildId
{ pub guild_id : String,pub user_id : String,pub roles : Vec<String>,pub nick : Option<String>,pub joined_at : Option<time::OffsetDateTime>,pub flags : i64,}pub struct CachedMemberSelectByGuildIdBorrowed<'a> { pub guild_id : &'a str,pub user_id : &'a str,pub roles : cornucopia_async::ArrayIterator<'a, &'a str>,pub nick : Option<&'a str>,pub joined_at : Option<time::OffsetDateTime>,pub flags : i64,}
impl<'a> From<CachedMemberSelectByGuildIdBorrowed<'a>> for CachedMemberSelectByGuildId
{
    fn from(CachedMemberSelectByGuildIdBorrowed { guild_id,user_id,roles,nick,joined_at,flags,}: CachedMemberSelectByGuildIdBorrowed<'a>) ->
    Self { Self { guild_id: guild_id.into(),user_id: user_id.into(),roles: roles.map(|v| v.into()).collect(),nick: nick.map(|v| v.into()),joined_at,flags,} }
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
        |row| { CachedMemberSelectByGuildIdBorrowed { guild_id: row.get(0),user_id: row.get(1),roles: row.get(2),nick: row.get(3),joined_at: row.get(4),flags: row.get(5),} }, mapper: |it| { <CachedMemberSelectByGuildId>::from(it) },
    }
} }}pub mod cached_member_select_by_user_id_and_guild_id
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct CachedMemberSelectByUserIdAndGuildIdParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,> { pub user_id: T1,pub guild_id: T2,}#[derive( Debug, Clone, PartialEq,)] pub struct CachedMemberSelectByUserIdAndGuildId
{ pub guild_id : String,pub user_id : String,pub roles : Vec<String>,pub nick : Option<String>,pub joined_at : Option<time::OffsetDateTime>,pub flags : i64,}pub struct CachedMemberSelectByUserIdAndGuildIdBorrowed<'a> { pub guild_id : &'a str,pub user_id : &'a str,pub roles : cornucopia_async::ArrayIterator<'a, &'a str>,pub nick : Option<&'a str>,pub joined_at : Option<time::OffsetDateTime>,pub flags : i64,}
impl<'a> From<CachedMemberSelectByUserIdAndGuildIdBorrowed<'a>> for CachedMemberSelectByUserIdAndGuildId
{
    fn from(CachedMemberSelectByUserIdAndGuildIdBorrowed { guild_id,user_id,roles,nick,joined_at,flags,}: CachedMemberSelectByUserIdAndGuildIdBorrowed<'a>) ->
    Self { Self { guild_id: guild_id.into(),user_id: user_id.into(),roles: roles.map(|v| v.into()).collect(),nick: nick.map(|v| v.into()),joined_at,flags,} }
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
        |row| { CachedMemberSelectByUserIdAndGuildIdBorrowed { guild_id: row.get(0),user_id: row.get(1),roles: row.get(2),nick: row.get(3),joined_at: row.get(4),flags: row.get(5),} }, mapper: |it| { <CachedMemberSelectByUserIdAndGuildId>::from(it) },
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
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct CachedMemberUpsertParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::ArraySql<Item = T4>,> { pub flags: i64,pub joined_at: Option<time::OffsetDateTime>,pub nick: Option<T1>,pub user_id: T2,pub guild_id: T3,pub roles: T5,}pub fn cached_member_upsert() -> CachedMemberUpsertStmt
{ CachedMemberUpsertStmt(cornucopia_async::private::Stmt::new("INSERT INTO \"DiscordFrontend\".\"Nightly\".\"CachedMembers\" (\"flags\", \"joined_at\", \"nick\", \"user_id\", \"guild_id\", \"roles\")
VALUES ($1, $2, $3, $4, $5, $6)
ON CONFLICT (\"user_id\", \"guild_id\") DO UPDATE
    SET
        \"flags\" = $1,
        \"joined_at\" = $2,
        \"nick\" = $3,
        \"roles\" = $6")) } pub struct
CachedMemberUpsertStmt(cornucopia_async::private::Stmt); impl CachedMemberUpsertStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,T3:
cornucopia_async::StringSql,T4:
cornucopia_async::StringSql,T5:
cornucopia_async::ArraySql<Item = T4>,>(&'a mut self, client: &'a  C,
flags: &'a i64,joined_at: &'a Option<time::OffsetDateTime>,nick: &'a Option<T1>,user_id: &'a T2,guild_id: &'a T3,roles: &'a T5,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[flags,joined_at,nick,user_id,guild_id,roles,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::ArraySql<Item = T4>,>
cornucopia_async::Params<'a, CachedMemberUpsertParams<T1,T2,T3,T4,T5,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for CachedMemberUpsertStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    CachedMemberUpsertParams<T1,T2,T3,T4,T5,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.flags,&params.joined_at,&params.nick,&params.user_id,&params.guild_id,&params.roles,)) }
}}pub mod cached_role_select_by_guild_id
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug, Clone, PartialEq,)] pub struct CachedRoleSelectByGuildId
{ pub color : i64,pub icon : Option<String>,pub id : String,pub guild_id : String,pub hoist : bool,pub mentionable : bool,pub flags : i32,pub managed : bool,pub position : i32,}pub struct CachedRoleSelectByGuildIdBorrowed<'a> { pub color : i64,pub icon : Option<&'a str>,pub id : &'a str,pub guild_id : &'a str,pub hoist : bool,pub mentionable : bool,pub flags : i32,pub managed : bool,pub position : i32,}
impl<'a> From<CachedRoleSelectByGuildIdBorrowed<'a>> for CachedRoleSelectByGuildId
{
    fn from(CachedRoleSelectByGuildIdBorrowed { color,icon,id,guild_id,hoist,mentionable,flags,managed,position,}: CachedRoleSelectByGuildIdBorrowed<'a>) ->
    Self { Self { color,icon: icon.map(|v| v.into()),id: id.into(),guild_id: guild_id.into(),hoist,mentionable,flags,managed,position,} }
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
        |row| { CachedRoleSelectByGuildIdBorrowed { color: row.get(0),icon: row.get(1),id: row.get(2),guild_id: row.get(3),hoist: row.get(4),mentionable: row.get(5),flags: row.get(6),managed: row.get(7),position: row.get(8),} }, mapper: |it| { <CachedRoleSelectByGuildId>::from(it) },
    }
} }}pub mod cached_role_select_by_id_and_guild_id
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct CachedRoleSelectByIdAndGuildIdParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,> { pub id: T1,pub guild_id: T2,}#[derive( Debug, Clone, PartialEq,)] pub struct CachedRoleSelectByIdAndGuildId
{ pub color : i64,pub icon : Option<String>,pub id : String,pub guild_id : String,pub hoist : bool,pub mentionable : bool,pub flags : i32,pub managed : bool,pub position : i32,}pub struct CachedRoleSelectByIdAndGuildIdBorrowed<'a> { pub color : i64,pub icon : Option<&'a str>,pub id : &'a str,pub guild_id : &'a str,pub hoist : bool,pub mentionable : bool,pub flags : i32,pub managed : bool,pub position : i32,}
impl<'a> From<CachedRoleSelectByIdAndGuildIdBorrowed<'a>> for CachedRoleSelectByIdAndGuildId
{
    fn from(CachedRoleSelectByIdAndGuildIdBorrowed { color,icon,id,guild_id,hoist,mentionable,flags,managed,position,}: CachedRoleSelectByIdAndGuildIdBorrowed<'a>) ->
    Self { Self { color,icon: icon.map(|v| v.into()),id: id.into(),guild_id: guild_id.into(),hoist,mentionable,flags,managed,position,} }
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
        |row| { CachedRoleSelectByIdAndGuildIdBorrowed { color: row.get(0),icon: row.get(1),id: row.get(2),guild_id: row.get(3),hoist: row.get(4),mentionable: row.get(5),flags: row.get(6),managed: row.get(7),position: row.get(8),} }, mapper: |it| { <CachedRoleSelectByIdAndGuildId>::from(it) },
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
{ pub id : String,pub bot : bool,pub discriminator : String,pub name : String,pub global_name : Option<String>,pub avatar : Option<String>,}pub struct CachedUserSelectByIdBorrowed<'a> { pub id : &'a str,pub bot : bool,pub discriminator : &'a str,pub name : &'a str,pub global_name : Option<&'a str>,pub avatar : Option<&'a str>,}
impl<'a> From<CachedUserSelectByIdBorrowed<'a>> for CachedUserSelectById
{
    fn from(CachedUserSelectByIdBorrowed { id,bot,discriminator,name,global_name,avatar,}: CachedUserSelectByIdBorrowed<'a>) ->
    Self { Self { id: id.into(),bot,discriminator: discriminator.into(),name: name.into(),global_name: global_name.map(|v| v.into()),avatar: avatar.map(|v| v.into()),} }
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
        |row| { CachedUserSelectByIdBorrowed { id: row.get(0),bot: row.get(1),discriminator: row.get(2),name: row.get(3),global_name: row.get(4),avatar: row.get(5),} }, mapper: |it| { <CachedUserSelectById>::from(it) },
    }
} }}pub mod cached_user_upsert
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct CachedUserUpsertParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,> { pub avatar: Option<T1>,pub id: T2,pub bot: bool,pub name: T3,pub discriminator: T4,pub global_name: Option<T5>,}pub fn cached_user_upsert() -> CachedUserUpsertStmt
{ CachedUserUpsertStmt(cornucopia_async::private::Stmt::new("INSERT INTO \"DiscordFrontend\".\"Nightly\".\"CachedUsers\" (\"avatar\", \"id\", \"bot\", \"name\", \"discriminator\", \"global_name\")
VALUES ($1, $2, $3, $4, $5, $6)
ON CONFLICT (\"id\") DO UPDATE
    SET
        \"avatar\" = $1,
        \"bot\" = $3,
        \"name\" = $4,
        \"discriminator\" = $5,
        \"global_name\" = $6")) } pub struct
CachedUserUpsertStmt(cornucopia_async::private::Stmt); impl CachedUserUpsertStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,T3:
cornucopia_async::StringSql,T4:
cornucopia_async::StringSql,T5:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
avatar: &'a Option<T1>,id: &'a T2,bot: &'a bool,name: &'a T3,discriminator: &'a T4,global_name: &'a Option<T5>,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[avatar,id,bot,name,discriminator,global_name,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,>
cornucopia_async::Params<'a, CachedUserUpsertParams<T1,T2,T3,T4,T5,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for CachedUserUpsertStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    CachedUserUpsertParams<T1,T2,T3,T4,T5,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.avatar,&params.id,&params.bot,&params.name,&params.discriminator,&params.global_name,)) }
}}}