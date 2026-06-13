#![windows_subsystem = "windows"]

use gpui::{ KeyBinding };
use gpui_component_assets::Assets;

mod app;
mod backend;
mod session;
mod sftp;
mod system;
mod terminal;

rust_i18n::i18n!("locales", fallback = "en");

gpui::actions!(ashell_terminal, [TerminalTabKey, TerminalBacktabKey]);

pub(crate) use app::{
    Ashell, ConnectionProgress, PaneLayout, SelectorEntry, SftpContextMenuState, TabGroup,
};

fn main() {
    app::startup::sync_macos_launch_environment();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    #[cfg(target_os = "macos")]
    let app = gpui_platform::application()
        .with_assets(Assets)
        .with_quit_mode(gpui::QuitMode::Explicit);

    #[cfg(not(target_os = "macos"))]
    let app = gpui_platform::application().with_assets(Assets);
    app.on_reopen(|cx| {
        if cx.windows().is_empty() {
            app::startup::open_main_window(cx);
        }
    });
    app.run(move |cx| {
        gpui_component::init(cx);
        cx.bind_keys([
            KeyBinding::new("tab", TerminalTabKey, Some(app::constants::TERMINAL_KEY_CONTEXT)),
            KeyBinding::new("shift-tab", TerminalBacktabKey, Some(app::constants::TERMINAL_KEY_CONTEXT)),
        ]);
        app::theme::load_embedded_themes(cx);
        if let Err(err) = app::theme::load_fonts(cx) {
            tracing::warn!("failed to load embedded fonts: {err:#}");
        }
        app::startup::open_main_window(cx);
    });
}
