macro_rules! created_at_utc {
    ($column:literal) => {
        concat!(
            "to_char(",
            $column,
            r#" at time zone 'utc', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as created_at"#
        )
    };
    ($column:literal, $alias:literal) => {
        concat!(
            "to_char(",
            $column,
            r#" at time zone 'utc', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') as "#,
            $alias
        )
    };
}

pub(crate) use created_at_utc;
