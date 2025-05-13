#[macro_export]
macro_rules! error {
    ($cond:expr -> $err:expr) => {
        if $cond {
            Err($err)?
        }
    };
}

#[macro_export]
macro_rules! update {
    ($entity:ty @ $id:expr) => {
        <$entity>::update_many().filter(<$entity>::primary_column().eq($id.into()))
    };

    ($entity:ty @ $id:expr => { $($col:ident: $val:expr),* $(,)? }) => {{
        let mut query = <$entity>::update_many()
            .filter(<$entity>::primary_column().eq($id.into()));
        $(
            query = query.set(<$entity>::Column::$col.set($val));
        )*
        query
    }};

    ($entity:ty @ $id:expr => { $($col:ident $op:tt $rhs:tt),* $(,)? }) => {{
        use sea_orm::sea_query::{Expr, Value, BinOper};
        let mut query = <$entity>::update_many()
            .filter(<$entity>::primary_column().eq($id.into()));

        $(
            let right_expr = parse_expr!($entity, $rhs);
            query = query.value(
                <$entity>::Column::$col,
                Expr::col(<$entity>::Column::$col).$op(right_expr),
            );
        )*
        query
    }};
}
