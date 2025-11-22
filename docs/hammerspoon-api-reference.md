# Hammerspoon API Reference (for Inspiration)

Hammerspoon is a powerful automation tool for macOS that allows users to interact with system APIs through Lua scripts. It provides extensive control over applications, windows, mouse, keyboard, file system, audio devices, battery, screen, clipboard, location services, Wi-Fi, and more.

The following APIs serve as a reference for potential future enhancements to `@tego/bot`, especially for macOS-specific features:

## Core Modules

### hs.application

Application management and control:

- `hs.application.applicationsForBundleID()` - Get applications by bundle ID
- `hs.application.launchOrFocus()` - Launch or focus an application
- `hs.application.launchOrFocusByBundleID()` - Launch or focus by bundle ID
- `hs.application:name()` - Get application name
- `hs.application:bundleID()` - Get bundle identifier
- `hs.application:isRunning()` - Check if application is running
- `hs.application:kill()` - Kill the application
- `hs.application:kill9()` - Force kill the application
- `hs.application:activate()` - Activate the application
- `hs.application:hide()` - Hide the application
- `hs.application:unhide()` - Unhide the application
- `hs.application:isHidden()` - Check if application is hidden
- `hs.application:allWindows()` - Get all windows of the application
- `hs.application:mainWindow()` - Get main window
- `hs.application:focusedWindow()` - Get focused window
- `hs.application:findMenuItem()` - Find menu item
- `hs.application:selectMenuItem()` - Select menu item

### hs.window

Window management and manipulation:

- `hs.window.allWindows()` - Get all windows
- `hs.window.orderedWindows()` - Get ordered windows
- `hs.window.focusedWindow()` - Get focused window
- `hs.window.find()` - Find windows by title/application
- `hs.window:title()` - Get window title
- `hs.window:application()` - Get window's application
- `hs.window:frame()` - Get window frame (position and size)
- `hs.window:setFrame()` - Set window frame
- `hs.window:size()` - Get window size
- `hs.window:setSize()` - Set window size
- `hs.window:topLeft()` - Get top-left position
- `hs.window:setTopLeft()` - Set top-left position
- `hs.window:center()` - Center window on screen
- `hs.window:move()` - Move window
- `hs.window:resize()` - Resize window
- `hs.window:minimize()` - Minimize window
- `hs.window:unminimize()` - Unminimize window
- `hs.window:isMinimized()` - Check if minimized
- `hs.window:maximize()` - Maximize window
- `hs.window:close()` - Close window
- `hs.window:focus()` - Focus window
- `hs.window:raise()` - Raise window to front
- `hs.window:setFullScreen()` - Set fullscreen mode
- `hs.window:isFullScreen()` - Check if fullscreen
- `hs.window:setShadows()` - Enable/disable shadows
- `hs.window:zoomButtonRect()` - Get zoom button rectangle
- `hs.window:otherWindowsSameScreen()` - Get other windows on same screen
- `hs.window:otherWindowsAllScreens()` - Get other windows on all screens

### hs.mouse

Mouse control and monitoring:

- `hs.mouse.getAbsolutePosition()` - Get absolute mouse position
- `hs.mouse.setAbsolutePosition()` - Set absolute mouse position
- `hs.mouse.getRelativePosition()` - Get relative position within screen
- `hs.mouse.setRelativePosition()` - Set relative position within screen
- `hs.mouse.getButtons()` - Get currently pressed mouse buttons
- `hs.mouse.getCurrentScreen()` - Get screen containing mouse
- `hs.mouse.trackingSpeed()` - Get/set mouse tracking speed
- `hs.mouse.scrollingDirection()` - Get/set scrolling direction
- `hs.mouse.click()` - Simulate mouse click
- `hs.mouse.leftClick()` - Simulate left click
- `hs.mouse.rightClick()` - Simulate right click
- `hs.mouse.middleClick()` - Simulate middle click
- `hs.mouse.scroll()` - Simulate scroll wheel
- `hs.mouse.doubleClick()` - Simulate double click
- `hs.mouse.dragFromTo()` - Drag from one point to another

### hs.keycodes

Keyboard key code mapping:

- `hs.keycodes.map` - Key code mapping table
- `hs.keycodes.getName()` - Get key name from code
- `hs.keycodes.getCode()` - Get key code from name
- `hs.keycodes.inputMethodChanged` - Monitor input method changes

### hs.eventtap

Low-level keyboard and mouse event monitoring and injection:

