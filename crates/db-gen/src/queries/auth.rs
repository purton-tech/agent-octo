// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct UpsertUserByIssuerSubParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
> {
    pub issuer: T1,
    pub sub: T2,
    pub email: T3,
    pub first_name: Option<T4>,
    pub last_name: Option<T5>,
}
#[derive(Debug)]
pub struct EnsureDefaultOrgMembershipForUserParams<T1: crate::StringSql> {
    pub user_id: uuid::Uuid,
    pub org_name: T1,
}
#[derive(Debug, Clone, PartialEq)]
pub struct AuthUser {
    pub id: uuid::Uuid,
    pub issuer: String,
    pub sub: String,
    pub email: String,
}
pub struct AuthUserBorrowed<'a> {
    pub id: uuid::Uuid,
    pub issuer: &'a str,
    pub sub: &'a str,
    pub email: &'a str,
}
impl<'a> From<AuthUserBorrowed<'a>> for AuthUser {
    fn from(
        AuthUserBorrowed {
            id,
            issuer,
            sub,
            email,
        }: AuthUserBorrowed<'a>,
    ) -> Self {
        Self {
            id,
            issuer: issuer.into(),
            sub: sub.into(),
            email: email.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct EnsureOrgMembership {
    pub ensured: bool,
}
#[derive(Debug, Clone, PartialEq)]
pub struct UserOrg {
    pub org_id: uuid::Uuid,
    pub org_public_id: String,
}
pub struct UserOrgBorrowed<'a> {
    pub org_id: uuid::Uuid,
    pub org_public_id: &'a str,
}
impl<'a> From<UserOrgBorrowed<'a>> for UserOrg {
    fn from(
        UserOrgBorrowed {
            org_id,
            org_public_id,
        }: UserOrgBorrowed<'a>,
    ) -> Self {
        Self {
            org_id,
            org_public_id: org_public_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
}
pub struct UserBorrowed<'a> {
    pub id: uuid::Uuid,
    pub email: &'a str,
}
impl<'a> From<UserBorrowed<'a>> for User {
    fn from(UserBorrowed { id, email }: UserBorrowed<'a>) -> Self {
        Self {
            id,
            email: email.into(),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct AuthUserQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<AuthUserBorrowed, tokio_postgres::Error>,
    mapper: fn(AuthUserBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> AuthUserQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(AuthUserBorrowed) -> R) -> AuthUserQuery<'c, 'a, 's, C, R, N> {
        AuthUserQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct EnsureOrgMembershipQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<EnsureOrgMembership, tokio_postgres::Error>,
    mapper: fn(EnsureOrgMembership) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> EnsureOrgMembershipQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(EnsureOrgMembership) -> R,
    ) -> EnsureOrgMembershipQuery<'c, 'a, 's, C, R, N> {
        EnsureOrgMembershipQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct UserOrgQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<UserOrgBorrowed, tokio_postgres::Error>,
    mapper: fn(UserOrgBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> UserOrgQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(UserOrgBorrowed) -> R) -> UserOrgQuery<'c, 'a, 's, C, R, N> {
        UserOrgQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct StringQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<&str, tokio_postgres::Error>,
    mapper: fn(&str) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> StringQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(&str) -> R) -> StringQuery<'c, 'a, 's, C, R, N> {
        StringQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct UserQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    query: &'static str,
    cached: Option<&'s tokio_postgres::Statement>,
    extractor: fn(&tokio_postgres::Row) -> Result<UserBorrowed, tokio_postgres::Error>,
    mapper: fn(UserBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> UserQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(UserBorrowed) -> R) -> UserQuery<'c, 'a, 's, C, R, N> {
        UserQuery {
            client: self.client,
            params: self.params,
            query: self.query,
            cached: self.cached,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let row =
            crate::client::async_::one(self.client, self.query, &self.params, self.cached).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let opt_row =
            crate::client::async_::opt(self.client, self.query, &self.params, self.cached).await?;
        Ok(opt_row
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stream = crate::client::async_::raw(
            self.client,
            self.query,
            crate::slice_iter(&self.params),
            self.cached,
        )
        .await?;
        let mapped = stream
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(mapped)
    }
}
pub struct UpsertUserByIssuerSubStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn upsert_user_by_issuer_sub() -> UpsertUserByIssuerSubStmt {
    UpsertUserByIssuerSubStmt(
        "INSERT INTO auth.users ( issuer, sub, email, first_name, last_name ) VALUES ( $1::TEXT, $2::TEXT, $3::TEXT, $4::TEXT, $5::TEXT ) ON CONFLICT (issuer, sub) DO UPDATE SET email = EXCLUDED.email, first_name = EXCLUDED.first_name, last_name = EXCLUDED.last_name, updated_at = NOW() RETURNING id, issuer, sub, email",
        None,
    )
}
impl UpsertUserByIssuerSubStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
        T5: crate::StringSql,
    >(
        &'s self,
        client: &'c C,
        issuer: &'a T1,
        sub: &'a T2,
        email: &'a T3,
        first_name: &'a Option<T4>,
        last_name: &'a Option<T5>,
    ) -> AuthUserQuery<'c, 'a, 's, C, AuthUser, 5> {
        AuthUserQuery {
            client,
            params: [issuer, sub, email, first_name, last_name],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<AuthUserBorrowed, tokio_postgres::Error> {
                    Ok(AuthUserBorrowed {
                        id: row.try_get(0)?,
                        issuer: row.try_get(1)?,
                        sub: row.try_get(2)?,
                        email: row.try_get(3)?,
                    })
                },
            mapper: |it| AuthUser::from(it),
        }
    }
}
impl<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
        T5: crate::StringSql,
    >
    crate::client::async_::Params<
        'c,
        'a,
        's,
        UpsertUserByIssuerSubParams<T1, T2, T3, T4, T5>,
        AuthUserQuery<'c, 'a, 's, C, AuthUser, 5>,
        C,
    > for UpsertUserByIssuerSubStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a UpsertUserByIssuerSubParams<T1, T2, T3, T4, T5>,
    ) -> AuthUserQuery<'c, 'a, 's, C, AuthUser, 5> {
        self.bind(
            client,
            &params.issuer,
            &params.sub,
            &params.email,
            &params.first_name,
            &params.last_name,
        )
    }
}
pub struct GetCurrentUserStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_current_user() -> GetCurrentUserStmt {
    GetCurrentUserStmt(
        "SELECT id, issuer, sub, email FROM auth.users WHERE id = auth.uid()",
        None,
    )
}
impl GetCurrentUserStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
    ) -> AuthUserQuery<'c, 'a, 's, C, AuthUser, 0> {
        AuthUserQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<AuthUserBorrowed, tokio_postgres::Error> {
                    Ok(AuthUserBorrowed {
                        id: row.try_get(0)?,
                        issuer: row.try_get(1)?,
                        sub: row.try_get(2)?,
                        email: row.try_get(3)?,
                    })
                },
            mapper: |it| AuthUser::from(it),
        }
    }
}
pub struct EnsureDefaultOrgMembershipForUserStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn ensure_default_org_membership_for_user() -> EnsureDefaultOrgMembershipForUserStmt {
    EnsureDefaultOrgMembershipForUserStmt(
        "WITH has_membership AS ( SELECT 1 FROM org.org_memberships WHERE user_id = $1::UUID LIMIT 1 ), inserted_org AS ( INSERT INTO org.orgs (name) SELECT $2::TEXT WHERE NOT EXISTS (SELECT 1 FROM has_membership) RETURNING id ), inserted_membership AS ( INSERT INTO org.org_memberships (org_id, user_id, role) SELECT io.id, $1::UUID, 'owner'::org.org_role FROM inserted_org io ON CONFLICT (org_id, user_id) DO NOTHING RETURNING 1 ) SELECT EXISTS(SELECT 1 FROM inserted_membership) AS ensured",
        None,
    )
}
impl EnsureDefaultOrgMembershipForUserStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        user_id: &'a uuid::Uuid,
        org_name: &'a T1,
    ) -> EnsureOrgMembershipQuery<'c, 'a, 's, C, EnsureOrgMembership, 2> {
        EnsureOrgMembershipQuery {
            client,
            params: [user_id, org_name],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<EnsureOrgMembership, tokio_postgres::Error> {
                    Ok(EnsureOrgMembership {
                        ensured: row.try_get(0)?,
                    })
                },
            mapper: |it| EnsureOrgMembership::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        EnsureDefaultOrgMembershipForUserParams<T1>,
        EnsureOrgMembershipQuery<'c, 'a, 's, C, EnsureOrgMembership, 2>,
        C,
    > for EnsureDefaultOrgMembershipForUserStmt
{
    fn params(
        &'s self,
        client: &'c C,
        params: &'a EnsureDefaultOrgMembershipForUserParams<T1>,
    ) -> EnsureOrgMembershipQuery<'c, 'a, 's, C, EnsureOrgMembership, 2> {
        self.bind(client, &params.user_id, &params.org_name)
    }
}
pub struct GetFirstOrgForUserStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_first_org_for_user() -> GetFirstOrgForUserStmt {
    GetFirstOrgForUserStmt(
        "SELECT org_id, public.uuid_to_b64url(org_id) AS org_public_id FROM org.org_memberships WHERE user_id = $1::UUID ORDER BY joined_at ASC LIMIT 1",
        None,
    )
}
impl GetFirstOrgForUserStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
        user_id: &'a uuid::Uuid,
    ) -> UserOrgQuery<'c, 'a, 's, C, UserOrg, 1> {
        UserOrgQuery {
            client,
            params: [user_id],
            query: self.0,
            cached: self.1.as_ref(),
            extractor:
                |row: &tokio_postgres::Row| -> Result<UserOrgBorrowed, tokio_postgres::Error> {
                    Ok(UserOrgBorrowed {
                        org_id: row.try_get(0)?,
                        org_public_id: row.try_get(1)?,
                    })
                },
            mapper: |it| UserOrg::from(it),
        }
    }
}
pub struct SetRequestClaimSubStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn set_request_claim_sub() -> SetRequestClaimSubStmt {
    SetRequestClaimSubStmt(
        "SELECT set_config( 'request.jwt.claim.sub', $1::TEXT, true )",
        None,
    )
}
impl SetRequestClaimSubStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        claim_sub: &'a T1,
    ) -> StringQuery<'c, 'a, 's, C, String, 1> {
        StringQuery {
            client,
            params: [claim_sub],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.into(),
        }
    }
}
pub struct SetRequestClaimIssStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn set_request_claim_iss() -> SetRequestClaimIssStmt {
    SetRequestClaimIssStmt(
        "SELECT set_config( 'request.jwt.claim.iss', $1::TEXT, true )",
        None,
    )
}
impl SetRequestClaimIssStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        claim_iss: &'a T1,
    ) -> StringQuery<'c, 'a, 's, C, String, 1> {
        StringQuery {
            client,
            params: [claim_iss],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.into(),
        }
    }
}
pub struct SetRequestClaimExternalSubStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn set_request_claim_external_sub() -> SetRequestClaimExternalSubStmt {
    SetRequestClaimExternalSubStmt(
        "SELECT set_config( 'request.jwt.claim.external_sub', $1::TEXT, true )",
        None,
    )
}
impl SetRequestClaimExternalSubStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s self,
        client: &'c C,
        claim_external_sub: &'a T1,
    ) -> StringQuery<'c, 'a, 's, C, String, 1> {
        StringQuery {
            client,
            params: [claim_external_sub],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row| Ok(row.try_get(0)?),
            mapper: |it| it.into(),
        }
    }
}
pub struct GetUsersStmt(&'static str, Option<tokio_postgres::Statement>);
pub fn get_users() -> GetUsersStmt {
    GetUsersStmt("SELECT id, email FROM auth.users", None)
}
impl GetUsersStmt {
    pub async fn prepare<'a, C: GenericClient>(
        mut self,
        client: &'a C,
    ) -> Result<Self, tokio_postgres::Error> {
        self.1 = Some(client.prepare(self.0).await?);
        Ok(self)
    }
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s self,
        client: &'c C,
    ) -> UserQuery<'c, 'a, 's, C, User, 0> {
        UserQuery {
            client,
            params: [],
            query: self.0,
            cached: self.1.as_ref(),
            extractor: |row: &tokio_postgres::Row| -> Result<UserBorrowed, tokio_postgres::Error> {
                Ok(UserBorrowed {
                    id: row.try_get(0)?,
                    email: row.try_get(1)?,
                })
            },
            mapper: |it| User::from(it),
        }
    }
}
