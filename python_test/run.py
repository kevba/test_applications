import sys

from server import get_server

PORT = 8000

# Starts the HTTP-server
# If a port is given, it tries to use that port; Otherwise it'll use 8000 as
# default.
if __name__ == '__main__':
    port = PORT
    try:
        port = int(sys.argv[1])
        print("A port was supplied: {}".format(port))
    except IndexError:
        print("No port supplied, using default: {}".format(port))

    server = get_server(port)
    print("serving at {}".format(port))
    server.serve_forever()
