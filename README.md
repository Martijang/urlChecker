# URL checker
Cli URL checker written in rust.

## how to use
checker.exe -u url(s)
example:

    checker.exe -u https://example.com https://www.google.com ..etc
or
to make post request use:

    checker.exe -u url.. -p true -b body(optional)

NOTICE: post request is mostly useless, especially if you are using this tool as 
url checking purposes.
### todo:
1. fix the terrible loop <- I freaked this I can't do it(at least for now)
2. add JSON request (optional)