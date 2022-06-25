# Text-to-speech for accessibility

## Overview

The text-to-speech (tts), used by accessibility is still being developed and is currently in an early stage. Assistive technology often need to convey information in different ways, one of them being speech. The Screen Reader uses the tts services to produce output that can be heard by blind and low-vision users.

There are a few participants in the tts world that one should be aware of:
1. **speaker interface**: is the interface used by assistive technology that wants to produce speech, by opening a registered tts engine with the tts manager, and then dispatching speech output requests. For now, this is just a c++ class, implemented by the tts manager, as they are running all inside the same component (a11y manager). In the future, this will probably have its own FIDL interface.
1. **Tts manager interface**: is the interface implemented by the component responsible for managing registered tts engines and responding to speakers requests.
1. **tts engine binary**: is the binary responsible for producing speech (normally in [pcm format][pcm]). The output produced by the engine is then sent for an audio service to be played. There is not at the moment a tts engine binary ported to run on Fuchsia, and, for now, each individual product configuration must bring its own tts engine binary.
1. **Engine interface**: is the Fuchsia FIDL interface implemented by tts engines that want to produce speech in the system.

## Tts interfaces

The current interfaces [can be found here][tts]. They are considered experimental and will be developed further once a tts engine is ported to Fuchsia.

## Clients

For now, only the Screen Reader is a client of the tts-related protocols.

## Future work

- Port a tts engine to run on Fuchsia natively.
- Improve the tts interfaces.
- Allow other clients other than accessibility services to use tts services.

[pcm]: https://en.wikipedia.org/wiki/Pulse-code_modulation#:~:text=Pulse%2Dcode%20modulation%20(PCM),and%20other%20digital%20audio%20applications.
[tts]: https://cs.opensource.google/fuchsia/fuchsia/+/master:sdk/fidl/fuchsia.accessibility.tts/tts_manager.fidl;l=1