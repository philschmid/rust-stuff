# Window manager controller to close and work with application windows automatically

wmctrl needs to be installed:


```bash
### Ubuntu
sudo apt install wmctrl

### Arch Linux
sudo pacman -S wmctrl

### Fedora 
dnf install -y wmctrl

# You get the idea
```


os.system(f'''/usr/bin/osascript -e 'tell app "Visual Studio Code" to close (every window whose name is "allen_grammar.txt")' ''')