- `hs.eventtap.new()` - Create new event tap
- `hs.eventtap:start()` - Start event tap
- `hs.eventtap:stop()` - Stop event tap
- `hs.eventtap.checkKeyboardModifiers()` - Check modifier keys
- `hs.eventtap.checkMouseButtons()` - Check mouse buttons
- `hs.eventtap.keyStroke()` - Send keystroke
- `hs.eventtap.keyStrokes()` - Send multiple keystrokes
- `hs.eventtap.keyRepeat()` - Enable/disable key repeat
- `hs.eventtap.eventTypes` - Event type constants

### hs.screen

Screen/monitor management:

- `hs.screen.allScreens()` - Get all screens
- `hs.screen.primaryScreen()` - Get primary screen
- `hs.screen:frame()` - Get screen frame
- `hs.screen:fullFrame()` - Get full frame (including menu bar)
- `hs.screen:availableFrame()` - Get available frame (excluding dock/menu bar)
- `hs.screen:name()` - Get screen name
- `hs.screen:id()` - Get screen ID
- `hs.screen:position()` - Get screen position
- `hs.screen:setPrimary()` - Set as primary screen
- `hs.screen:setMode()` - Set screen resolution/refresh rate
- `hs.screen:currentMode()` - Get current mode
- `hs.screen:availableModes()` - Get available modes
- `hs.screen:setBrightness()` - Set brightness
- `hs.screen:getBrightness()` - Get brightness
- `hs.screen:setGamma()` - Set gamma correction
- `hs.screen:getGamma()` - Get gamma correction
- `hs.screen:setInvertedPolarity()` - Invert colors
- `hs.screen:setTint()` - Set color tint

### hs.clipboard

Clipboard operations:

- `hs.clipboard.getContents()` - Get clipboard contents
- `hs.clipboard.setContents()` - Set clipboard contents
- `hs.clipboard.changeCount()` - Get change count
- `hs.clipboard.readImage()` - Read image from clipboard
- `hs.clipboard.writeImage()` - Write image to clipboard
- `hs.clipboard.readStyledText()` - Read styled text
- `hs.clipboard.writeStyledText()` - Write styled text
- `hs.clipboard.readSound()` - Read sound from clipboard
- `hs.clipboard.writeSound()` - Write sound to clipboard
- `hs.clipboard.readURL()` - Read URL from clipboard
- `hs.clipboard.writeURL()` - Write URL to clipboard

### hs.fs

File system operations:

- `hs.fs.attributes()` - Get file attributes
- `hs.fs.chdir()` - Change directory
- `hs.fs.currentDir()` - Get current directory
- `hs.fs.dir()` - List directory contents
- `hs.fs.link()` - Create symbolic/hard link
- `hs.fs.lock()` - Lock file
- `hs.fs.mkdir()` - Create directory
- `hs.fs.remove()` - Remove file/directory
- `hs.fs.rename()` - Rename file/directory
- `hs.fs.rmdir()` - Remove directory
- `hs.fs.touch()` - Touch file (update timestamp)
- `hs.fs.unlock()` - Unlock file
- `hs.fs.volumeInformation()` - Get volume information

### hs.pathwatcher

File system change monitoring:

- `hs.pathwatcher.new()` - Create new path watcher
- `hs.pathwatcher:start()` - Start watching
- `hs.pathwatcher:stop()` - Stop watching

### hs.timer

Timer and scheduling:

- `hs.timer.doAfter()` - Execute after delay
- `hs.timer.doEvery()` - Execute periodically
- `hs.timer.doAt()` - Execute at specific time
- `hs.timer:start()` - Start timer
- `hs.timer:stop()` - Stop timer
- `hs.timer:running()` - Check if running
- `hs.timer:nextTrigger()` - Get next trigger time

### hs.hotkey

Global hotkey bindings:

- `hs.hotkey.bind()` - Bind hotkey
- `hs.hotkey.bindAll()` - Bind all hotkeys
- `hs.hotkey.unbindAll()` - Unbind all hotkeys
- `hs.hotkey:disable()` - Disable hotkey
- `hs.hotkey:enable()` - Enable hotkey
- `hs.hotkey:delete()` - Delete hotkey

### hs.chooser

Interactive chooser UI:

- `hs.chooser.new()` - Create new chooser
- `hs.chooser:show()` - Show chooser
- `hs.chooser:hide()` - Hide chooser
- `hs.chooser:query()` - Set search query
- `hs.chooser:choices()` - Set choices
- `hs.chooser:selectedRow()` - Get selected row

### hs.dialog

Dialog boxes:

