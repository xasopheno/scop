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

#[derive(Debug, Clone)]
struct Defs<T> {
    defs: HashMap<String, Def<T>>,
    scopes: Vec<String>,
}

fn main() -> Result<(), ScopError> {
    println!("Scop!");
    let mut defs: Defs<NormalForm> = Defs::new();

    defs.insert("global", "1", 1)?;
    defs.insert("global", "2", 2)?;

    let new_scope = defs.create_uuid_scope();
    defs.insert(&new_scope, "3", 3)?;

    let result = defs.find("id");
    dbg!(defs);
    dbg!(result);

    Ok(())
}

impl<T> Defs<T>
where
    T: Clone + Sized,
{
    pub fn new() -> Self {
        let mut defs = HashMap::new();
        defs.insert("global".into(), HashMap::new());
        Self {
            defs: defs,
            scopes: vec!["global".to_string()],
        }
    }

    pub fn create_uuid_scope(&mut self) -> String {
        let new_scope = Uuid::new_v4().to_string();
        self.defs.insert(new_scope.to_string(), Def::new());
        self.scopes.push(new_scope.to_string());
        new_scope
    }

    pub fn create_named_scope(&mut self, new_scope: &str) -> () {
        self.defs.insert(new_scope.to_string(), Def::new());
        self.scopes.push(new_scope.to_string());
    }

    pub fn insert(&mut self, scope: &str, name: &str, value: T) -> Result<(), ScopError> {
        let current_scope = self
            .defs
            .entry(scope.to_string())
            .or_insert_with(HashMap::new);
        current_scope.insert(name.to_string(), value);
        Ok(())
    }

    pub fn find(&self, id: &str) -> Option<T> {
        for scope in self.scopes.iter().rev() {
            let current = self
                .defs
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
    fn it_can_insert_and_find_in_global_scope() -> Result<(), ScopError> {
        let mut defs: Defs<NormalForm> = Defs::new();

        defs.insert("global", "1", 1)?;
        defs.insert("global", "2", 2)?;

        let found = defs.find("1");
        assert_eq!(found, Some(1));
        let not_found = defs.find("3");
        assert_eq!(not_found, None);
        Ok(())
    }

    #[test]
    fn it_can_insert_and_find_with_uuid_scope_name() -> Result<(), ScopError> {
        let mut defs: Defs<NormalForm> = Defs::new();

        defs.insert("global", "1", 1)?;
        defs.insert("global", "2", 2)?;

        let new_scope = defs.create_uuid_scope();
        defs.insert(&new_scope, "3", 3)?;

        let found = defs.find("3");
        assert_eq!(found, Some(3));
        Ok(())
    }

    #[test]
    fn it_finds_value_in_innermost_scope() -> Result<(), ScopError> {
        let mut defs: Defs<NormalForm> = Defs::new();

        defs.insert("global", "1", 1)?;
        defs.insert("global", "2", 2)?;

        let new_scope = defs.create_uuid_scope();
        defs.insert(&new_scope, "1", 10)?;

        let found = defs.find("1");
        assert_eq!(found, Some(10));
        Ok(())
    }
}
