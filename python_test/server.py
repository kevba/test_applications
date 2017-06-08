import json

from BaseHTTPServer import BaseHTTPRequestHandler, HTTPServer
from SocketServer import ThreadingMixIn
from connection import call_iris


class TestHandler(BaseHTTPRequestHandler):
    def _set_headers(self, status_code=200, content_type='application/json'):
        """ Function to easily set the headers.

        :param status_code : The status_code that will be returned.
        :param content_type: The content-type that will be returned, defaults
                             to 'application/json'
        """
        self.send_response(status_code)
        self.send_header('Content-type', content_type)
        self.end_headers()

    def handle_input(self, port_number, port_type):
        """ Calls Iris to read the digital pin with given port_number.

        :param port_number: The port number of the pin that will be read.
        :param port_type: The type of port that will be read
                           (analog or digital).
        """
        return call_iris('read_{}'.format(port_type), {
            'port_number': int(port_number)
        })

    def handle_output(self, port_number, port_type, value):
        """ Calls Iris to read the digital pin with given port_number.

        :param port_number: The port number of the pin to write to.
        :param port_type: The type of port to write to (analog or digital).
        :param value: The value to write tot the pin.
        """
        return call_iris('write_{}'.format(port_type), {
            'port_number': port_number,
            'value': value
        })

    def do_HEAD(self):
        self._set_headers()

    def do_GET(self):
        """do_GET gets called for every GET-request to this server.
        """
        # split the url into a list containing each part of the url.
        # The list should look like ['api', 'analog', 'input', '1']
        path_parts = self.path.split('/')[1:]
        try:
            if path_parts[0] == 'api' and path_parts[2] == 'input':
                if path_parts[1] not in ['analog', 'digital']:
                    self.send_error(404)
                    return

                result = self.handle_input(int(path_parts[3]),
                                           path_parts[1])
                if result is None:
                    self.send_error(500)
                    return

                data = json.dumps({"value": str(result)})
                self._set_headers()
                self.wfile.write(data)
        except(IndexError):
            pass
        self.send_error(404)

    def do_POST(self):
        """do_POST gets called for every POST-request to this server.
        """
        path_parts = self.path.split('/')[1:]
        data = json.loads(
            self.rfile.read(int(self.headers.getheader('content-length')))
        )
        try:
            if path_parts[0] == 'api' and path_parts[2] == 'input':
                if path_parts[1] not in ['analog', 'digital']:
                    self.send_error(404)
                    return

                result = self.handle_output(
                    int(path_parts[3]),
                    path_parts[1],
                    int(data[u'value'])
                )

                if result is None:
                    self.send_error(500)
                    return
                data = json.dumps({"value": str(result)})
                self._set_headers()
                self.wfile.write(data)
                return
        except(IndexError):
            pass
        self.send_error(404)


class ThreadedHTTPServer(ThreadingMixIn, HTTPServer):
    """Handle requests in a separate thread."""


def get_server(port):
    """ Sets up the server with the given port.
    :param port: The port for the server.
    """
    server_address = ("", port)
    return ThreadedHTTPServer(server_address, TestHandler)
