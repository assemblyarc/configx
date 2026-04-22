use crate::wizard::state::{Explorer, Extra, KeybindingStyle, WizardState};

pub fn generate(state: &WizardState) -> String {
    let mut out = String::new();

    out.push_str(
        "-- =============================================================================\n",
    );
    out.push_str("-- keymaps.lua — key bindings\n");
    out.push_str(
        "-- =============================================================================\n\n",
    );
    out.push_str("local map = vim.keymap.set\n\n");

    out.push_str(
        "-- ── Leader ─────────────────────────────────────────────────────────────────\n",
    );
    out.push_str("-- Space is the most comfortable leader key for modern Neovim setups.\n");
    out.push_str("vim.g.mapleader      = \" \"\n");
    out.push_str("vim.g.maplocalleader = \" \"\n\n");

    out.push_str(
        "-- ── Universal bindings ─────────────────────────────────────────────────────\n",
    );
    out.push_str(
        "map(\"n\", \"<Esc>\", \"<cmd>nohlsearch<CR>\", { desc = \"Clear search highlights\" })\n",
    );
    out.push_str("map(\"n\", \"<leader>w\", \"<cmd>w<CR>\", { desc = \"Save file\" })\n");
    out.push_str("map(\"n\", \"<leader>q\", \"<cmd>q<CR>\", { desc = \"Quit\" })\n");
    out.push_str("map(\"n\", \"<leader>x\", \"<cmd>bd<CR>\", { desc = \"Close buffer\" })\n\n");

    out.push_str(
        "-- ── Window navigation ──────────────────────────────────────────────────────\n",
    );
    out.push_str("map(\"n\", \"<C-h>\", \"<C-w>h\", { desc = \"Move to left window\" })\n");
    out.push_str("map(\"n\", \"<C-j>\", \"<C-w>j\", { desc = \"Move to lower window\" })\n");
    out.push_str("map(\"n\", \"<C-k>\", \"<C-w>k\", { desc = \"Move to upper window\" })\n");
    out.push_str("map(\"n\", \"<C-l>\", \"<C-w>l\", { desc = \"Move to right window\" })\n");
    out.push_str("\n");

    out.push_str(
        "-- ── Buffer navigation ──────────────────────────────────────────────────────\n",
    );
    out.push_str("map(\"n\", \"<S-h>\", \"<cmd>bprevious<CR>\", { desc = \"Previous buffer\" })\n");
    out.push_str("map(\"n\", \"<S-l>\", \"<cmd>bnext<CR>\", { desc = \"Next buffer\" })\n\n");

    out.push_str(
        "-- ── Move lines ─────────────────────────────────────────────────────────────\n",
    );
    out.push_str("map(\"v\", \"J\", \":m '>+1<CR>gv=gv\", { desc = \"Move line down\" })\n");
    out.push_str("map(\"v\", \"K\", \":m '<-2<CR>gv=gv\", { desc = \"Move line up\" })\n\n");

    match state.keybinding_style {
        KeybindingStyle::Purist => {
            out.push_str(
                "-- ── Vim Purist bindings ─────────────────────────────────────────────────────\n",
            );
            out.push_str(
                "map(\"n\", \"<leader>ff\", \"<cmd>Telescope find_files<CR>\", { desc = \"Find files\" })\n",
            );
            out.push_str(
                "map(\"n\", \"<leader>fg\", \"<cmd>Telescope live_grep<CR>\", { desc = \"Live grep\" })\n",
            );
            out.push_str(
                "map(\"n\", \"<leader>fb\", \"<cmd>Telescope buffers<CR>\", { desc = \"Find buffers\" })\n",
            );
            out.push_str(
                "map(\"n\", \"<leader>fh\", \"<cmd>Telescope help_tags<CR>\", { desc = \"Help tags\" })\n\n",
            );
        }
        KeybindingStyle::VsCode => {
            out.push_str(
                "-- ── VSCode-Like bindings ────────────────────────────────────────────────────\n",
            );
            out.push_str(
                "map(\"n\", \"<C-p>\", \"<cmd>Telescope find_files<CR>\", { desc = \"Quick open\" })\n",
            );
            out.push_str(
                "map(\"n\", \"<C-f>\", \"<cmd>Telescope live_grep<CR>\", { desc = \"Search in files\" })\n",
            );
            out.push_str("map(\"n\", \"<C-s>\", \"<cmd>w<CR>\", { desc = \"Save file\" })\n");
            out.push_str(
                "map(\"i\", \"<C-s>\", \"<Esc><cmd>w<CR>\", { desc = \"Save file (insert)\" })\n",
            );
            out.push_str("map(\"n\", \"<C-z>\", \"u\", { desc = \"Undo\" })\n");
            out.push_str(
                "map(\"n\", \"<C-/>\", \"gcc\", { remap = true, desc = \"Toggle comment\" })\n\n",
            );
        }
    }

    match state.explorer {
        Explorer::NeoTree => {
            out.push_str(
                "-- ── File explorer (neo-tree) ────────────────────────────────────────────────\n",
            );
            out.push_str(
                "map(\"n\", \"<leader>e\", \"<cmd>Neotree toggle<CR>\", { desc = \"Toggle file tree\" })\n\n",
            );
        }
        Explorer::NvimTree => {
            out.push_str(
                "-- ── File explorer (nvim-tree) ───────────────────────────────────────────────\n",
            );
            out.push_str(
                "map(\"n\", \"<leader>e\", \"<cmd>NvimTreeToggle<CR>\", { desc = \"Toggle file tree\" })\n\n",
            );
        }
        Explorer::Oil => {
            out.push_str(
                "-- ── File explorer (oil.nvim) ────────────────────────────────────────────────\n",
            );
            out.push_str(
                "map(\"n\", \"<leader>e\", \"<cmd>Oil<CR>\", { desc = \"Open file browser\" })\n\n",
            );
        }
        Explorer::None => {}
    }

    if state.has_extra(Extra::Gitsigns) || state.has_extra(Extra::Neogit) {
        out.push_str(
            "-- ── Git ─────────────────────────────────────────────────────────────────────\n",
        );
        if state.has_extra(Extra::Neogit) {
            out.push_str(
                "map(\"n\", \"<leader>gg\", \"<cmd>Neogit<CR>\", { desc = \"Open Neogit\" })\n",
            );
        }
        if state.has_extra(Extra::Gitsigns) {
            out.push_str(
                "map(\"n\", \"<leader>gp\", \"<cmd>Gitsigns preview_hunk<CR>\", { desc = \"Preview hunk\" })\n",
            );
            out.push_str(
                "map(\"n\", \"]h\", \"<cmd>Gitsigns next_hunk<CR>\", { desc = \"Next git hunk\" })\n",
            );
            out.push_str(
                "map(\"n\", \"[h\", \"<cmd>Gitsigns prev_hunk<CR>\", { desc = \"Prev git hunk\" })\n",
            );
        }
        out.push('\n');
    }

    out
}
