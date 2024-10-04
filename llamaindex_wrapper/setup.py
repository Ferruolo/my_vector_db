# File: setup.py

import os
import sys
from setuptools import setup, Extension, find_packages
from Cython.Build import cythonize
import numpy

# Get the path of the virtual environment
venv_path = os.path.dirname(sys.executable)

# Determine the site-packages directory
if sys.platform == 'win32':
    site_packages = os.path.join(venv_path, 'Lib', 'site-packages')
else:
    site_packages = os.path.join(venv_path, 'lib', 'python' + sys.version[:3], 'site-packages')

# Get all the .so or .pyd files from the site-packages
def get_shared_libs():
    shared_libs = []
    for root, _, files in os.walk(site_packages):
        for file in files:
            if file.endswith('.so') or file.endswith('.pyd'):
                shared_libs.append(os.path.join(root, file))
    return shared_libs

extensions = [
    Extension(
        "llamafile_embedding_lib",
        ["llamafile_embedding_lib.pyx"],
        include_dirs=[numpy.get_include(), os.path.join(site_packages, 'numpy', 'core', 'include')],
        library_dirs=[site_packages],
        runtime_library_dirs=[site_packages],
        extra_objects=get_shared_libs(),
    ),
]

setup(
    name="LlamafileEmbedding",
    version="1.0",
    packages=find_packages(),
    ext_modules=cythonize(extensions),
    include_package_data=True,
    zip_safe=False,
    install_requires=[
        'llama-index',
        'torch',
        'numpy',
    ],
)