use wasm_bindgen::JsValue;
use web_sys::Storage;

const KEY: &str = "RTML.todomvc";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Item {
    pub description: String,
    pub completed: bool,
    pub editing: bool,
}

pub struct Store {
    pub items: Vec<Item>,
    ls: Storage,
}

fn get_local_storage() -> web_sys::Storage {
    web_sys::window().unwrap().local_storage().unwrap().unwrap()
}

impl Store {
    pub fn load() -> Self {
        let ls = get_local_storage();
        let items: Vec<Item> = ls
            .get(KEY)
            .and_then(|items| items.ok_or(JsValue::from_str("empty")))
            .map(|items| serde_json::from_str(&items).unwrap())
            .unwrap_or_default();
        Self { items, ls }
    }

    pub fn save(&self) -> Result<(), JsValue> {
        self.ls
            .set(KEY, &serde_json::to_string(&self.items).unwrap())
    }

    pub fn add(&mut self, description: String) {
        self.items.push(Item {
            description,
            completed: false,
            editing: false,
        });
        self.save().unwrap();
    }

    pub fn remove(&mut self, index: usize) {
        self.items.remove(index);
        self.save().unwrap();
    }

    pub fn edit(&mut self, index: usize, description: String) {
        self.items.get_mut(index).map(|item| {
            item.description = description;
            item.editing = false;
        });
        self.save().unwrap();
    }
}
