### Start the emulator

If you do not already have an instance running, start the emulator:

1.  Start a new emulator instance:

    ```posix-terminal
    ffx emu start --headless
    ```

    When startup is complete, the emulator prints the following message and
    returns:

    ```none {:.devsite-disable-click-to-copy}
    Logging to "{{ '<var>' }}$HOME{{ '</var>' }}/.local/share/Fuchsia/ffx/emu/instances/fuchsia-emulator/emulator.log"
    Waiting for Fuchsia to start (up to 60 seconds)........
    Emulator is ready.
    ```

1.  Start a package server to enable the emulator to load software packages:

    ```posix-terminal
    fx serve-updates
    ```
