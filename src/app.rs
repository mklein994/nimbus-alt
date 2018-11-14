use clap::App;

pub fn build_cli() -> App<'static, 'static> {
    app_from_crate!()
}
