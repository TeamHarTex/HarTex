import bootstrap
import sys

if sys.version_info.major < 3:
    print("x: \u001B[1;31mfatal: \u001B[0mthis script is running in Python 2")
    print("x: \u001B[1;31mfatal: \u001B[0mthis script requires Python 3 or above")
else:
    bootstrap.run()
