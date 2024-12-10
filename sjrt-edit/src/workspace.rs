use std::collections::HashMap;

use super::instance::Instance;

pub struct Workspace {
    instance_table: HashMap<uuid::Uuid, Instance>,
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut workspace = Workspace::new();
        let id = workspace.create_instance();
        workspace.destroy_instance(id);
    }
}
