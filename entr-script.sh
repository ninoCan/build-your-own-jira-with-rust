#!/bin/zsh

while true; do
    # Watch for changes in files with extension .js (adjust as needed)
    find jira-wip | entr -d sh -c """
    # Run your test command
    cargo test --bin jira-wip

    # If the test succeeds, add changes
    if [ $? -eq 0 ]; then
        git add .

        # Stop the entr loop
        pkill -f 'entr.*sh'
    fi
    """
done

