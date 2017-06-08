import json
import socket
import sys
from uuid import uuid4

SOCKET_TIMEOUT = 2
SOCKET_NAME = '/tmp/iris'


class Connection(object):
    def __init__(self):
        self.socket = None

    def _connect(self):
        self.socket = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        self.socket.settimeout(SOCKET_TIMEOUT)
        self.socket.connect(SOCKET_NAME)

    def write(self, data):
        if self.socket is None:
            self._connect()
        self.socket.sendall(data.encode('utf-8'))

    def read(self, bufsize=4096):
        if self.socket is None:
            self._connect()

        result = ''

        while True:
            try:
                data = self.socket.recv(bufsize)
                result += data
                if sys.getsizeof(data) < bufsize:
                    break

            except socket.timeout:
                print('socket timed out')
        return result.decode('utf-8')

    def __enter__(self):
        """ Connect to socket. """
        if self.socket is None:
            self._connect()
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        """ Close socket. """
        if self.socket is not None:
            self.socket.shutdown(socket.SHUT_RDWR)
            self.socket.close()


def call_iris(method, params):
    request = {
        'jsonrpc': '2.0',
        'method': method,
        'params': params,
        'id': uuid4().int
    }

    try:
        with Connection() as c:
            c.write(json.dumps(request))
            return handleResponse(c.read())
    except IOError:
        print("calling Iris failed, is Iris running?")
        return None


def handleResponse(data):
    data = json.loads(data)
    if 'result' in data:
        return data['result']
    return None
