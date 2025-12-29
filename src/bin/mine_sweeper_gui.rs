// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use modern_minesweeper::controller::{
    AboutDialog, GameConfig, GameDifficulty, GameState, MINE_VALUE, MainWindow, StateDialog,
    change_flag, change_visibility, check_win, clear_grid, expand_selection, fill_grid, new_grid,
    vec2d_to_model_grid,
};
use slint::ComponentHandle;
use std::{cell::RefCell, env, rc::Rc};

fn main() -> Result<(), slint::PlatformError> {
    unsafe {
        env::set_var("RUST_BACKTRACE", "1");
    }
    let main_window = MainWindow::new()?;

    // Global Configs
    let level = Rc::new(RefCell::new(GameDifficulty::Medium));
    let game_config = Rc::new(RefCell::new(GameConfig::new(*level.borrow())));

    // Empty Grid
    let tiles = Rc::new(RefCell::new(new_grid(&*game_config.borrow())));
    let model = vec2d_to_model_grid(&*tiles.borrow());
    let font_size = 28.0;
    main_window.set_grid(model);
    main_window.set_state(GameState::Initial);
    main_window.set_mine_value(MINE_VALUE);
    main_window.set_flags(game_config.borrow().mine_count as i32);
    main_window.set_font_size(font_size);
    main_window.set_levels(GameDifficulty::create_model());
    main_window.invoke_initial_level((*level.borrow()).into());

    // First Move Occured
    let main_window_weak = main_window.as_weak();
    let game_config_cloned = game_config.clone();
    let tiles_cloned = tiles.clone();
    main_window.on_first_move_occured(move |position| {
        fill_grid(
            &*game_config_cloned.borrow(),
            position,
            &mut *tiles_cloned.borrow_mut(),
        );
        let model = vec2d_to_model_grid(&*tiles_cloned.borrow());
        main_window_weak.unwrap().set_grid(model);
        main_window_weak.unwrap().set_state(GameState::Normal);
    });

    // Quit Button
    let main_window_weak = main_window.as_weak();
    main_window.on_close(move || {
        main_window_weak.unwrap().hide().unwrap();
    });

    // Restart Button
    let main_window_weak = main_window.as_weak();
    let tiles_cloned = tiles.clone();
    let game_config_cloned = game_config.clone();
    main_window.on_restart(move || {
        clear_grid(&mut *tiles_cloned.borrow_mut());
        let model = vec2d_to_model_grid(&*tiles_cloned.borrow());
        main_window_weak.unwrap().set_grid(model);
        main_window_weak.unwrap().set_state(GameState::Initial);
        main_window_weak
            .unwrap()
            .set_flags(game_config_cloned.borrow().mine_count as i32);
        main_window_weak.unwrap().invoke_reset_timer();
    });

    // Expand Selection
    let main_window_weak = main_window.as_weak();
    let game_config_cloned = game_config.clone();
    let tiles_cloned = tiles.clone();
    main_window.on_expand_selection(move |position| {
        if let Some(_) = expand_selection(
            &*game_config_cloned.borrow(),
            &position,
            &mut *tiles_cloned.borrow_mut(),
        ) {
            main_window_weak.unwrap().set_state(GameState::Lose);
            // State Dialog
            if let Ok(state_dialog) = StateDialog::new() {
                state_dialog.set_state(GameState::Lose);
                state_dialog.set_font_size(font_size);
                state_dialog.show().unwrap();
                let state_dialog_weak = state_dialog.as_weak();
                state_dialog.on_close(move || {
                    state_dialog_weak.unwrap().hide().unwrap();
                });
            }
        }
        let model = vec2d_to_model_grid(&*tiles_cloned.borrow());
        main_window_weak.unwrap().set_grid(model);
    });

    // Change Flag
    let tiles_cloned = tiles.clone();
    main_window.on_change_flag(move |position, flag| {
        change_flag(&mut *tiles_cloned.borrow_mut(), &position, flag);
    });

    // Change Visibility
    let tiles_cloned = tiles.clone();
    let main_window_weak = main_window.as_weak();
    main_window.on_change_visibility(move |position, visible| {
        change_visibility(&mut *tiles_cloned.borrow_mut(), &position, visible);
        let tiles_ref = &*tiles_cloned.borrow();
        let tile = &tiles_ref[position.row as usize][position.col as usize];
        if tile.value == -1 {
            main_window_weak.unwrap().set_state(GameState::Lose);
            // State Dialog
            if let Ok(state_dialog) = StateDialog::new() {
                state_dialog.set_state(GameState::Lose);
                state_dialog.set_font_size(font_size);
                state_dialog.show().unwrap();
                let state_dialog_weak = state_dialog.as_weak();
                state_dialog.on_close(move || {
                    state_dialog_weak.unwrap().hide().unwrap();
                });
            }
        }
    });

    // Win Condition
    let game_config_cloned = game_config.clone();
    let tiles_cloned = tiles.clone();
    let main_window_weak = main_window.as_weak();
    main_window.on_check_win(move || {
        if check_win(&*game_config_cloned.borrow(), &*tiles_cloned.borrow()) {
            main_window_weak.unwrap().set_state(GameState::Win);
            // State Dialog
            if let Ok(state_dialog) = StateDialog::new() {
                state_dialog.set_state(GameState::Win);
                state_dialog.set_font_size(font_size);
                state_dialog.show().unwrap();
                let state_dialog_weak = state_dialog.as_weak();
                state_dialog.on_close(move || {
                    state_dialog_weak.unwrap().hide().unwrap();
                });
            }
        }
    });

    // Difficulty Changed
    let game_config_cloned = game_config.clone();
    let tiles_cloned = tiles.clone();
    let main_window_weak = main_window.as_weak();
    main_window.on_level_changed(move |index| {
        level.borrow_mut().clone_from(&index.into());
        game_config_cloned
            .borrow_mut()
            .clone_from(&GameConfig::new(*level.borrow()));
        tiles_cloned
            .borrow_mut()
            .clone_from(&new_grid(&*game_config_cloned.borrow()));
        let model = vec2d_to_model_grid(&*tiles_cloned.borrow());
        main_window_weak.unwrap().set_grid(model);
        main_window_weak
            .unwrap()
            .set_flags(game_config_cloned.borrow().mine_count as i32);
    });

    // About
    main_window.on_about(move || {
        if let Ok(about) = AboutDialog::new() {
            about.set_font_size(16.0);
            about.set_version(env!("CARGO_PKG_VERSION").into());
            about.set_home_page(env!("CARGO_PKG_REPOSITORY").into());
            about.set_license(env!("CARGO_PKG_LICENSE").into());
            about.show().unwrap();
            let about_weak = about.as_weak();
            about.on_close(move || {
                about_weak.unwrap().hide().unwrap();
            });
        }
    });

    main_window.run()
}
