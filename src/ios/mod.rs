pub mod system;
pub mod user;

pub fn resolve(iri: &iref::Iri<'_>) -> Result<std::path::PathBuf, crate::ResolveError> {
    match iri.scheme().as_str() {
        "file" => Ok(std::path::PathBuf::from(iri.path().into_str())),
        "container" => Ok(user::home_dir().join(iri.path().into_str())),
        unhandled => Err(crate::ResolveError::InvalidScheme(unhandled.to_string())),
    }
}
