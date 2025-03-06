//! Submodule providing the `Deletable` trait and the `User` struct.

use diesel::{Connection, Identifiable};

/// Trait representing an object that can be inserted into a table.
pub trait Deletable: diesel::Identifiable + Copy
where
    Self::Table: diesel::Table
        + 'static
        + diesel::query_dsl::methods::FindDsl<<Self as Identifiable>::Id>
        + diesel::query_builder::QueryId,
    diesel::helper_types::Find<Self::Table, <Self as Identifiable>::Id>:
        diesel::query_builder::IntoUpdateTarget<Table = Self::Table>,
{
    fn delete<C: diesel::Connection>(&self, conn: &mut C) -> Result<bool, diesel::result::Error>
    where
        <diesel::helper_types::Find<Self::Table, <Self as Identifiable>::Id> as diesel::query_builder::IntoUpdateTarget>::WhereClause:
            diesel::query_builder::QueryFragment<C::Backend> + diesel::query_builder::QueryId,
        <Self::Table as diesel::QuerySource>::FromClause: diesel::query_builder::QueryFragment<C::Backend>,
        C::Backend: diesel::backend::DieselReserveSpecialization
    {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        diesel::delete(Self::table().find(self.id()))
            .execute(conn)
            .map(|rows_deleted| rows_deleted > 0)
    }
}

impl<U> Deletable for U
where
    U: diesel::Identifiable + Copy,
    U::Table: diesel::Table
        + 'static
        + diesel::query_dsl::methods::FindDsl<<U as Identifiable>::Id>
        + diesel::query_builder::QueryId,
    diesel::helper_types::Find<U::Table, <U as Identifiable>::Id>:
        diesel::query_builder::IntoUpdateTarget<Table = U::Table>,
{
}

diesel::table! {
    /// Small table example for testing purpouses
    users (id) {
        id -> Integer,
        name -> Text
    }
}

#[derive(
    diesel::Queryable,
    diesel::QueryableByName,
    PartialEq,
    Eq,
    diesel::Identifiable,
    diesel::Selectable,
    Debug,
    Clone,
    Hash,
)]
#[diesel(table_name = users)]
#[diesel(primary_key(id))]
pub struct User {
    id: i32,
    name: String,
}

impl User {
    pub fn delete_me<C: Connection>(&self, conn: &mut C) -> Result<bool, diesel::result::Error>
    where
        C::Backend: diesel::backend::DieselReserveSpecialization,
        i32: diesel::serialize::ToSql<diesel::sql_types::Integer, C::Backend>,
    {
        self.delete(conn)
    }
}
