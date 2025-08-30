use sqlx::query::QueryAs;
use sqlx::Database;
use sqlx::Encode;
use sqlx::Type;

/// Trait contains some extensions for `sqlx`.
pub trait QueryExt<'a, DB: Database> {
    /// Method which allows you to bind several values in one go. This method will accept any
    /// container which can be turned into an Iterator of values.
    fn bind_all<Vs, V>(self, values: Vs) -> Self
    where
        V: Send + Encode<'a, DB> + Type<DB> + 'a,
        Vs: IntoIterator<Item = V>;
}

impl<'a, DB, O> QueryExt<'a, DB> for QueryAs<'a, DB, O, <DB as Database>::Arguments<'a>>
where
    DB: Database,
{
    fn bind_all<Vs, V>(mut self, values: Vs) -> Self
    where
        V: Send + Encode<'a, DB> + Type<DB> + 'a,
        Vs: IntoIterator<Item = V>,
    {
        for v in values {
            self = self.bind(v);
        }
        self
    }
}
