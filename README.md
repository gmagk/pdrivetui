### Config
Use cli option `config-file`, if path does not exist then the default config will load
`pdrivetui config-file`

#### keybindings
For the keybindings missing from config file the defaults will be used  
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

### Naming
When Renaming or Creating New Folder, spaces get replaced by underscore "_",  
any invalid characters will prevent the action from completing successfully.

### Actions
- Copy: Copies files and folders. You can copy across My files, devices and shared folders.
- ..