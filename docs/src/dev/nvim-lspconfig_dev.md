# Neovim lspconfig dev environment


Ensure `nvim-lspconfig` configured and loaded in nvim

## Plug and its nix-compatible `WPlug`
```lua
local Plug = vim.fn['plug#']

vim.call('plug#begin')

-- other plugins
Plug('neovim/nvim-lspconfig') -- built-in LSP configurations
-- other plugins

vim.call('plug#end')

```

## Monkey patch onto `lspconfig`

The main reason why we're doing monkey patching is to ensure it's easy to put
to `lspconfig` when we're ready for production

```lua
local function ensure_ungrammar_lspconfig()
  local lspconfig = require('lspconfig')
  local lspconfig_util = require('lspconfig.util')
  local configs = require('lspconfig.configs')

  if not lspconfig.ungrammar_lsp then
    configs.ungrammar_lsp = {
      default_config = {
        cmd = { "nix", "run", "github:pegasust/zork#ungrammar_lsp" },
        filetypes = { "ungram" },
        root_dir = lspconfig.util.root_pattern(".git", ".ungram"),
        settings = {
          -- ungrammar lsp settings to be determined
        },
      },
    }
  end
end


ensure_ungrammar_lspconfig()
```

## Attach handler to `lspconfig`

Note that when we tell `mason` to `ensure_installed`, or install it manually,
Mason will take care of attaching the handler to `lspconfig`. We don't have
this integration yet, so we'll need to do it manually.

```lua
local function setup_ungrammar_handler()
  -- NOTE: requires `lspconfig.configs.ungrammar_lsp.default_config` to exists
  require('lspconfig').ungrammar_lsp.setup {
    on_attach = on_attach,
    capabilities = capabilities,
    settings = { 
      -- ungrammar lsp settings to be determined
    },
  }
end

setup_ungrammar_handler()
```

### Quick reminder on `on_attach` and `capabilities`

This should be accessible from your neovim config

```lua
-- A reminder on popular `on_attach` and `capabilities` configuration
local on_attach = function(client, bufnr)
  local nmap = function(keys, func, desc)
    if desc then
      desc = 'LSP: ' .. desc
    end

    vim.keymap.set('n', keys, func, { noremap = true, buffer = bufnr, desc = desc })
  end

  nmap('<leader>rn', vim.lsp.buf.rename, '[R]e[n]ame')
  nmap('<leader>ca', vim.lsp.buf.code_action, '[C]ode [A]ction')
  vim.api.nvim_buf_set_option(bufnr, 'omnifunc', 'v:lua.vim.lsp.omnifunc')
  nmap('<leader>df', function() vim.lsp.buf.format({ async = true }) end, '[D]ocument [F]ormat')
  -- ... your other LSP config mapping monolith here
end

local capabilities = require('cmp_nvim_lsp').default_capabilities()
```

---
Refs
- [nvim-lspconfig.git](https://github.com/neovim/nvim-lspconfig)
- [mason.nvim.git](https://github.com/williamboman/mason.nvim)
- [mason-lspconfig.nvim.git](https://github.com/williamboman/mason-lspconfig.nvim)
