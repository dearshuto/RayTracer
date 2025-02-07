use std::fs::File;

#[tokio::main]
async fn main() {
    let mut workspace = sjrt_edit::Workspace::new();
    let id = workspace.create_instance();

    workspace.request_render(id, 256, 256);
    workspace.wait(id).await;

    let mut file = File::create("edit_simple.png").unwrap();
    workspace.peek_rendered_image(id, &mut file);

    workspace.destroy_instance(id);
}