- `hs.dialog.blockAlert()` - Show blocking alert
- `hs.dialog.textPrompt()` - Show text input dialog
- `hs.dialog.webviewAlert()` - Show webview alert

### hs.webview

Web view component:

- `hs.webview.new()` - Create new webview
- `hs.webview:url()` - Get/set URL
- `hs.webview:html()` - Get/set HTML content
- `hs.webview:reload()` - Reload page
- `hs.webview:goBack()` - Go back
- `hs.webview:goForward()` - Go forward
- `hs.webview:stopLoading()` - Stop loading
- `hs.webview:show()` - Show webview
- `hs.webview:hide()` - Hide webview

### hs.menubar

Menu bar item:

- `hs.menubar.new()` - Create new menubar item
- `hs.menubar:setTitle()` - Set title
- `hs.menubar:setIcon()` - Set icon
- `hs.menubar:setMenu()` - Set menu
- `hs.menubar:setClickCallback()` - Set click callback
- `hs.menubar:delete()` - Delete menubar item

### hs.grid

Window grid management:

- `hs.grid.set()` - Set window to grid position
- `hs.grid.get()` - Get window grid position
- `hs.grid.snap()` - Snap window to grid
- `hs.grid.adjustWidth()` - Adjust grid width
- `hs.grid.adjustHeight()` - Adjust grid height
- `hs.grid.show()` - Show grid overlay
- `hs.grid.hide()` - Hide grid overlay

### hs.layout

Window layout management:

- `hs.layout.apply()` - Apply layout
- `hs.layout.get()` - Get current layout

### hs.battery

Battery information:

- `hs.battery.percentage()` - Get battery percentage
- `hs.battery.isCharging()` - Check if charging
- `hs.battery.isCharged()` - Check if fully charged
- `hs.battery.timeRemaining()` - Get time remaining
- `hs.battery.timeToFullCharge()` - Get time to full charge
- `hs.battery.psuSerial()` - Get power supply serial
- `hs.battery.health()` - Get battery health
- `hs.battery.healthCondition()` - Get health condition
- `hs.battery.cycles()` - Get charge cycles
- `hs.battery.voltage()` - Get voltage
- `hs.battery.amperage()` - Get amperage
- `hs.battery.temperature()` - Get temperature
- `hs.battery.maxCapacity()` - Get max capacity
- `hs.battery.designCapacity()` - Get design capacity

### hs.wifi

Wi-Fi information:

- `hs.wifi.currentNetwork()` - Get current network
- `hs.wifi.interface()` - Get Wi-Fi interface
- `hs.wifi.availableNetworks()` - Get available networks
- `hs.wifi.setPower()` - Enable/disable Wi-Fi
- `hs.wifi.power()` - Check if Wi-Fi is on

### hs.location

Location services:

- `hs.location.get()` - Get current location
- `hs.location.start()` - Start location services
- `hs.location.stop()` - Stop location services
- `hs.location.distance()` - Calculate distance

### hs.audiodevice

Audio device control:

- `hs.audiodevice.allDevices()` - Get all audio devices
- `hs.audiodevice.defaultInputDevice()` - Get default input
- `hs.audiodevice.defaultOutputDevice()` - Get default output
- `hs.audiodevice:setDefaultInputDevice()` - Set default input
- `hs.audiodevice:setDefaultOutputDevice()` - Set default output
- `hs.audiodevice:volume()` - Get/set volume
- `hs.audiodevice:muted()` - Get/set mute state
- `hs.audiodevice:name()` - Get device name
- `hs.audiodevice:uid()` - Get device UID

### hs.caffeinate

System sleep/wake control:

- `hs.caffeinate.start()` - Prevent system sleep
- `hs.caffeinate.stop()` - Allow system sleep
- `hs.caffeinate.set()` - Set caffeinate state
- `hs.caffeinate.get()` - Get caffeinate state
- `hs.caffeinate.displaySleep()` - Prevent display sleep
- `hs.caffeinate.systemSleep()` - Prevent system sleep
- `hs.caffeinate.system()` - Prevent system sleep

### hs.notify

Notification system:

- `hs.notify.new()` - Create new notification
- `hs.notify:send()` - Send notification
- `hs.notify:withdraw()` - Withdraw notification
- `hs.notify:title()` - Set title
- `hs.notify:subTitle()` - Set subtitle
- `hs.notify:informativeText()` - Set informative text
- `hs.notify:actionButtonTitle()` - Set action button
- `hs.notify:otherButtonTitle()` - Set other button
- `hs.notify:hasActionButton()` - Check if has action button
- `hs.notify:hasReplyButton()` - Check if has reply button
- `hs.notify:setIdImage()` - Set ID image
- `hs.notify:setContentImage()` - Set content image
- `hs.notify:setSoundName()` - Set sound
- `hs.notify:setAlwaysPresent()` - Set always present
- `hs.notify:setAutoWithdraw()` - Set auto withdraw
- `hs.notify:setWithdrawAfter()` - Set withdraw delay

