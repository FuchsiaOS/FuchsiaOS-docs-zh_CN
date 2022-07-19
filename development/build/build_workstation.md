# Build Workstation

Workstation (`workstation_eng`) is an open source reference design for Fuchsia.
Workstation is not a consumer-oriented product. Workstation is a tool for
developers and enthusiasts to explore Fuchsia and experiment with evolving
concepts and features.

Workstation does not come with strong security, privacy, or robustness
guarantees. Bugs and rapid changes are expected – to help improve Fuchsia,
please [file bugs and send feedback][report-issue].

## Get started with Workstation {#get-started-with-workstation}

To get started with Workstation, you need to be familiar with how to get the
Fuchsia source code, build Fuchsia images, and run Fuchsia on a device or
emulator – the instructions in this section are based on the
[Get started with Fuchsia][get-started-with-fuchsia] flow.

Workstation is designed to be used with an Intel NUC or the Fuchsia emulator
(FEMU).

*   {Intel NUC}

    To install Workstation on an Intel NUC, do the following:

    1.  Complete the [Download the Fuchsia source code][get-fuchsia-source]
        guide.
    2.  As part of [Configure and Build Fuchsia][build-fuchsia], set your build
        configuration to use the following Workstation product:

        ```posix-terminal
        fx set workstation_eng.x64 --release
        ```

    3.  Complete the [Install Fuchsia on a NUC][intel-nuc] guide.

*   {FEMU}

    To try Workstation on the Fuchsia emulator, do the following:

    1.  Complete the [Download the Fuchsia source code][get-fuchsia-source]
        guide.
    2.  As part of [Configure and Build Fuchsia][build-fuchsia], set your build
        configuration to use the following Workstation product:

        ```posix-terminal
        fx set workstation_eng.qemu-x64 --release
        ```

    3.  Complete the [Start the Fuchsia emulator][start-femu] guide.

<!-- Reference links -->

[report-issue]: /contribute/report-issue.md
[get-started-with-fuchsia]: /get-started
[get-fuchsia-source]: /get-started/get_fuchsia_source.md
[build-fuchsia]: /get-started/build_fuchsia.md
[intel-nuc]: /development/hardware/intel_nuc.md
[start-femu]: /get-started/set_up_femu.md
