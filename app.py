# app = Flask(__name__)
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

    # app.add_url_rule('/users', "users_get", get_handler, {"methods": ['GET']})
    # app.add_url_rule('/users', "users_post", post_handler, {"methods": ['POST']})

    pprint(app.view_functions)

    resources = ["users", "files", "metas"]

    for r in resources:
        path = f"/{r}"
        @app.route(path, methods=["GET"], endpoint=f"get_{r}")
        def get_users():
            return "get"


        @app.route(path, methods=["POST"], endpoint=f"post_{r}")
        def post_users():
            return "post"

    app.run()
    from waitress import serve

    # serve(app, host="0.0.0.0", port=8080)
