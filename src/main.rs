use dark_pig_git::actions::Quit;
use dark_pig_git::garph::Garph;
use dark_pig_git::workspace::Workspace;
use dotenv::dotenv;
use gpui::{App, AppContext, Application, KeyBinding, WindowOptions};
use std::env;
use std::error::Error; // 👈 มาจาก lib.rs

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let path_repo = env::var("GIT_REPO_PATH")?;
    let repo = git2::Repository::open(&path_repo)?;
    let garph = Garph::new(repo);

    Application::new().run(|cx: &mut App| {
        cx.bind_keys([KeyBinding::new("ctrl-q", Quit, None)]);
        cx.on_action(|_action: &Quit, cx: &mut gpui::App| {
            println!("Quit action received");
            cx.quit();
        });
        cx.open_window(
            WindowOptions {
                // window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                let garph = cx.new(|_| garph);

                cx.new(|cx| Workspace::new(Some(garph), cx))
            },
        )
        .unwrap();
    });
    Ok(())
}
