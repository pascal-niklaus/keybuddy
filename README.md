# Keybuddy

**KeyBuddy** is a command-line utility to execute arbitrary commands
based on configurable key sequences typed using an external keyboard.
I use a small external numeric keypad.

The key sequences and commands that are executed upon a match are
freely configurable.

## Installation

- Compile the rust program using `cargo`.

- Add an udev rule to make the input device modifiable as user. In my
  case, the udev rule is stored in `/etc/udev/rules.d/80-keypad.rules`
  and contains the following line:

  ```
    ACTION=="add", ATTRS{idVendor}=="1234", ATTRS{idProduct}=="5678", SUBSYSTEM=="input", OWNER="user_name"
  ```

- Create a config file. Default is `$HOME/.config/keybuddy.conf`. Here
  is an example showing the basic structure. The content of this file
  is just an example and does not make much sense.

  ```
    # Different comment styles are supported
    // this line also is a comment
    ; and this one as well

    # Maximum delay between keystrokes (is seconds) so that these still
    # belong to the same key sequence. Chose a value long enough, but not
    # too long because otherwise you have to wait considerably to abort
    # a key sequence and start a new one
    
    delay = 1.5

    # Commands are executed using a particular shell; I use 'fish', for most folks 
    # this will be either 'sh' or 'bash'
    
    shell = "fish"

    # Several filters allow to identify the keyboard keybuddy takes input
    # from. The filters must leave exactly one keyboard that matches.
    # Available filters are the USB vendor and/or product id...
    
    vid = 0x046a
    pid = 0x0014
    
    # ...or parts of the device's name that must be present or absent
    
    # device_exclude = "Control"
    # device_include = "USB"

    # A pseudo-command tells the applicaton when to terminate
    
    quit_command = "quit"

    # Definition of key sequences

    # Key sequences are a comma-separated list of raw key codes or 
    # mnemonic equivalents, followed by '=>' and the command to execute.
    # The command must be quoted.
    
    # This is the key sequence that causes keybuddy to terminate:
    
    KEY_ESC, KEY_ESC, KEY_ESC => "quit"
    
    # some examples that do not make any sense but show the idea:
    # KEY_KP1 is the "1" on the numeric keypad. 

    KEY_ESC, KEY_KP1  => "command1"
    KEY_ESC, KEY_KP2  => "command2"
    KEY_ESC, KEY_KP3, KEY_KP1  => "command4"
    KEY_ESC, KEY_KP3, KEY_KP2  => "command5"

    KEY_LEFTALT => "command6"
    KEY_NUMLOCK, KEY_NUMLOCK => "command7"
    KEY_BACKSPACE => "command8"

    # here is a more sensible example:
    # take a screenshot using imagemagick's 'import' when ENTER is hit
    # twice on the numeric keypad:

    KEY_KPENTER, KEY_KPENTER => "import -window root \"(date +%x-%Hh%Mm%Ss).png\""
  ```

- Start 'keybuddy' and use it !

## Details

- `keybuddy` sets the keyboard into "floating" state, i.e. it
  disconnects it from your applications. Hitting keys on this keyboard
  will therefore not have any effect anymore.  **Be careful not to set
  your main keyboard floating!**
  
  Here you see this effect in the output of `xlist`:
  
  Before running keybuddy, `xinput --list` shows the keypad as follows:
  
  ```
  $ xinput --list
  ⎡ Virtual core pointer                          id=2    [master pointer  (3)]
  ⎜   ↳ Virtual core XTEST pointer                id=4    [slave  pointer  (2)]
  |  ....
  ⎣ Virtual core keyboard                         id=3    [master keyboard (2)]
      ↳ Virtual core XTEST keyboard               id=5    [slave  keyboard (3)]
      ↳ ...
      ↳ HID 1234:5678                             id=19   [slave  keyboard (3)]    
  ```
  
  After starting keybuddy, the keypad becomes "floating":
  
  ```
  $ xinput --list
  ⎡ Virtual core pointer                          id=2    [master pointer  (3)]
  ⎜   ↳ Virtual core XTEST pointer                id=4    [slave  pointer  (2)]
  |  ....
  ⎣ Virtual core keyboard                         id=3    [master keyboard (2)]
      ↳ Virtual core XTEST keyboard               id=5    [slave  keyboard (3)]
      ↳ ...
  ∼ HID 1234:5678                                 id=19   [floating slave]
  ```

  KeyBuddy does not reattach the floating keypad when it terminates,
  but in the example above you could do this manually like so:
  
  ```
  $ xinput reattach 19 3
  ```

- Key sequences can be written as raw key codes, or as mnemonic
  equivalents.  To have these listed, start keybuddy with the `-k`
  option, which will print the mnemonics of the keys you press to the
  terminal.
  
  Example:
  
  ```
  $ keybuddy -k

  Showing codes of key strokes received (Ctrl-C to abort)
  Listening on device /dev/input/event17 ...
  KEY_KP1 KEY_KP2 KEY_KP3 KEY_ESC KEY_BACKSPACE KEY_KPENTER KEY_KPDOT KEY_KP0
  ```

- The `-v` (or `--verbose`) option shows the key strokes as they are
  typed, the commands that are executed, and the tree with the key
  sequences that is constructed based on the config file's
  content. Here is an example:
  
   ```
   > keybuddy -v
   KeyBuddy -- (C) 2024 Pascal Niklaus
   Listening on device /dev/input/event17 ...
   key-command association tree:
   (0, None)
   |-- (ESC, None)
   |   |-- (ESC, None)
   |   |   `-- (ESC, Some("quit"))
   |   |-- (KP1, Some("command1"))
   |   |-- (KP2, Some("command2"))
   |   `-- (KP3, None)
   |       |-- (KP1, Some("command4"))
   |       `-- (KP2, Some("command5"))
   |-- (LEFTALT, Some("command6"))
   |-- (NUMLOCK, None)
   |   `-- (NUMLOCK, Some("command7"))
   |-- (BACKSPACE, Some("command8"))
   `-- (KPENTER, None)
       `-- (KPENTER, Some("import -window root \\\"(date +%x-%Hh%Mm%Ss).png\\\""))

   KEY_ESC KEY_KP1 -> executing <command1>
   KEY_ESC KEY_KP2 -> executing <command2>
   KEY_ESC KEY_KP3 KEY_KP1 -> executing <command4>
   KEY_ESC KEY_KP3 KEY_KP2 -> executing <command5>
   KEY_ESC KEY_ESC KEY_ESC -> exiting...
   ```

- Of course, there is also a help feature (`-h`, `--help`)

   ```
   $ keybuddy -h
   KeyBuddy -- keystroke interpreter for separate keypad
   (C) 2024 Pascal Niklaus

   Usage: keybuddy [OPTIONS]

   Options:
     -k, --show-keys            Show key strokes received
     -d, --delay <SECONDS>      Set maximum time span between keystrokes that form a sequence [default: 2]
         --cfg-file <CFG_FILE>  Use config file [default: /home/your_name/.config/keybuddy.conf]
     -v, --verbose              Be verbose (for debugging)
     -h, --help                 Print help
     -V, --version              Print version
  ```

    
