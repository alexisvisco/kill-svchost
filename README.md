# Automated way to kill svchost

REMOVE THIS SHIT OF SVCHOST AND TAKE CONTROL OF YOUR COMPUTER !! :@

## Download

To download this program go to this link and get the latest version: [All releases are here](https://github.com/AlexisVisco/Kill-svchost-native/releases)

## Use

Run this program as administrator.

## What i'm doing ?

The script, every 8 seconds iterate through the output of the command : netstat -b -o -n. If a line contain svchost.exe the script get the PID of the processus and kill it by doing this taskkill /F /PID $PID_OF_PROCESS.

