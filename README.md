# Secret Hitler

We try to build a command line based version of secret hitler that can be played over the internet. The project is split in a client using some ncurses like interface and a server connecting the clients.

## Required features / Open Questions

### Both
- players are able to reconnect
- it needs to run very stable!

### Client
- client should work cross platform (at least windows & linux)
- what terminal size should we use? 

### Server
- max 1ish GB memory footprint (just putting this out there)


 states
 
pre_game
identity_assignment
election
    override_vote
legislative_session
discussion
executive_action
    execution
    investigate_loyality
    policy_peek
     "veto power?"
execution_vote
execution
game_over