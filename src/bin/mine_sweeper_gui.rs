use minesweeper_slint::mine_sweeper_ui::MainWindow;
use slint::ComponentHandle;

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;
    main_window.run()
}
