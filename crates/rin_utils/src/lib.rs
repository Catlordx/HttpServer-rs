mod json;
mod query;

pub use json::ContextJsonExt;
pub use query::ContextQueryExt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
