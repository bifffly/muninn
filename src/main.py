import os
import socket

import conf

class TCPServer:
    host = conf.host
    port = int(conf.port)

    def start(self):
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.bind((self.host, self.port))
        s.listen(5)

        print("Listening at", s.getsockname())

        while True:
            conn, addr = s.accept()
            print("Connected by", addr)
            req = conn.recv(1024)
            res = self.handle_request(req)
            conn.sendall(res)
            conn.close()

    def handle_request(self, req):
        return data

class RydjaServer(TCPServer):
    def handle_request(self, req):
        filepath = conf.homedir + req.decode()
        if os.path.exists(filepath) and os.path.isfile(filepath):
            with open(filepath, 'rb') as f:
                return f.read()
        elif req.decode() == '/':
            return self.handle_request(str.encode(conf.autohome))
        else:
            return b"""
                @rydja(Error)
                @head(File not found)
            """

if __name__ == '__main__':
    server = RydjaServer()
    server.start()
