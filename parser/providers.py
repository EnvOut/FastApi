from typing import List, Tuple

import yaml


class ConfigProvider:

    def read(self) -> Tuple[List[str], List[str]]:
        """return (endpoints, providers)"""
        pass


class YamlConfigProvider(ConfigProvider):
    def __init__(self, file_path):
        self._file_path = file_path
        self.providers = []
        self.endpoints = []

    def read(self) -> Tuple[List[str], List[str]]:
        with open(self._file_path) as file:
            config = yaml.full_load(file)
            self.endpoints = config['endpoints']
            self.providers = config['providers']

            return self.endpoints, self.providers
