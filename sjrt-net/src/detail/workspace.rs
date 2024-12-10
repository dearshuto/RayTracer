use std::collections::HashMap;

use super::instance::Instance;

#[allow(unused)]
pub struct Workspace {
    instance_table: HashMap<uuid::Uuid, Instance>,
}

#[allow(unused)]
impl Workspace {
    pub fn new() -> Self {
        Self {
            instance_table: Default::default(),
        }
    }

    pub fn create_instance(&mut self) -> uuid::Uuid {
        let instance = Instance::new();
        let id = uuid::Uuid::new_v4();
        let _ = self.instance_table.insert(id, instance);
        id
    }

    pub fn destroy_instance(&mut self, id: uuid::Uuid) {
        self.instance_table.remove(&id);
    }

    pub async fn update(&mut self) {
        for instance in self.instance_table.values_mut() {
            instance.update().await;
        }
    }
}
