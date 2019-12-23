# app = Flask(__name__)
from copy import deepcopy
from pprint import pprint

# @app.route('/')
# def hello_world():
#     return 'Hello World!'
#
#
# if __name__ == '__main__':
#     app.run()
#
#
# import requests
#
# resp = requests.get('https://todolist.example.com/tasks/')
# if resp.status_code != 200:
#     # This means something went wrong.
#     raise ApiError('GET /tasks/ {}'.format(resp.status_code))
# for todo_item in resp.json():
#     print('{} {}'.format(todo_item['id'], todo_item['summary']))
from functools import reduce

from api_provider.base_provider import BaseProvider
from api_provider.proxy_provider import ProxyProvider
from managers.manager import LocalCommonManager
from parser.providers import YamlConfigProvider, ConfigProvider
from provider.provider_factories import DefaultProviderFactory, ProviderFactory
from web.api_generator import WaitressFluskServerProxyBuilder
from web.endpoint.endpoint_factory import CommonEndpointFactory
import os

if __name__ == '__main__':
    checker = None

    config_provider: ConfigProvider = YamlConfigProvider('config.yml')
    endpoints_definition, providers_definition = config_provider.read()


    def get_provider_map(providers_definition:dict) -> dict:
        factories = [DefaultProviderFactory(name, definition) for name, definition in providers_definition.items()]
        filtered_factories = [f for f in factories if f.is_valid()]
        providers = [x.get_provider() for x in filtered_factories]
        return reduce(lambda m, p: {**m, f'{p.name}': p}, providers, {})


    provider_map = get_provider_map(providers_definition)


    def get_endpoints():
        endpoint_factories = [CommonEndpointFactory(providers=provider_map, name=n, definition=d)
                              for n, d in endpoints_definition.items()]
        filtered_factories = [f for f in endpoint_factories if f.is_valid()]
        # filtered_factories = filter(lambda x: x.is_valid(), endpoint_factories)
        return map(lambda x: x.create_endpoint(), filtered_factories)


    endpoints = get_endpoints()

    server_builder = WaitressFluskServerProxyBuilder()
    for e in endpoints:
        server_builder.add_endpoint(e)

    local_common_manager = LocalCommonManager(server_builder)
    local_common_manager.run()
