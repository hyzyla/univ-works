import pathlib

from aiohttp import web

from routes import setup_routes

app = web.Application()
# app['config'] = load_config(str(pathlib.Path('.') / 'config' / 'polls.yaml'))
# setup_routes(app)
web.run_app(app, host="localhost", port=8080)