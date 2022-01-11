# How to create and validate JWTs using Rust and RSA Keys + JWKs

## Getting Started

1. Create new private RSA Key using `openssl`

```bash
openssl genrsa -out prv_key.pem 2048
```

2. Generate `jwk.json` from the private key (make sure you run `pip install -r requirements.txt`) 

```bash
python3 create_jwk.py prv_key.pem
```