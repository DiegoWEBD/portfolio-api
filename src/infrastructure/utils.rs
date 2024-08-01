pub fn convert_pg_error(error: tokio_postgres::Error) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, error)
}