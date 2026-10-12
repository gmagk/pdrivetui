### Config
Run with `pdrivetui --conf-file <CONF_FILE>`.
If path does not exist then will load the default config.

#### Configurable keybindings
For any missing keybindings in config file the corresponding defaults will be used.    
Supported modifiers: `Ctrl`,`Alt`, the order does not matter in configured bindings.  
Config example:  
```toml                                           
[keybindings]
quit = "Ctrl+q"
keybindings = "?"
```  

### Keybindings

| action      | key               | usage                                                                | configurable |
|-------------|-------------------|----------------------------------------------------------------------|--------------|
| quit        | Ctrl+q            | close app                                                            | *            |
| keybindings | ?                 | list keybindings                                                     | *            |
| move down   | j / Down          | navigate contents list, context menu                                 |              |
| move up     | k / Up            | navigate contents list, context menu                                 |              |
| move right  | l / Right         | navigate (go 1 level down) contents list                             |              |
| move left   | h / Left          | navigate (go 1 level up) contents list                               |              |
| page down   | Ctrl+d / PageDown | navigate (go to bottom) contents list, context menu                  |              |
| page up     | Ctrl+u / PageUp   | navigate (go to top) contents list, context menu                     |              |
| select      | Enter             | open context menu, select context menu action, submit prompt, e.t.c. |              |
| close       | Esc               | close context menu, prompt, e.t.c.                                   |              |


### Naming
When Renaming (folder/file) or Creating New Folder, spaces get replaced by underscore "_",    
any invalid characters will prevent the action from completing successfully.

### Context Menu
| action        | description                                                                                                                                                                     | available                                       |
|---------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------|
| Delete        | Permanently deletes trashed items only.  First trash your files and then use this command to delete them permanently.                                                           | on `/my-files` subfolders and files             |
| Download      | Downloads files and folders.  On conflict uses protons default strategy if no strategy is configured in pdrivetui config.  Proton Docs or Sheets will be skipped.               | on files in `/my-files` and its subfolders      |
| EmptyTrash    | Permanently deletes all items in /trash.  Does not affect /photos-trash.  The operation is asynchronous.                                                                        | on `/trash`                                     |
| Copy/Paste    | Copies files and folders                                                                                                                                                        | on `/my-files` folder, its subfolders and files |
| Create Folder | Creates subfolders                                                                                                                                                              | on `/my-files` and its subfolders               |
| Move/Place    | Moves files and folders.  You can move files within the same user.                                                                                                              | on `/my-files` subfolders and files             |
| Rename        | Renames a node in place.  Does not move it to a different folder.                                                                                                               | on files in `/my-files` and its subfolders      |
| Restore       | Restores trashed items.                                                                                                                                                         | on folders and files in `/trash`                | 
| Trash         | Moves items to trash.  Does not permanently delete, use delete or empty-trash for that.                                                                                         | on `/my-files` subfolders and files             |
| Upload        | Uploads files and folders.  On conflict uses protons default strategy if no strategy is configured in pdrivetui config.  Files with the same content are automatically skipped. | on `/my-files` and its subfolders               |
