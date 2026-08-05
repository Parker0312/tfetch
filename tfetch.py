import os
import socket
import platform
import cpuinfo
import psutil

# ascii art
print("""⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⢀⡴⢲⡄⠀⠀⠀⢀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⢀⡞⠀⠀⡇⠀⢀⡴⠋⠁⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⣸⠁⠀⠀⡇⣠⠟⠀⠀⠀⣼⣠⣤⣤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⣿⠀⢠⢤⡿⠃⣀⠀⢀⡞⠉⠁⠀⠀⠈⠙⠶⡄⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⣻⣰⡏⣼⢁⡴⠋⣰⠋⠀⠀⠀⠀⠀⠀⠀⠀⠹⡄⣀⡀⠀⠀⠀⠀
⠀⠀⠀⢀⡾⠋⠉⠁⠀⠙⠁⠞⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⡈⢷⠀⠀⠀⠀
⠀⠀⠀⣼⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄⠀⠀⠀⠀⠀⠀⠀⢀⣧⡾⠁⠀⠀⠀
⠀⠀⠀⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄⠀⠀⠀⠀⠀⠀⠀⣾⠃⠀⠀⠀⠀⠀
⠀⠀⠀⣹⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⠁⠀⠀⡀⠀⠀⢰⡾⠋⠀⠀⠀⠀⠀⠀
⠀⠀⠀⢻⡇⣀⡀⠀⠺⣿⠇⠀⣀⣤⣄⣀⣠⣬⣥⣤⠾⠛⠁⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠉⠛⠓⠂⠤⠤⠖⠊⠉⠉⠉⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
      """)

# Divider
print("✩｡:*•.─────  ❁ ❁  ─────.•*:｡✩\n")

# basic stuff
host = socket.gethostname()
os = platform.platform()
cpu = cpuinfo.get_cpu_info()["brand_raw"]

# disk
disk = psutil.disk_usage("/")
total_diskgb = disk.total / (1024 ** 3)

# memory
memory = psutil.virtual_memory()
total_gb = memory.total / (1024 ** 3)

# print the info 
# (goodluck reading this, I didn't format it in a readable way)
print("\033[34mHost: \033[0m" + host)
print("\033[34mOS: \033[0m" + os)
print ("\033[34mCPU: \033[0m" + cpu)
print ("\033[34mMemory: \033[0m~" + str(round(total_gb)) + " GiB")
print("\033[34mDisk: \033[0m~" + str(round(total_diskgb)) + " GiB") 
print("")

