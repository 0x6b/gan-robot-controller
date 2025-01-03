# gan-robot-controller

A simple CLI to control the [GAN Cube Robot](https://www.gancube.com/products/gan-speed-cube-robot).

> [!CAUTION]
> Running this code may break your cube or robot. Use at your own risk.

## Usage

```console
$ gan-robot-controller --help
Usage: gan-robot-controller [OPTIONS] <COMMAND>

Commands:
  scramble  Scramble the cube with the given number of moves
  move      Do moves on the cube with the given move sequence
  repl      Enter a REPL to interact with the cube
  help      Print this message or the help of the given subcommand(s)

Options:
  -n, --name <NAME>
          The name of the GAN robot [env: GAN_ROBOT_NAME=] [default: GAN-a7f13]
  -m, --move-characteristic <MOVE_CHARACTERISTIC>
          The move characteristic UUID of the GAN robot [env:
          GAN_ROBOT_MOVE_CHARACTERISTIC=] [default:
          0000fff3-0000-1000-8000-00805f9b34fb]
  -s, --status-characteristic <STATUS_CHARACTERISTIC>
          The status characteristic UUID of the GAN robot [env:
          GAN_ROBOT_STATUS_CHARACTERISTIC=] [default:
          0000fff2-0000-1000-8000-00805f9b34fb]
  -h, --help
          Print help
  -V, --version
          Print version
```

i.e.

```console
$ gan-robot-controller scramble 8
```

or

```console
$ gan-robot-controller move "R D2"
```

Supported moves include `R`, `R'`, `R2`, `R2'`, `F`, `F'`, `F2`, `F2'`, `D`, `D'`, `D2`, `D2'`, `L`, `L'`, `L2`, `L2'`, `B`, `B'`, `B2`, and `B2'`.

## Supported Platforms

Tested on macOS, but could work on other platforms. See [deviceplug/btleplug](https://github.com/deviceplug/btleplug) for more information.

## Acknowledgements

[cubing/cubing.js](https://github.com/cubing/cubing.js), especially the [GanRobot.ts](https://github.com/cubing/cubing.js/blob/19e893db4d6b2feaeafd4e40f3a5183b6bad88fc/src/cubing/bluetooth/smart-robot/GanRobot.ts), for the GAN robot control logic.

## License

MIT. See [LICENSE](LICENSE) for details.
