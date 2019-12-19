from collections import deque, defaultdict
from functools import reduce
from typing import NamedTuple, Dict, Callable, List, Any

from flask import request

from api_provider.base_provider import AbstractAllowedParametersChecker, BaseProvider


class Endpoint(NamedTuple):
    name: str
    handler: Callable
    methods: List = []
    paths: List = []

    # def add_handler(self, handler: Callable):
    #     midpoint = len(self.handlers) // 2  # for 7 items, after the 3th
    #     lst = self.handlers[0:midpoint] + deque(handler) + self.handlers[midpoint:]
    #
    # def add_pre_handler(self, handler: Callable):
    #     self.handlers.appendleft(handler)
    #     return self
    #
    # def add_post_handler(self, handler: Callable):
    #     self.handlers.append(handler)

    def add_path(self, path: str):
        self.paths.append(path)

    def __add__(self, o):
        if self.handler is not o.handler:
            raise ValueError("Handler are not the same")
        Endpoint(handler=self.handler, methods=self.methods + o.methods, paths=self.paths + o.paths)


class ServerBuilder(object):
    endpoints: List[Endpoint] = []

    def add_endpoint(self, endpoint: Endpoint):
        pass

    def build_and_run(self, port):
        map_path_endpoints = defaultdict(list)

        for e in self.endpoints:
            pass
            # for p in e.paths:
            #     map_path_endpoints[p].append(e)

        from flask import Flask
        app: Flask = Flask(__name__, instance_relative_config=True)

        for path, entities in map_path_endpoints.items():
            endpoint: Endpoint = reduce(lambda x, y: x + y, entities)
            app.add_url_rule(path, endpoint.name, endpoint.handler, methods=endpoint.methods)

        # index.methods = ['GET', 'OPTIONS']

        # app.add_url_rule('/', 'index2', index, {"methods": 'POST'})
        # app.add_url_rule('/', 'index2', index, {"methods": ['GET', 'OPTIONS']})

        print(app.url_map)
        print(app.view_functions['index2'])

        from waitress import serve

        serve(app, host="0.0.0.0", port=8080)


class WaitressFluskServerProxyBuilder(ServerBuilder):
    endpoints = []

    def __init__(self, checker: AbstractAllowedParametersChecker):
        self.checker = checker

    def add_endpoint(self, path, name, proxy):
        self.endpoints.append({'path': path, 'name': name, 'proxy': proxy})

    def build(self):
        from flask import Flask
        app: Flask = Flask(__name__, instance_relative_config=True)

        def create_endpoint(path=None, name=None, proxy: BaseProvider = None, methods=None):
            @app.route(path, methods=methods, endpoint=f"{name}")
            def execute():
                return proxy.execute(request.method, request.args)

        for path, name, proxy in self.endpoints:
            create_endpoint(path, name, proxy, self.checker.allowed_parameters)

        from waitress import serve

        serve(app, host="0.0.0.0", port=8080)


# class WaitressFluskServerBuilder(ServerBuilder):
#     def __init__(self, port=None, **options):
#         self._port = port if port else 8080
#
#     def add_endpoint(self, endpoint: Endpoint): pass
#
#     def build_and_run(self, **kwargs): pass


# class AbstractApp:
#     def add_endpoint(self, endpoint, method: str, function):
#         index.provide_automatic_options = False
#         if not function.methods:
#             function.methods = []
#         function.methods += method.upper()
#         index.methods = ['GET', 'OPTIONS']
#         # app.add_url_rule('/', f"{endpoint}_{method}", index, {"methods":['GET']})


def post_handler():
    return "post"


def get_handler():
    return "get"


if __name__ == '__main__':
    # app.route(route)(view_func)
    # app.view_functions["/hello"] = "hello resp"
    # app.view_functions["/hello"] = lambda r: "hello resp"
    print("start")

    from flask import Flask

    app: Flask = Flask(__name__, instance_relative_config=True)

    # from collections import defaultdict
    #
    # l = []
    # l.append(f)
    # l.append(s)
    #
    # d = defaultdict(list)
    #
    # d[f].append("hello f")
    # d[f].append("hello f2")
    #
    # d[s].append("hello s1")

    # print(d)
    # hello.lol = "rrr"
    # print(**hello)
    # print("end main")
    # l = ['e', 'l', 'l', 'o']
    # reduce1 = reduce(lambda x, y: x + y, l, 'h')
    #
    # app.add_url_rule('/users', "users_get", get_handler, {"methods": ['GET']})
    # app.add_url_rule('/users', "users_post", post_handler, {"methods": ['POST']})
    #
    # app.view_functions
    #
    # from waitress import serve
    #
    # serve(app, host="0.0.0.0", port=8080)
    # print(reduce1)

    # d = {"name":"Pete", "age":18}
    # name, age = d
    # print(f"{name} {age}")
