# AutoHotkey API Reference (for Inspiration)

AutoHotkey is a powerful automation scripting language for Windows that provides many additional features beyond basic mouse and keyboard control. The following APIs serve as a reference for potential future enhancements to `@tego/bot`:

## Window Management

AutoHotkey provides extensive window manipulation capabilities:

### Window Detection & Activation

- `WinExist()` - Check if a window exists
- `WinActivate()` - Activate a window
- `WinActivateBottom()` - Activate the bottommost matching window
- `WinWait()` - Wait for a window to exist
- `WinWaitActive()` - Wait for a window to become active
- `WinWaitClose()` - Wait for a window to close

### Window State Control

- `WinMinimize()` - Minimize a window
- `WinMaximize()` - Maximize a window
- `WinRestore()` - Restore a window
- `WinHide()` - Hide a window
- `WinShow()` - Show a window
- `WinClose()` - Close a window
- `WinKill()` - Forcefully close a window

### Window Position & Size

- `WinMove()` - Move and/or resize a window
- `WinGetPos()` - Retrieve position and size of a window
- `WinSet()` - Change various window attributes (always on top, transparency, etc.)

### Window Information

- `WinGetTitle()` - Retrieve window title
- `WinGetClass()` - Retrieve window class name
- `WinGetText()` - Retrieve window text
- `WinGetID()` - Retrieve unique window ID
- `WinGetPID()` - Retrieve process ID of window's process
- `WinGetCount()` - Count matching windows

## Process Management

- `Run()` - Run a program, script, or document
- `RunWait()` - Run a program and wait until it finishes
- `Process` - Perform various operations on a process (close, wait, priority, etc.)
- `ProcessExist()` - Check if a process exists
- `ProcessWait()` - Wait for a process to exist
- `ProcessWaitClose()` - Wait for a process to close

## Clipboard Operations

- `Clipboard` - Access/modify clipboard contents
- `ClipboardAll` - Retrieve all clipboard data including formatting
- `ClipWait()` - Wait for clipboard to contain data

## File & Directory Operations

### File Operations

- `FileRead()` - Read file contents
- `FileAppend()` - Append text to a file
- `FileDelete()` - Delete files
- `FileMove()` - Move or rename files
- `FileCopy()` - Copy files
- `FileExist()` - Check if file/folder exists
- `FileGetSize()` - Retrieve file size
- `FileGetTime()` - Retrieve file time
- `FileSetTime()` - Set file time

### Directory Operations

- `DirCreate()` - Create directory/folder
- `DirDelete()` - Delete directory/folder
- `DirMove()` - Move/rename directory
- `DirCopy()` - Copy directory
- `DirSelect()` - Display folder selection dialog

## Image & Pixel Operations

- `ImageSearch()` - Search for an image on screen
- `PixelSearch()` - Search for a pixel of specified color
- `PixelGetColor()` - Retrieve pixel color at coordinates

## GUI Creation

- `Gui` - Create and manage custom windows
- `GuiCtrl` - Add controls (buttons, text, edit boxes, etc.)
- `GuiShow()` - Show/hide GUI window
- `GuiClose()` - Close GUI window

## System Information & Control

- `SysGet()` - Retrieve system information (screen size, work area, etc.)
- `Shutdown()` - Shutdown, restart, or log off the system
- `Sleep()` - Wait/sleep for specified milliseconds
- `SetTimer()` - Create/manage timed subroutines
- `FormatTime()` - Format time/date string
- `A_Now` - Current date and time variable

## String Operations

- `StringReplace()` - Replace substring in string
- `StringSplit()` - Split string into array
- `StringLen()` - Get string length
- `SubStr()` - Extract substring
- `StrReplace()` - Replace occurrences of substring
- `RegExMatch()` - Match string using regular expression
- `RegExReplace()` - Replace using regular expression

## Registry Operations

- `RegRead()` - Read from registry
- `RegWrite()` - Write to registry
- `RegDelete()` - Delete from registry

## Network Operations

- `UrlDownloadToFile()` - Download file from URL
- `ComObjCreate()` - Create COM objects (for advanced automation)

## User Interaction

- `MsgBox()` - Display message box
- `InputBox()` - Prompt user for input
- `FileSelectFile()` - Display file selection dialog
- `FileSelectFolder()` - Display folder selection dialog
- `ToolTip()` - Display tooltip
- `SplashTextOn/Off()` - Display splash text

## Advanced Mouse & Keyboard

- `MouseGetPos()` - Get mouse position (with window info)
- `MouseClickDrag()` - Click and drag mouse
- `SendRaw()` - Send raw text without interpreting special characters
- `SendInput()` - Send keystrokes using SendInput method
- `SendPlay()` - Send keystrokes using Play method
- `SendMode` - Set send mode (Input, Play, Event, etc.)
- `SetKeyDelay()` - Set delay between keystrokes
- `SetMouseDelay()` - Set delay between mouse movements

## Hotkeys & Hotstrings

- Hotkey definitions (e.g., `^c::` for Ctrl+C)
- Hotstring definitions (e.g., `::btw::by the way`)
- Context-sensitive hotkeys
- Custom key combinations

## Notes

While `@tego/bot` currently focuses on cross-platform mouse, keyboard, and screen automation, many of these AutoHotkey features could serve as inspiration for future enhancements. However, some features (like Windows-specific window management) may not be applicable to cross-platform libraries.

For more information, refer to the [AutoHotkey official documentation](https://www.autohotkey.com/docs/).

