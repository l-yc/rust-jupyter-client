# Adding a new SHELL message

These are the steps to add a new Command/Response pair:

- Add a new enum variant for `Command`
- Add a new implementation to turn the command into a wire message (`Command::into_wire`)
- Add a new enum variant for `Response`
- Add a new deserializable struct for response content
- Add an entry to match the `msg_type` in `WireMessage::into_response`
- Add `into_packets` and `message_parsing` tests in `wire.rs`

# Messages requiring implementation

## Client -> Kernel (SHELL)

- [x] `kernel_info_request`
- [x] `execute_request`
- [x] `inspect_request`
- [x] `complete_request`
- [x] `history_request`
- [x] `is_complete_request`
- [ ] `comm_info_request`
- [ ] `shutdown_request`
- [ ] `interrupt_request`

## Kernel -> Client (SHELL)

- [x] `kernel_info_reply`
- [x] `execute_reply`
- [x] `inspect_reply`
- [x] `complete_reply`
- [x] `history_reply`
- [x] `is_complete_reply`
- [ ] `comm_info_reply`
- [ ] `shutdown_reply`

## Kernel -> Client (IOPUB)

- [x] `stream`
- [x] `status`
- [ ] `display_data`
- [ ] `update_display_data`
- [x] `execute_input`
- [ ] `execute_result`
- [x] `error`
- [ ] `clear_output`

## Kernel -> Client (STDIN)

- [ ] `input_request`

## Client -> Kernel (STDIN)

- [ ] `input_reply`

## Kernel -> Client (CONTROL)

- [ ] `interrupt_reply`
