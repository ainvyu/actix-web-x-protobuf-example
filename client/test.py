import requests
import google.protobuf as protobuf
from test_pb2 import Character

character = Character(name="test")
print(character)

res = requests.post(url='http://localhost:18081',
                    data=character.SerializeToString(),
                    headers={'Content-Type': 'application/octet-stream'})
