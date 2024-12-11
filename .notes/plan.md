
# REMOVE p2p, just go for standard server
- [ ] set up streaming listener for it
- [ ] create grpc server to enable discovery of other cat
- [ ] sync state

# configuration tool
- [ ] use tauri to create a helpful configuration tool

# sprites
- [ ] sprite sheet for cat / find one
  - https://stackoverflow.com/questions/74358029/bevy-have-multiple-sprites-sheets-per-entity

- [ ] figure out how to change sprite animation state
- [ ] figure out all the different states of the cat
- [ ] entire state machine with the different interactions of the cat

# cat behaviors
- [ ] meow when message received
- [ ] when message arrives, wait 10 seconds, and if theres no click, drag mouse around


# bevy
- [x] figure out how to emit events when bridge receives something
- [ ] state machine to handle all the things
- [ ] figure out movement for the cat (abstracted really nicely)

# SYSTEM todo
- [ ] rewrite the systems not in terms of what they do but try not to
      rely on multithreading as much
