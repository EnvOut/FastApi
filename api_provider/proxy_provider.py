import requests
from werkzeug.local import LocalProxy

from api_provider.base_provider import BaseProvider, AllowedParametersChecker, AnyAllowedParametersChecker, \
    NoAllowedParametersChecker, AbstractAllowedParametersChecker, http_methods

# import grequests

default_allowed_methods = http_methods
default_method = default_allowed_methods[0]


def is_allowed_method(method):
    return bool(method) and method in default_allowed_methods


def parse_allowed_parameters(parameters) -> (bool, AbstractAllowedParametersChecker):
    if parameters is None:
        parameters = True

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


class ProxyProvider(BaseProvider):
    def __init__(self, name: str, path: str, schema: str = 'https', allowed=None, **kwargs):
        if allowed is None:
            allowed = {}

        self.name = name
        self._config = {}

        self._path = path
        self._schema = schema
        self._methods = set([m.upper() for m in allowed.get('methods', default_allowed_methods)])

        self._config['is_allowed_params'], self._config['params'] = parse_allowed_parameters(allowed.get('parameters'))

    def is_valid(self):
        return bool(self._path) and self.__verify_methods()

    def __verify_methods(self):
        return bool(self._methods) and len(self._methods) > 0 \
               and all(elem in default_allowed_methods for elem in self._methods)

    def execute(self, config: LocalProxy):
        params_checker: AbstractAllowedParametersChecker = self._config['params']

        params = config.args
        method = config.method

        if not params:
            params = {}

        # if not params_checker:
        #     raise ValueError("Request parameters are not permitted")
        # elif not params_checker.is_all_allowed(list(params.keys())):
        #     raise ValueError(f"Permitted only {params_checker.allowed_parameters}")
        if not params_checker.is_all_allowed(list(params.keys())):
            raise ValueError(f"Permitted only {params_checker.allowed_parameters}")

        resp = requests.request(method, f'{self._schema}://{self._path}', params=params)
        return resp.text
