# List of notes

A list of notes I will keep to better my understanding of both Rust and third party libraries used, mostly PortMidi.

## Note 1: Segfaults with IO ports

### Problem: programs segfaulted initially when creating IO structs

PortMidi differentiates MIDI devices into I/O categories, so you could separate an input channel from an output channel. This makes it easier to create more customizable programs across multiple devices only selecting what you want.

I'm only working with a single Launchpad here, so grabbing both IO ports and grouping it into a struct is what I wanted. However, any program doing this naively would crash.

### Solution: do not let the initial PortMidi instance fall out of scope

With no lifetime bound to it, during my initial struct construction, I let the PortMidi instance fall out of scope, which conveniently will kill any ports created from it later on. The best thing you can do is bind a reference to the original PortMidi instance in your struct.

```rust
// keep the PortMidi instance with the IO ports
pub struct Launchpad {
	pub input: portmidi::InputPort,
	pub output: portmidi::OutputPort,
	pub midi: portmidi::PortMidi,
}
```
