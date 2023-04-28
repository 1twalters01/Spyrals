
ORIGINS = [
    "http://127.0.0.1:8080",
    "http://localhost:3000",
]

MIDDLEWARE_SETTINGS={
    'allow_origins': ORIGINS,
    'allow_credentials': True,
    'allow_methods': ["*"],
    'allow_headers': ["*"],
}