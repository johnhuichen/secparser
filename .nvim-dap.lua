require("mason").setup()
require("mason-lspconfig").setup()

local dap = require("dap")
local mason_registry = require("mason-registry")

local codelldb = mason_registry.get_package("codelldb")
local extension_path = codelldb:get_install_path() .. "/extension/"
local codelldb_path = extension_path .. "adapter/codelldb"

dap.adapters.codelldb = {
    type = "server",
    port = "${port}",
    executable = {command = codelldb_path, args = {"--port", "${port}"}}
}
dap.configurations.rust = {
    {
        name = "target/debug/local_rust_dap_example",
        type = "codelldb",
        request = "launch",
        program = function()
            vim.fn.jobstart('cargo build')
            return "target/debug/secingest"
        end,
        cwd = "${workspaceFolder}",
        stopOnEntry = false
    }
}
