use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum ScopError {
    #[error("`{0}`")]
    Error(String),
}

type NormalForm = i32;
type Def<T> = HashMap<String, T>;
// type Scope = Vec<String>;
#[derive(Debug, Clone)]
struct Scopes(Vec<String>);

#[derive(Debug, Clone)]
struct Defs<T>(HashMap<String, Def<T>>);

fn main() -> Result<(), ScopError> {
    println!("Scop!");
    let mut defs: Defs<NormalForm> = Defs::new();
    let mut scopes = Scopes::new();

    defs.insert("global", "1", 1)?;
    defs.insert("global", "2", 2)?;

    let new_scope = defs.create_scope(&mut scopes);
    defs.insert(&new_scope, "3", 3)?;

    let result = defs.find("id", &scopes);
    dbg!(scopes);
    dbg!(defs);
    dbg!(result);

    Ok(())
}

impl Scopes {
    pub fn new() -> Self {
        Self(vec!["global".to_string()])
    }
    pub fn push(&mut self, v: String) {
        self.0.push(v);
    }
}

impl<T> Defs<T>
where
    T: Clone + Sized,
{
    pub fn new() -> Self {
        let mut defs = HashMap::new();
        defs.insert("global".into(), HashMap::new());
        Self(defs)
    }

    pub fn create_scope(&mut self, scopes: &mut Scopes) -> String {
        let new_scope = Uuid::new_v4().to_string();
        self.0.insert(new_scope.to_string(), Def::new());
        scopes.push(new_scope.to_string());
        new_scope
    }

    pub fn insert(&mut self, scope: &str, name: &str, value: T) -> Result<(), ScopError> {
        let current_scope = self.0.entry(scope.to_string()).or_insert_with(HashMap::new);
        current_scope.insert(name.to_string(), value);
        Ok(())
    }

    pub fn find(&self, id: &str, scopes: &Scopes) -> Option<T> {
        for scope in scopes.0.iter().rev() {
            let current = self
                .0
                .get(&scope.to_string())
                .expect("Named scope not found");
            let result = current.get(&id.to_string());
            if result.is_some() {
                return result.map(T::to_owned);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_insert_and_find_in_global_scope() {
        let mut defs: Defs<NormalForm> = Defs::new();
        let scopes = Scopes::new();

        defs.insert("global", "1", 1).unwrap();
        defs.insert("global", "2", 2).unwrap();

        let found = defs.find("1", &scopes);
        assert_eq!(found, Some(1));
        let not_found = defs.find("3".into(), &scopes);
        assert_eq!(not_found, None);
    }

    #[test]
    fn it_can_insert_and_find_with_generated_scope_name() {
        let mut defs: Defs<NormalForm> = Defs::new();
        let mut scopes = Scopes::new();

        defs.insert("global", "1", 1).unwrap();
        defs.insert("global", "2", 2).unwrap();

        let new_scope = defs.create_scope(&mut scopes);
        defs.insert(&new_scope, "3", 3).unwrap();

        let found = defs.find("3", &scopes);
        assert_eq!(found, Some(3));
    }

    #[test]
    fn it_finds_value_in_inner_scope() {
        let mut defs: Defs<NormalForm> = Defs::new();
        let mut scopes = Scopes::new();

        defs.insert("global", "1", 1).unwrap();
        defs.insert("global", "2", 2).unwrap();

        let new_scope = defs.create_scope(&mut scopes);
        defs.insert(&new_scope, "1", 10).unwrap();

        let found = defs.find("1", &scopes);
        assert_eq!(found, Some(10));
    }
}
