use std::path::Path;

use clap::{Command, CommandFactory};
use clap_complete::Shell;

fn generate_impl(shell: Shell, app: &mut Command, app_name: &str, out_dir: &Path, file: String) {
    let dest_file = out_dir.join(file);

    std::fs::create_dir_all(dest_file.parent().unwrap()).unwrap();

    if let Ok(mut dest) = std::fs::File::create(dest_file) {
        clap_complete::generate(shell, app, app_name, &mut dest);
    }
}

pub fn generate(out_dir: &Path) {
    use clap_complete::Shell::{Bash, Elvish, Fish, PowerShell, Zsh};

    let app_name = "txtfind";
    let mut app = crate::Args::command();

    app.set_bin_name(app_name);

    generate_impl(Bash, &mut app, app_name, out_dir, format!("bash/{app_name}"));
    generate_impl(Elvish, &mut app, app_name, out_dir, format!("elvish/{app_name}"));
    generate_impl(Fish, &mut app, app_name, out_dir, format!("fish/{app_name}"));
    generate_impl(
        PowerShell,
        &mut app,
        app_name,
        out_dir,
        format!("powershell/{app_name}"),
    );
    generate_impl(Zsh, &mut app, app_name, out_dir, format!("zsh/_{app_name}"));
}