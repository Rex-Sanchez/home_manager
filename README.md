# home_manager
A simple tool to manage you configs / themes / icons / fonts and many more

## Usage
Define a lua config file


```lua
-- config.lua

local scr = env.SCRIPT_DIR
local config = env.CONFIG_DIR


return {
  {
    name = "themes",
    src = scr ..  "/themes",
    dest = env.THEME_DIR,
    enable = true,
    force = true
  },
  {
    name = "icons",
    src = scr .. "/icons",
    dest = env.ICON_DIR,
    enable = true,
    force = true
  },
  {
    name = "fonts",
    src = scr .. "/fonts",
    dest = env.FONT_DIR,
    enable = true,
    force = true
  },
  {
    name = "zsh config",
    src = scr .. "/dotfiles/.zshrc",
    dest = scr .. "/.zshrc",
    enable = true,
    force = true
  },
  {
    name = "i3 config",
    src = scr .. "/dotfiles/i3",
    dest = config .. "/i3",
    enable = true,
    force = true
  },
  {
    name = "i3 status config",
    src = scr .. "/dotfiles/i3status",
    dest = config .. "/i3status",
    enable = true,
    force = true
  }
}

```

Run the config with home_manager to symlink the dirs/files
`home_manager -c config.lua`

If you just want to update your config and leave all otheres unchanged,
you can use the update flag, this will only link enabled non linked configs
this will ignore the `force` field in the config
`home_manager -u -c config.lua`

## env variables
There are a few global variables defined under the `env` namespace

``` lua
env.HOME_DIR    -- This is the current users home directory
env.SCRIPT_DIR  -- This is the directory where the lua config script is located
env.THEME_DIR   -- This is the users theme directory '~/.themes'
env.ICON_DIR    -- This is the users icons directory '~/.local/share/icons'
env.FONT_DIR    -- This is the users font directory '~/.local/share/fonts'
env.CONFIG_DIR  -- This is the users config directory '~/.config'
```

## Table description

```lua
{
 name = "i3 config",            -- A descriptive name, 
 src = "/some/absolute/url"     -- The source location of the file / dir that should be linked, this is a required field.  
 dest = "/some/absolute/url"    -- The destination location of the file / dir that should be linked, this is a required field.
 enable = true,                 -- If set to disable this config will not be linked. default is true.
 force = true                   -- If there aleady exists a link / file / dir at the destination overwrite it.
}

```







