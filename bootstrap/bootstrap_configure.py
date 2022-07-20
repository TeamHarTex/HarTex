import os

LINES = [
    "[cache]\n",
    'backend = "postgres"\n',
    "\n",
    "[loadbal]\n",
    "servers = [\n",
    '    { type = "rest", address = "127.0.0.1:8000" }\n',
    "]\n"
]


def run_configure():
    print("x: checking for existing build configuration file(s)")

    if os.path.exists("buildconf.toml"):
        print("x: \u001B[1;31merror: \u001B[0mone or more configuration file(s) already exist(s). exiting.")
        print("x: \u001B[1;33mnote: \u001B[0mif you want to reconfigure the build environment, run the `reconfigure` "
              "command.")
        return
    else:
        print("x: creating build configuration file")

        file = open("buildconf.toml", "x")
        file.writelines(LINES)

        print("""x: default configuration:
    \u001B[1mcache backend: \u001B[0mpostgres
    \u001B[1mload balancer: \u001B[0m1 server(s) to load balance:
                  - \u001B[1mtype: \u001B[0mrest; \u001B[1maddress: \u001B[0m127.0.0.1:8000
        """)
        file.close()
