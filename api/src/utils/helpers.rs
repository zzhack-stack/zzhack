use sea_orm::DbErr;

// SeaORM return Vec<(Model, Vec<Model>)>, but in most cases, we just need the second Model vector
pub fn parse_load_many_result<T: Clone, U: Clone>(result: Vec<(T, Vec<U>)>) -> Vec<U> {
    if result.len() == 0 {
        vec![]
    } else {
        let (_, inner) = result[0].clone();

        inner
    }
}

// SearORM will return DbErr::RecordNotInserted error when execute DoNothing on_conflict
// So we need to filter this error to avoid panic
pub fn filter_record_not_insert_error<T>(result: Result<T, DbErr>) {
    match result {
        Err(err) if err != DbErr::RecordNotInserted => panic!("{}", err),
        _ => (),
    }
}
