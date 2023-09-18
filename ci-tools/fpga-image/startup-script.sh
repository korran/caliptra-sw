#!/bin/bash

# Stop spewing kernel noise to the UART
echo 3 > /proc/sys/kernel/printk

echo "36668aa492b1c83cdd3ade8466a0153d --- Command input"
echo Available commands:
echo "  runner-jitconfig <base64>"
echo "  login"
echo -n "> "
read cmd
cmd_array=($cmd)
if [[ "${cmd}" == "login" ]]; then
    login
elif [[ "${cmd_array[0]}" == "runner-jitconfig" ]]; then
    echo "Executing GHA runner"
    su runner -c "./run.sh --jitconfig \"${cmd_array[1]}\""
    echo "GHA runner complete"
else
    echo "Unknown command ${cmd}"
fi
echo "3297327285280f1ffb8b57222e0a5033 --- ACTION IS COMPLETE"
sleep 1
shutdown -h now