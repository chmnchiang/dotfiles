use slog;

pub struct Context {
    pub logger: slog::Logger,
    pub is_dry_run: bool,
}
