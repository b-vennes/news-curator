pub fn traverse_vec<A, E>(values: Vec<Result<A, E>>) -> Result<Vec<A>, E> {
    let mut parsed = vec![];
    for value in values {
        match value {
            Ok(value) => parsed.push(value),
            Err(error) => return Err(error),
        }
    }
    return Ok(parsed);
}

pub fn traverse_option<A, E>(opt: Option<Result<A, E>>) -> Result<Option<A>, E> {
    match opt {
        Some(Ok(a)) => Ok(Some(a)),
        Some(Err(e)) => Err(e),
        None => Ok(None),
    }
}
