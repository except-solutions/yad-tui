local dap = require('dap')
local mason_registry = require("mason-registry")
local codelldb_root = mason_registry.get_package("codelldb"):get_install_path() .. "/extension/"
local codelldb_path = codelldb_root .. "adapter/codelldb"
local liblldb_path = codelldb_root .. "lldb/lib/liblldb.so"

dap.adapters.lldb = {
    type = "executable",
    name = "lldb",
    command = "lldb-vscode"
}

dap.adapters.codelldb = {
  type = "server",
  port = "${port}",
  host = "127.0.0.1",
  executable = {
    command = codelldb_path,
    args = { "--liblldb", liblldb_path, "--port", "${port}" },
  },
}


dap.configurations.rust = {
    {
        name = "yad-tui2",
        type = "lldb",
        request = "launch",
        console = "integratedTerminal",
        terminal = "integratedTerminal",
        program = function()
            return vim.fn.getcwd() .. "/target/debug/yad-tui"
        end,
        cwd = "${workspaceFolder}",
        justMyCode = false,
        env = {
            RUST_BACKTRACE = 1

        },
        options = {
            env = {
                RUST_BACKTRACE = 1
                }
            }
    },
    {
        name = "hello-dap",
        type = "codelldb",
        request = "launch",
        program = function()
            return vim.fn.getcwd() .. "/target/debug/yad-tui"
        end,
        cwd = "${workspaceFolder}",
        stopOnEntry = false,
--        consoleMode = "evaluate",
--        console = "externalTerminal",
--        terminal = "integrated",
--        terminal = "external",
        sourceLanguages = { 'rust' }
    },
}
