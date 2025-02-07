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

    pub fn request_render(&mut self, id: uuid::Uuid, width: u32, height: u32) {
        let Some(instance) = self.instance_table.get_mut(&id) else {
            return;
        };

        instance.request_render(width, height);
    }

    pub async fn wait(&mut self, id: uuid::Uuid) {
        let Some(_instance) = self.instance_table.get_mut(&id) else {
            return;
        };

        // TODO
    }

    pub fn peek_rendered_image<W>(&self, id: uuid::Uuid, writer: &mut W)
    where
        W: std::io::Write + std::io::Seek,
    {
        let Some(instance) = self.instance_table.get(&id) else {
            return;
        };

        instance.peek_rendered_image(writer);
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
