import distro
import platform
import psutil
import os

# System info as variables cuz its more readable
pretty_name = distro.name(pretty=True)
kernel = platform.release()

### Memory shit
memory = psutil.virtual_memory()
total = memory.total / (1024 ** 3)
used = memory.used / (1024 ** 3)
### Memory shit

cpu = model_line = [line for line in open("/proc/cpuinfo").read().split("\n") if "model name" in line][0].split(":", 1)[1].strip()
uptime = os.popen('uptime -p').read().strip()

# (yes i know theres likely a better way to print all this)
print(f""" ⢀⣀⠀⠀⠀⠀⠀⢀⣀⠀
⢠⣯⢬⣷⡀⠀⠀⣴⡯⢌⣧     os: {pretty_name}
⠸⣿⠀⠹⣷⠀⢸⡝⠀⢸⡿     kernel: {kernel}
⠀⠻⣧⣀⣿⣦⣼⡁⣠⣿⠃     memory: {used:.2f} / {total:.2f}
⠀⢀⡾⠋⠀⠀⠀⠈⣙⣯      cpu: {cpu}
⠀⣾⠀⠀⠀⠀⠀⠀⠀⠸⡆     uptime: {uptime}
⢰⡧⢄⢰⡆⠀⢰⡆⡠⢄⣧ 
⠀⠳⣼⣤⣤⣤⣤⣤⣧⠾⠁     ⠀
""")
