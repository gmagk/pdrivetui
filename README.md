### Config
use cli option `config-file`  
if path does not exist then the default config will load

#### keybindings
for the keybindings missing from config file the defaults will be used  
modifiers supported `Ctrl`,`Alt`, the order does not matter  
add in config-file:  
```toml
[keybindings]
quit = "Ctrl+q"
add = "a"
del = "Ctrl+d"
download = "d"
info = "i"
rename = "r"
copy = "c"
```  