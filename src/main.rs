use std::collections::HashMap;
type NormalForm = String;
type Def = HashMap<String, NormalForm>;
type Scopes = Vec<String>;

#[derive(Debug, Clone)]
struct Defs(HashMap<String, Def>);

fn main() {
    println!("Scop!");

    let mut defs = Defs::new();
    let mut global = Def::new();

    global.insert("1".into(), "one".into());
    global.insert("2".into(), "two".into());

    defs.0.insert("global".into(), global);
    let mut scopes = Scopes::new();
    scopes.push("global".into());

    dbg!(&defs);

    let id = "1".to_string();

    let result = defs.find(id, &scopes);
    dbg!(result);
}

impl Defs {
    pub fn new() -> Self {
        Defs(HashMap::new())
    }
    pub fn find(&self, id: String, scopes: &Scopes) -> Option<NormalForm> {
        for scope in scopes {
            let current = self.0.get(scope).expect("Named scope not found");
            let result = current.get(&id);
            if result.is_some() {
                return result.map(NormalForm::to_owned);
            }
        }

        None
    }
}
