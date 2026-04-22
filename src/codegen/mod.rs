pub mod lsp_servers;
pub mod plugins;

mod autocmds;
mod init;
mod keymaps;
mod options;

use std::{
    fs,
    io::{self, Write},
    path::Path,
};

use anyhow::{Context, Result};

use crate::wizard::state::{Extra, Feature, WizardState};

pub fn generate(state: &WizardState, output_dir: &Path, dry_run: bool) -> Result<()> {
    if dry_run {
        generate_dry_run(state)
    } else {
        generate_to_disk(state, output_dir)
    }
}

fn generate_to_disk(state: &WizardState, output_dir: &Path) -> Result<()> {
    let lua_config = output_dir.join("lua").join("config");
    let lua_plugins = output_dir.join("lua").join("plugins");

    fs::create_dir_all(&lua_config)
        .with_context(|| format!("Failed to create {}", lua_config.display()))?;
    fs::create_dir_all(&lua_plugins)
        .with_context(|| format!("Failed to create {}", lua_plugins.display()))?;

    write_file(&output_dir.join("init.lua"), &init::generate(state))?;

    write_file(&lua_config.join("options.lua"), options::generate())?;
    write_file(&lua_config.join("keymaps.lua"), &keymaps::generate(state))?;
    write_file(&lua_config.join("autocmds.lua"), autocmds::generate())?;

    write_file(
        &lua_plugins.join("colorscheme.lua"),
        &plugins::colorscheme::generate(state),
    )?;

    if state.has_feature(Feature::Treesitter) {
        write_file(
            &lua_plugins.join("treesitter.lua"),
            &plugins::treesitter::generate(state),
        )?;
    }

    if state.has_feature(Feature::Lsp) {
        write_file(&lua_plugins.join("lsp.lua"), &plugins::lsp::generate(state))?;
    }

    if state.has_feature(Feature::Completion) {
        write_file(
            &lua_plugins.join("completion.lua"),
            &plugins::completion::generate(state),
        )?;
    }

    if let Some(content) = plugins::explorer::generate(state) {
        write_file(&lua_plugins.join("explorer.lua"), &content)?;
    }

    if state.has_extra(Extra::Telescope) {
        write_file(
            &lua_plugins.join("telescope.lua"),
            plugins::telescope::generate(),
        )?;
    }

    let git_content = plugins::git::generate(state);
    if !git_content.is_empty() {
        write_file(&lua_plugins.join("git.lua"), &git_content)?;
    }

    if state.has_extra(Extra::Lualine) {
        write_file(
            &lua_plugins.join("statusline.lua"),
            plugins::statusline::generate(),
        )?;
    }

    let extras_content = plugins::extras::generate(state);
    if !extras_content.is_empty() {
        write_file(&lua_plugins.join("extras.lua"), &extras_content)?;
    }

    Ok(())
}

fn write_file(path: &Path, content: &str) -> Result<()> {
    fs::write(path, content).with_context(|| format!("Failed to write {}", path.display()))
}

fn generate_dry_run(state: &WizardState) -> Result<()> {
    let stdout = io::stdout();
    let mut out = stdout.lock();

    let files: Vec<(&str, String)> = collect_files(state);

    for (name, content) in &files {
        writeln!(out, "\n{}", "═".repeat(80))?;
        writeln!(out, "FILE: {name}")?;
        writeln!(out, "{}", "═".repeat(80))?;
        writeln!(out, "{content}")?;
    }

    writeln!(out, "\n[dry-run] Would write {} files.", files.len())?;
    Ok(())
}

fn collect_files(state: &WizardState) -> Vec<(&'static str, String)> {
    let mut files: Vec<(&str, String)> = vec![
        ("init.lua", init::generate(state)),
        ("lua/config/options.lua", options::generate().to_string()),
        ("lua/config/keymaps.lua", keymaps::generate(state)),
        ("lua/config/autocmds.lua", autocmds::generate().to_string()),
        (
            "lua/plugins/colorscheme.lua",
            plugins::colorscheme::generate(state),
        ),
    ];

    if state.has_feature(Feature::Treesitter) {
        files.push((
            "lua/plugins/treesitter.lua",
            plugins::treesitter::generate(state),
        ));
    }
    if state.has_feature(Feature::Lsp) {
        files.push(("lua/plugins/lsp.lua", plugins::lsp::generate(state)));
    }
    if state.has_feature(Feature::Completion) {
        files.push((
            "lua/plugins/completion.lua",
            plugins::completion::generate(state),
        ));
    }
    if let Some(content) = plugins::explorer::generate(state) {
        files.push(("lua/plugins/explorer.lua", content));
    }
    if state.has_extra(Extra::Telescope) {
        files.push((
            "lua/plugins/telescope.lua",
            plugins::telescope::generate().to_string(),
        ));
    }

    let git = plugins::git::generate(state);
    if !git.is_empty() {
        files.push(("lua/plugins/git.lua", git));
    }

    if state.has_extra(Extra::Lualine) {
        files.push((
            "lua/plugins/statusline.lua",
            plugins::statusline::generate().to_string(),
        ));
    }

    let extras = plugins::extras::generate(state);
    if !extras.is_empty() {
        files.push(("lua/plugins/extras.lua", extras));
    }

    files
}
