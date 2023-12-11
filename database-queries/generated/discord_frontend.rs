// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod cached_guild_upsert
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct CachedGuildUpsertParams < T1 : cornucopia_async::StringSql,T2 : cornucopia_async::ArraySql<Item = T1>,T3 : cornucopia_async::StringSql,T4 : cornucopia_async::StringSql,T5 : cornucopia_async::StringSql,T6 : cornucopia_async::StringSql,> { pub default_message_notifications : i32,pub explicit_content_filter : i32,pub features : T2,pub icon : T3,pub large : bool,pub name : T4,pub owner_id : T5,pub id : T6,}pub fn cached_guild_upsert() -> CachedGuildUpsertStmt
{ CachedGuildUpsertStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO
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
        \"owner_id\" = $7")) } pub
struct CachedGuildUpsertStmt(cornucopia_async :: private :: Stmt) ; impl
CachedGuildUpsertStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::ArraySql<Item = T1>,T3 : cornucopia_async::StringSql,T4 : cornucopia_async::StringSql,T5 : cornucopia_async::StringSql,T6 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
default_message_notifications : & 'a i32,explicit_content_filter : & 'a i32,features : & 'a T2,icon : & 'a T3,large : & 'a bool,name : & 'a T4,owner_id : & 'a T5,id : & 'a T6,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [default_message_notifications,explicit_content_filter,features,icon,large,name,owner_id,id,]) .await
} }impl < 'a, C : GenericClient + Send + Sync, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::ArraySql<Item = T1>,T3 : cornucopia_async::StringSql,T4 : cornucopia_async::StringSql,T5 : cornucopia_async::StringSql,T6 : cornucopia_async::StringSql,>
cornucopia_async :: Params < 'a, CachedGuildUpsertParams < T1,T2,T3,T4,T5,T6,>, std::pin::Pin<Box<dyn futures::Future<Output = Result <
u64, tokio_postgres :: Error > > + Send + 'a>>, C > for CachedGuildUpsertStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    CachedGuildUpsertParams < T1,T2,T3,T4,T5,T6,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result < u64, tokio_postgres ::
    Error > > + Send + 'a>> { Box::pin(self.bind(client, & params.default_message_notifications,& params.explicit_content_filter,& params.features,& params.icon,& params.large,& params.name,& params.owner_id,& params.id,) ) }
}}}