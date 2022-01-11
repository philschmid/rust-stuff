import argparse
import copy
import time

from jwcrypto import jwt, jwk

def main(args):
    # read pem key
    with open(args.key) as f:
        pem_data = f.read()

    # generate jwk
    pem_data_encode = pem_data.encode("utf-8")
    key = jwk.JWK.from_pem(pem_data_encode)
  
    # write jwk.json
    with open("jwk.json", "w+") as fout:
        fout.write("{ \"keys\":[ ")
        fout.write(key.export(private_key=False))
        fout.write("]}")

if __name__ == '__main__':
  parser = argparse.ArgumentParser(
          )
      # positional arguments
  parser.add_argument(
      'key',
      help='The path to the key pem file. The key can be generated with openssl command: `openssl genrsa -out key.pem 2048`')
  main(parser.parse_args())