from web.api_generator import ServerBuilder


class ApplicationManager:
    def run(self):pass


class LocalCommonManager(ApplicationManager):
    def __init__(self, server_builder: ServerBuilder):
        self.server_builder = server_builder

    def run(self):
        self.server_builder.build_and_run()

