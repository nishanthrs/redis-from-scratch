#!/opt/local/bin/bash

for i in `seq 1 150`
do
    # Execute in async fashion
    redis-cli ping &
    redis-cli echo "Hello World!" &
done
# Wait for all 150 requests to be sent and then exit execution
wait
