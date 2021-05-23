use crate::query::Select;
use crate::ToSql;
use async_trait::async_trait;

pub trait Row {
    type Error;
}

#[async_trait(?Send)]
pub trait Connection {
    type Row: Row;
    type Error: From<<Self::Row as Row>::Error>;

    async fn query(&mut self, sql: impl ToSql + 'async_trait) -> Result<Self::Row, Self::Error>;
}

pub trait FromRow<R: Row>: Sized {
    fn from_row(row: &R) -> Result<Self, R::Error>;
}

#[async_trait(?Send)]
pub trait Load<C: Connection>: ToSql + Sized {
    type Output: FromRow<C::Row>;

    async fn load(self, conn: &mut C) -> Result<Self::Output, C::Error> {
        conn.query(self)
            .await
            .and_then(|row| FromRow::from_row(&row).map_err(Into::into))
    }
}

impl<C, S> Load<C> for S
where
    C: Connection,
    S: Select,
    S::Queryable: FromRow<C::Row>,
{
    type Output = S::Queryable;
}
