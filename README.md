# App Paths

A cross-platform lib that retrieves the important paths for a cross-platform app. The path returned is guaranteed to exist.

## Supported Paths

|         | Description                                      | Android | Windows(UWP) | Linux                             |
| ------- | ------------------------------------------------ | ------- | ------------ | --------------------------------- |
| DataDir | The root directory where all app data are stored | ✓       | ✓            | ✓(next to the current executable) |
