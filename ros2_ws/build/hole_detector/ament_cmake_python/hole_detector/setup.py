from setuptools import find_packages
from setuptools import setup

setup(
    name='hole_detector',
    version='0.0.0',
    packages=find_packages(
        include=('hole_detector', 'hole_detector.*')),
)
