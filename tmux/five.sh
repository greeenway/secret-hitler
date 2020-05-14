#!/usr/bin/zsh

sh_folder='.'


tmux kill-session -t secrethitler
tmux new-session -s 'secrethitler' -d 'cd '"$sh_folder"'; cargo run --bin server local.yaml;zsh -i'
sleep 0.5 # give server some time to start


client_command='cd '"$sh_folder"'; cargo run --bin client local.yaml USER_NAME;zsh -i'

cmd=`echo ${client_command/USER_NAME/val}`
tmux split-window -h $cmd
cmd=`echo ${client_command/USER_NAME/lukas}`
tmux split-window -v $cmd
cmd=`echo ${client_command/USER_NAME/markus}`
tmux split-window -v $cmd
tmux select-pane -t 0
cmd=`echo ${client_command/USER_NAME/andi}`
tmux split-window -v $cmd
cmd=`echo ${client_command/USER_NAME/stefan}`
tmux split-window -v $cmd



tmux select-layout tiled


tmux select-pane -t 2
tmux setw synchronize-panes on
# for more players use a different window
#tmux new-window $client_command
#tmux split-window -h $client_command
tmux -2 attach-session -d