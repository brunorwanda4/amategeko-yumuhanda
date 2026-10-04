param(
    [Parameter(Position = 0)]
    [ValidateSet("dev", "check")]
    [string]$Task = "dev"
)

switch ($Task) {
    "dev" {
        watchexec -r -e rs,toml,json -w crates -w assets -- cargo run -p desktop
    }
    "check" {
        watchexec -e rs,toml -w crates -- cargo check --workspace
    }
}
