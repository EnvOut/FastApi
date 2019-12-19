import yaml


class YamlProvider:
    def __init__(self, file_path):
        self._file_path = file_path
        self.providers = []
        self.endpoints = []

    def read(self):
        with open(self._file_path) as file:
            config = yaml.full_load(file)
            self.providers = config['providers']
            self.endpoints = config['endpoints']
