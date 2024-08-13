use tui_v::*;


// ANCHOR: main
fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = init()?;
    let app_result = App::default().run(&mut terminal);
    if let Err(err) = restore() {
        eprintln!(
            "failed to restore terminal. Run `reset` or restart your terminal to recover: {}",
            err
        );
    }
    app_result
}
// ANCHOR_END: main