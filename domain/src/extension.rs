use diesel::{sql_function, sql_types::Bool};

sql_function! {
    #[aggregate]
    fn every(b: Bool) -> Bool
}
