pub fn parse_load_many_result<T: Clone, U: Clone>(result: Vec<(T, Vec<U>)>) -> Vec<U> {
    if result.len() == 0 {
        vec![]
    } else {
        let (_, inner) = result[0].clone();

        inner
    }
}
