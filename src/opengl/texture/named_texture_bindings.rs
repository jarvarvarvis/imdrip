use std::collections::HashMap;

pub type TextureUnit = u32;

pub struct NamedTextureBindings {
    bindings: HashMap<String, TextureUnit>,
}

impl NamedTextureBindings {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn from<const N: usize>(key_values: [(String, TextureUnit); N]) -> Self {
        let named_units = HashMap::from(key_values);
        Self { bindings: named_units }
    }

    pub fn add(&mut self, name: &str, unit: TextureUnit) {
        let name = String::from(name);
        self.bindings.insert(name, unit);
    }

    pub fn has_unit(&mut self, name: &str) -> bool {
        self.bindings.contains_key(name)
    }

    pub fn get(&mut self, name: &str) -> Option<TextureUnit> {
        self.bindings.get(name).copied()
    }

    pub fn activate(&mut self, name: &str) -> Result<(), String> {
        if let Some(unit) = self.get(name) {
            return super::set_active_texture_unit(unit);
        }

        Err(format!("No texture unit with name \"{}\" exists", name))
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, TextureUnit> {
        self.bindings.iter()
    }
}
