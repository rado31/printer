# Steps 

* Install 'node-v20.15.0-x64.msi'
* Unzip printer.zip
* Open the terminal and enter to printer folder
* run command to check printer: ```node test_printer.js```

In printer folder there is .env file with fields:

```
# default values are in terminals
# 3540 -> 0DD4, 349 -> 015D

PRINTER_VID=3540
PRINTER_PID=349
```

Change if your VID and PID are difference
