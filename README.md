# CarPlay Client

CarPlay client application written for "AutoBox" dongles sold on AliExpress. Heavily influenced from <https://github.com/electric-monk/pycarplay> and <https://github.com/harrylepotter/carplay-receiver>, which are python client applications with similar objectives. The credit for the reverse engineering efforts on the "AutoBox" dongles and initial implementation goes to **electric-monk** (<https://github.com/electric-monk>) who wrote an HTML based CarPlay client for Tesla's infotainment system in `pycarplay`. A more native implementation was written by **harrylepotter** (<https://github.com/harrylepotter>) which was used to create a rough architecture for this application.

The application is written in Rust and depends on `libinput` and `libmpv`.

## Architecture

The application is split up into several "layers". You can loosely think of the interaction between the layers as an implementation of the Model-View-Controller design pattern. In this analogy, the "model" would be the "link layer", the "view" would be the "player_layer", and the "controller" would be the "input_layer". The "client" essentially encapsulates all three layers and orchestrates the communication between them.

## Motivation

I don't like most off-the-shelf car infotainment systems nor most OEM solutions, so I wanted to make my own fully fledged car computer. There certainly are thousands of projects aiming to do the same, but I wanted to learn about the challenges a car company (or more like the development team outsourced by the car company) faces while putting one of these things together. CarPlay is at this point a standard feature, so my infotainment system will be incomplete without a program like this.

While electric-monk and harrylepotter's implementations are a great base, I could not get the performance or feature set I desired in python. I also wanted to learn more about Rust, so I saw this as a great opportunity to work on my car computer and have a Rust project. Who says you can't have your cake and eat it too?

## Credits

As noted before, this application would not be possible without the efforts of electric-monk and harrylepotter. I take no credit in reverse engineering the dongle's protocol nor the initial development of the client application.
