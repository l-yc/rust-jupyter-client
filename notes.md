# Adding a new SHELL message

These are the steps to add a new Command/Response pair:

- Add a new enum variant for `Command`
- Add a new implementation to turn the command into a wire message (`Command::into_wire`)
- Add a new enum variant for `Response`
- Add a new deserializable struct for response content
- Add an entry to match the `msg_type` in `WireMessage::into_response`
- Add `into_packets` and `message_parsing` tests in `wire.rs`
