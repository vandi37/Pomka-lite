#[macro_export]
macro_rules! error {
    ($cond:expr => $err:expr) => {
        if $cond {
            Err($err)?
        }
    };
}

#[macro_export]
macro_rules! update {
    ($entity:ty where $col:ident : $id:expr) => {
        <$entity>::update_many().filter(<$entity as EntityTrait>::Column::$col.eq($id))
    };

    ($entity:ty : $id:expr) => {
        update!($entity where Id : $id)
    };

    ($entity:ty : $id:expr => { $($col:ident : $val:expr),* $(,)? }) => {{
        let mut query = <$entity>::update_many()
            .filter(<$entity as EntityTrait>::Column::Id.eq($id));
        $(
            query = query.col_expr(<$entity as EntityTrait>::Column::$col, Expr::value($val));
        )*
        query
    }};
    ($entity:ty where $id_col:ident : $id:expr => { $($col:ident : $val:expr),* $(,)? }) => {{
        let mut query = <$entity>::update_many()
            .filter(<$entity as EntityTrait>::Column::$id_col.eq($id));
        $(
            query = query.col_expr(<$entity as EntityTrait>::Column::$col, Expr::value($val));
        )*
        query
    }};
}

#[macro_export]
macro_rules! action {
    ($repo:expr ; $action_type:ident @ $id:expr => $description:expr) => {
        $repo
            .new_action($id, models::actions::Type::$action_type, $description)
            .await?;
    };
}
