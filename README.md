# tfetch
My own fetch program. (tubular-fetch)

# Installation
This is a bit tricky, so stay with me here.

- make a folder in your home directory called tfetch and move tfetch.py into it
- in your terminal do ```cd tfetch``` to move into the tfetch directory
- from the tfetch directory, run ```python -m venv .venv```
- then run ```source .venv/bin/activate```
- then install the necessary libraries that tfetch needs with ```pip install psutil``` and ```pip install py-cpuinfo```

**almost done stay with me**

- make a file called **tfetch** (no file extension) in your tfetch folder
- within the **tfetch** file, add:
```
#!/bin/bash
source ~/tfetch/.venv/bin/activate
exec python ~/tfetch/tfetch.py "$@"
```

- then run ```chmod +x tfetch```
- and finally, ```sudo mv tfetch /usr/local/bin/tfetch```

i prolly messed something up so if you need help join my dc server
https://discord.gg/cnEKvAB83r
