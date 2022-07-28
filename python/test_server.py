from random import randint
import socket
import time
# sock.bind(("127.0.0.1", 9099))
while True:
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.connect(("127.0.0.1", 9098))
        data = str(randint(0, 45)).encode("utf-8")
        print(f'UwU Sending {len(data)} bytes to server')
        sock.send(data)
        sock.close()
    except KeyboardInterrupt:
        sock.close()
    except Exception as e:
        print(f'UwU Failed')
    finally:
        time.sleep(1)
