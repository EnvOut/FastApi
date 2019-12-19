import distutils

import requests

# import grequests
from api_provider.base_provider import BaseProvider, AllowedParametersChecker, AnyAllowedParametersChecker, \
    NoAllowedParametersChecker, AbstractAllowedParametersChecker, http_methods
from parser.providers import YamlProvider

default_allowed_methods = http_methods
default_method = default_allowed_methods[0]


def is_allowed_method(method):
    return bool(method) and method in default_allowed_methods


def parse_allowed_parameters(parameters):
    if parameters is True:
        param_type = type(parameters)
        if param_type is list:
            return True, AllowedParametersChecker(parameters)
        elif param_type is bool:
            if (param_type):
                return True, AnyAllowedParametersChecker()
            else:
                return False, NoAllowedParametersChecker()
        else:
            raise ValueError(f"undefined behaviour for the type '{param_type}'")
    elif parameters is None:
        return True, default_allowed_methods
    else:
        return


class ProxyProvider(BaseProvider):
    def __init__(self, path: str, schema: str = 'https', allowed: dict = None, **kwargs):
        if allowed is None:
            allowed = {}

        self._config = {}

        self._path = path
        self._schema = schema
        self._methods = set([m.upper() for m in allowed.get('methods', default_allowed_methods)])

        self._config['is_allowed_params'], self._config['params'] = parse_allowed_parameters(allowed.get('parameters'))
        # self._config['is_allowed_params'] = allowed.get('parameters', 'True').lower() == 'true'

        # self._method = kwargs.get('method', 'GET').upper()

    def is_valid(self):
        return bool(self._path) and self.__verify_methods()

    def __verify_methods(self):
        return bool(self._methods) and len(self._methods) > 0 \
               and all(elem in default_allowed_methods for elem in self._methods)

    def execute(self, params: dict, method: str = None, **kwargs):
        params_checker: AbstractAllowedParametersChecker = self._config['params']

        if not params_checker:
            raise ValueError("Request parameters are not permitted")
        elif not params_checker.is_all_allowed(list(params.keys())):
            raise ValueError(f"Permitted only {params_checker.allowed_parameters}")

        return requests.request(method, self._path, params=params)


if __name__ == '__main__':
    # from parser.providers import YamlProvider
    #
    # provider = YamlProvider(r'../config.yml')
    # provider.read()
    # p = list(provider.providers.values())[0]
    #
    # # [x if x % 2 else x * 100 for x in range(1, 10)]
    # proxy_provider_list = [ProxyProvider(p) for p in provider.providers.values() if p['type'] == 'proxy']
    # print(proxy_provider_list)

    provider = YamlProvider(r'../config.yml')
    provider.read()
    pg_allowed = list(provider.providers.values())[-1]['allowed']

    print(len(pg_allowed))
    # r = parse_allowed_parameters(**pg_allowed)