### hs.http

HTTP requests:

- `hs.http.asyncGet()` - Async GET request
- `hs.http.asyncPost()` - Async POST request
- `hs.http.get()` - Synchronous GET request
- `hs.http.post()` - Synchronous POST request
- `hs.http.doRequest()` - Generic HTTP request
- `hs.http.urlParts()` - Parse URL
- `hs.http.encodeForQuery()` - Encode for query string
- `hs.http.convertHtmlToText()` - Convert HTML to text

### hs.json

JSON operations:

- `hs.json.encode()` - Encode to JSON
- `hs.json.decode()` - Decode from JSON
- `hs.json.write()` - Write JSON to file
- `hs.json.read()` - Read JSON from file

### hs.socket

Network socket operations:

- `hs.socket.new()` - Create new socket
- `hs.socket:connect()` - Connect to host
- `hs.socket:disconnect()` - Disconnect
- `hs.socket:write()` - Write data
- `hs.socket:read()` - Read data
- `hs.socket:listen()` - Listen for connections
- `hs.socket:accept()` - Accept connection

### hs.uielement

UI element inspection:

- `hs.uielement.new()` - Create UI element
- `hs.uielement:isValid()` - Check if valid
- `hs.uielement:isApplication()` - Check if application
- `hs.uielement:isWindow()` - Check if window
- `hs.uielement:title()` - Get title
- `hs.uielement:role()` - Get role
- `hs.uielement:selectedText()` - Get selected text
- `hs.uielement:attributeValue()` - Get attribute value
- `hs.uielement:setAttributeValue()` - Set attribute value
- `hs.uielement:performAction()` - Perform action
- `hs.uielement:elementAtPosition()` - Get element at position

### hs.image

Image operations:

- `hs.image.imageFromPath()` - Load image from path
- `hs.image.imageFromASCII()` - Create image from ASCII
- `hs.image:size()` - Get image size
- `hs.image:setSize()` - Set image size
- `hs.image:saveToFile()` - Save to file
- `hs.image:encodeAsPNG()` - Encode as PNG
- `hs.image:encodeAsJPEG()` - Encode as JPEG

### hs.screen.mainScreen

Main screen operations:

- `hs.screen.mainScreen()` - Get main screen
- `hs.screen:toEast()` - Get screen to the east
- `hs.screen:toWest()` - Get screen to the west
- `hs.screen:toNorth()` - Get screen to the north
- `hs.screen:toSouth()` - Get screen to the south
- `hs.screen:next()` - Get next screen
- `hs.screen:previous()` - Get previous screen

### hs.spaces

Mission Control spaces:

- `hs.spaces.allSpaces()` - Get all spaces
- `hs.spaces.mainScreenSpaces()` - Get main screen spaces
- `hs.spaces.currentSpace()` - Get current space
- `hs.spaces.moveWindowToSpace()` - Move window to space
- `hs.spaces.gotoSpace()` - Go to space

### hs.fnutils

Functional utilities:

- `hs.fnutils.contains()` - Check if contains
- `hs.fnutils.filter()` - Filter array
- `hs.fnutils.map()` - Map array
- `hs.fnutils.reduce()` - Reduce array
- `hs.fnutils.each()` - Iterate array
- `hs.fnutils.concat()` - Concatenate arrays
- `hs.fnutils.indexOf()` - Find index
- `hs.fnutils.find()` - Find element
- `hs.fnutils.sortByKeys()` - Sort by keys

## Spoons (Plugins)

Hammerspoon also supports a plugin system called "Spoons" that extends functionality:

- **Window management Spoons**: Window snapping, tiling, management
- **Application Spoons**: Application-specific automation
- **Utility Spoons**: Various utility functions
- **UI Spoons**: User interface enhancements

## Notes

Hammerspoon is macOS-specific and provides deep integration with macOS system APIs. While `@tego/bot` focuses on cross-platform automation, many of these concepts (especially window management, screen operations, and system integration) could serve as inspiration for platform-specific enhancements or future cross-platform abstractions.

For more information, refer to the [Hammerspoon official documentation](https://www.hammerspoon.org/docs/).

