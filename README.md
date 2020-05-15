# Secret Hitler

We try to build a command line based version of secret hitler that can be played over the internet. The project is split in a client using some ncurses like interface and a server connecting the clients.

## usage

Start the server
```
cargo run --bin server local.yaml
```

Start the client
```
cargo run --bin client local.yaml
```

## Features needed before playable
- win condition: fascist policies, liberal policies, hitler elected chancelor if fascist policies > ?, hitler executed
- game over state entered from win condition check, shows who won (and optionally restarts the game)
- executive actions, these are triggered on a fascist policy being enacted. which is determined based on the fascist policy count and player number 
    5-6: 1: nothing, 2: nothing, 3: policy peek, 4: execution, 5: execution
    7-8: 1: nothing, 2: investigate loyality, 3: special election, 4: execution, 5: execution
    9-10: 1: investigate loyality, 2: investigate loyality (can't be the same person), 3: special election, 4: execution, 5: execution
    veto power is unlocked after/at? 5 fascist policies (always needed)
    - execution (always needed) 
    - investigate_loyality (7-10)
    - policy peek (5-8)
    - veto power (always needed)
    - special election (7-8)
- eligibility check in nomination phase
- block people joining after game was started
- chaos phase if election fails 3 times
- create observer type for executed players (can't act but see the game), could also join new players after game started like this

## Roadmap

### Playable for 5-6
- [x] server client communication
- [x] sync threads with mutex
- [x] state machine client
- [x] state machine server
- [x] login
- [x] pre game state
- [x] identity assignment
- [x] nomination
- [x] election
- [x] legislative session
- [x] implement policy deck
- [x] enable efficent testing with tmux
- [x] win condition
- [x] game over state
- [x] policy peek
- [x] chaos phase
- [ ] veto power
- [ ] block players from joining
- [ ] observer state
- [ ] execution
- [x] win condition hitler executed
- [x] tmux script for 5-6
- [ ] beta testing for 5-6

### Playable for 7-8
- [ ] investigate loyality
- [ ] beta testing for 7-8

### Playable for 9-10
- [ ] tmux script for 9-10 players
- [ ] special election
- [ ] beta testing for 9-10

### nice to haves
- [ ] rendering performance improvements (windows!)
- [ ] password protected login
- [ ] clients only see public information
- [ ] make everything prettier
- [ ] show all information eg. policy deck numbers, ... 

