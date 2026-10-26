# forticlient-tray-helper

![UI](https://i.imgur.com/QOXYQqb.png)
![Tray](https://i.imgur.com/8D6K4Ff.png)

> [!NOTE]
> This application has only been tested on Fedora 44

forticlient-tray-helper is a free and open-source system tray application for Linux that helps you manage the FortiClient service, GUI and tray launcher.

## Why use forticlient-tray-helper?

FortiClient installs a systemd service and a tray launcher that are permanently active in the background, even when you are not connected to the VPN. forticlient-tray-helper puts you back in control:

* **Privacy:** with the added options of only enabling FortiClient services or applications when needed, this application can help preserve your privacy. When the option is enabled, there is no background FortiClient daemon or tray process that can keep connections alive, phone home or collect telemetry when this application is closed. Nothing FortiClient-related runs until you say so.
* **On-demand usage:** start the official FortiClient service and GUI only when you actually need them (for example, to connect to a VPN), and shut everything down again when you are done.
* **Boot control:** disable the `forticlient.service` systemd unit and the tray launcher autostart entry so nothing FortiClient-related starts at boot or login. Re-enable them whenever you want FortiClient to come back automatically.
* **Resource usage:** fewer background processes means lower CPU and memory usage when you do not need FortiClient.

## Features

* Start and stop the FortiClient service and GUI
  * Starting waits until the FortiClient daemon is fully loaded before launching the GUI
  * The FortiClient GUI is launched in a separate process, so it does not block the application from responding to other actions and will close automatically when FortiClient is connected.
* Enable or disable the `forticlient.service` systemd unit (controls whether FortiClient starts at boot)
* Enable or disable the FortiClient tray launcher autostart entry
* Optionally start FortiClient automatically when this application starts (disabled by default)
* Live in the system tray for quick access
  * The tray menu lets you start/stop FortiClient, show the window, or quit (quitting also stops FortiClient unless disabled in the settings)
  * This app does not start or run automatically, meaning you are in full control of when FortiClient is active.
* Perform privileged operations through `pkexec`/polkit: you get one authentication prompt per operation, and the application never runs as root itself

## How it works

* The `forticlient.service` systemd unit is managed through `systemctl`, invoked via `pkexec`, so you authenticate with a polkit prompt instead of running the application as root.
* The tray launcher is disabled by renaming its binaries (e.g. `/opt/forticlient/fortitray` to `/opt/forticlient/fortitray.disabled`) and removing its autostart entry. FortiClient's own daemons normally respawn the tray launcher; because they reference the binaries by their original paths, renaming the binaries prevents the tray from being relaunched. Enabling the launcher restores the original file names and the autostart entry.
* Process detection is done through `/proc` by matching resolved executable paths, instead of fragile command-line name matching.

## Requirements

* Linux with systemd
* polkit (`pkexec`) for privileged operations
* [FortiClient VPN](https://www.fortinet.com/support/product-downloads#vpn) installed at `/opt/forticlient` with the `forticlient.service` systemd unit

## Building

### Development

You can run the frontend development server by running the following command:

```shell
yarn dev
```

You can run the full Tauri application in development mode by running the following command:

```shell
yarn tdev
```

### Frontend

You can build the frontend using `Vite` by running the following command:

```shell
yarn build
```

### Application

You can build the full Tauri application by running the following command:

```shell
yarn tauri build
```

For more information about building Tauri applications, please read the [Tauri documentation](https://v2.tauri.app/distribute/).

## Credits

### ReactJS

This project uses [React](https://reactjs.org/) to create the user interface.

### Theme

The theme used in this application is [Mantine](https://mantine.dev/).

### Images

The application icon (and derivatives) are provided by [Tabler](https://tabler.io/).

## About

This application is maintained by CodeDead. You can find more about us using the following links:
* [Website](https://codedead.com/)
* [Bluesky](https://bsky.app/profile/codedead.com)
* [Facebook](https://facebook.com/deadlinecodedead/)
* [Reddit](https://www.reddit.com/r/CodeDead/)

Copyright © 2026 CodeDead
