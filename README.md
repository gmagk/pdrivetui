## Config
Run with `pdrivetui --conf-file <CONF_FILE>`.  
If path does not exist then will load the default config.  
For any missing keys in config file the corresponding defaults will be used.

#### Keybindings
Supported modifiers: `Ctrl`,`Alt`, the order does not matter in configured bindings.  
```toml                                           
[keybindings]
quit = "Ctrl+q"
keybindings = "?"
```

#### Conflict Strategy
```toml                                           
[strategy]
upload_file = "create-new-revision"
upload_folder = "merge"
download_file = "rename"
download_folder = "merge"
```

<br/>
<br/>

### Keybindings

| action      | key                   | usage                                                                | configurable |
|-------------|-----------------------|----------------------------------------------------------------------|--------------|
| quit        | `Ctrl+q`              | close app                                                            | *            |
| keybindings | `?`                   | list keybindings                                                     | *            |
| move down   | `j` / `Down`          | navigate contents list, context menu                                 |              |
| move up     | `k` / `Up`            | navigate contents list, context menu                                 |              |
| move right  | `l` / `Right`         | navigate (go 1 level down) contents list                             |              |
| move left   | `h` / `Left`          | navigate (go 1 level up) contents list                               |              |
| page down   | `Ctrl+d` / `PageDown` | navigate (go to bottom) contents list, context menu                  |              |
| page up     | `Ctrl+u` / `PageUp`   | navigate (go to top) contents list, context menu                     |              |
| select      | `Enter`               | open context menu, select context menu action, submit prompt, e.t.c. |              |
| close       | `Esc`                 | close context menu, prompt, e.t.c.                                   |              |

<br/>
<br/>

### Conflict Strategy

| case            | available actions                                            |
|-----------------|--------------------------------------------------------------|
| upload file     | `create-new-revision` (default), `rename`, `replace`, `skip` |
| upload folder   | `merge` (default), `rename`, `replace`, `skip`               |
| download file   | `rename` (default), `remove`, `skip`                         |
| download folder | `merge` (default), `rename`, `remove`, `skip`                |

<br/>
<br/>

### Context Menu
| action        | description                                                                                                                                                                             | available in                                                                                                          |
|---------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------|
| Delete        | Permanently deletes trashed items only.  First trash your files and then use this command to delete them permanently.                                                                   | `/trash`                                                                                                              |
| Download      | Downloads files and folders.  On conflict uses default strategy if no strategy is configured in pdrivetui config (see Conflict Strategy table).  Proton Docs or Sheets will be skipped. | `/my-files`, `/devices`, `/shared-with-me`                                                                            |
| EmptyTrash    | Permanently deletes all items in /trash.  Does not affect /photos-trash.  The operation is asynchronous.                                                                                | `/trash`                                                                                                              |
| Copy/Paste    | Copies files and folders                                                                                                                                                                | `/my-files`, `/devices`, `/shared-with-me`                                                                            |
| Create Folder | Creates subfolders                                                                                                                                                                      | `my-files`, `/devices`, `/shared-with-me`                                                                             |
| Move/Place    | Moves files and folders.  You can move files within the same user.                                                                                                                      | `/my-files`, `/devices`, `/shared-with-me`                                                                            |
| Rename        | Renames a node in place.  Does not move it to a different folder.                                                                                                                       | `/my-files`, `/devices`, `/shared-with-me`, `/trash`, `/albums`, `/photos`, `/photos-shared-with-me`, `/photos-trash` |
| Restore       | Restores trashed items.                                                                                                                                                                 | `/trash`                                                                                                              | 
| Trash         | Moves items to trash.  Does not permanently delete, use delete or empty-trash for that.                                                                                                 | `/my-files`, `/shared-with-me`, `/photos`                                                                             |
| Upload        | Uploads files and folders.  On conflict uses protons default strategy if no strategy is configured in pdrivetui config.  Files with the same content are automatically skipped.         | `/my-files`, `/shared-with-me`                                                                                        |

<br/>
<br/>

### Naming
When Renaming (folder/file) or Creating New Folder, spaces get replaced by underscore "_",    
any invalid characters will prevent the action from completing successfully.