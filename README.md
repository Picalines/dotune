# dotune

A dotfile manager for configs applied on different operating systems simultaneously. Write Lua scripts to decide where your stuff is being linked to

## Example

Say I want to link [Vieb](https://vieb.dev/) configuration:

```sh
~/Repos/dotfiles/
├── .dotune.lua  # <-- linking script
└── web/         # <-- dotfiles source
    └── vieb/
        ├── .viebrc
        └── colors/...
```

```lua
-- Check if Vieb is installed (if you want)
if not (Windows.roaming_app_data / 'Vieb'):exists() then
    return
end

-- Use `Path` primitive instead of strings
local src = Path.here / 'web/vieb'

Dot.link(src / 'colors', Path.home / '.vieb/colors')
-- ^ symlink ~/Repos/dotfiles/web/vieb/colors -> ~/.vieb/colors

Dot.link((src / '.viebrc'):to(Path.home))
-- ^ symlink ~/Repos/dotfiles/web/vieb/.viebrc -> ~/.viebrc

-- Use `include` to run configs in child folders (like require, but relative)
include 'web/qutebrowser'  -- https://github.com/qutebrowser/qutebrowser
```

```sh
# Execute the script
dotune ~/Repos/dotfiles
```

## CLI

```sh
dotune --help     # Show usage and options
dotune            # Apply .dotune.lua in the current directory
dotune <dir>      # Use another repository folder
dotune --dry-run  # Don't apply write operations to the file system
```

## Lua API

### Standard library

Lua scripts have access to very basic parts of standard library: `print`, `string`, `table` and `utf8`. More complicated stuff like file I/O and environment access are disabled

### include

```lua
include 'shell'     -- Run shell/.dotune.lua relative to the current script
include 'web/vieb'  -- Nested paths are allowed; absolute paths, . and .. are not
```

### Dot

#### Dot.link

```lua
-- Create a symlink to a file or directory, creating missing parent directories.
-- Sources must be inside the repository; destinations must be outside it.
-- Replaces existing symlinks; errors if a regular file or directory exists.
Dot.link(Path.here / 'config', XDG.config / 'myapp')
```

#### Dot.copy

```lua
-- Copy a file or directory, creating missing parent directories.
-- Sources must be inside the repository; destinations must be outside it.
-- Directories are merged, matching files are overwritten, extra files are kept.
Dot.copy(Path.here / 'defaults', XDG.config / 'otherapp')
```

### Path

```lua
local config = Path.here / 'config.toml'   -- Join; same as Path.here:join('config.toml')

Dot.link(config:to(XDG.config))            -- Passes source and destination, keeping the filename
Dot.link(config, XDG.config / 'app.toml')  -- Choose a different destination name

local renamed = config % 'app.toml'        -- Replace filename; :with_tail('app.toml')
local backup = config * 'bak'              -- Replace extension; :with_extension('bak')
local parent = config.parent               -- Parent path, or nil at the filesystem root
local exists = config:exists()             -- Whether the path exists
local resolved = config:resolve()          -- Absolute path with symlinks resolved, or nil on failure
local text = tostring(config)              -- Convert a Path to a string

local binary = Path.which('vieb')          -- Find an executable on PATH, or return nil
local home = Path.home                     -- User's home directory
local here = Path.here                     -- Current script's directory; alias of Path.script_dir
local script = Path.script                 -- Current .dotune.lua file
local root = Path.root                     -- Filesystem root (e.g. / or C:\)
```

### XDG

```lua
-- Uses XDG environment variables, falling back to these paths under Path.home:
local config = XDG.config  -- XDG_CONFIG_HOME: ~/.config
local data   = XDG.data    -- XDG_DATA_HOME:   ~/.local/share
local state  = XDG.state   -- XDG_STATE_HOME:  ~/.local/state
local cache  = XDG.cache   -- XDG_CACHE_HOME:  ~/.cache
```

### OS

```lua
local config = OS.match {
    windows = Windows.roaming_app_data,
    macos   = Path.home / 'Library/Application Support',
    linux   = XDG.config,
    unix    = XDG.config,
    _       = Path.home / '.config',
}
-- Priority: exact OS, then unix, then _;
-- All keys are optional, but no match is an error
```

### Platform paths

(Some paths are missing, I didn't go through all this)

#### Windows

```lua
local roaming    = Windows.roaming_app_data    -- APPDATA, or ~/AppData/Roaming
local local_data = Windows.local_app_data      -- LOCALAPPDATA, or ~/AppData/Local
local low        = Windows.local_low_app_data  -- ~/AppData/LocalLow
local documents  = Windows.documents           -- ~/Documents
```

#### Mac

```lua
local applications = Mac.applications  -- /Applications
```

## Development

```sh
mise install
cargo build --release
```
