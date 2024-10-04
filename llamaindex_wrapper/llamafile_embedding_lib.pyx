# File: llamafile_embedding_lib.pyx

# distutils: language = c++
# cython: language_level = 3

import numpy as np
cimport numpy as np
from libcpp.string cimport string
from libcpp.vector cimport vector

from llamafile_embedding_interface import LlamafileEmbeddingInterface, init

cdef class PyLlamafileEmbedding:
    cdef LlamafileEmbeddingInterface _interface

    def __cinit__(self, model_path: str):
        self._interface = init(model_path)

    def get_embedding(self, text: str) -> np.ndarray:
        return self._interface.get_embedding(text)

    def get_embeddings(self, texts: list) -> np.ndarray:
        return self._interface.get_embeddings(texts)

# C API
cdef extern from *:
    ctypedef char* const_char_ptr "const char*"

cdef public api:
    PyLlamafileEmbedding* create_embedding(const_char_ptr model_path):
        return new PyLlamafileEmbedding(model_path.decode('utf-8'))

    void destroy_embedding(PyLlamafileEmbedding* embedding):
        del embedding

    np.ndarray[np.float32_t, ndim=1] get_single_embedding(PyLlamafileEmbedding* embedding, const_char_ptr text):
        return embedding.get_embedding(text.decode('utf-8'))

    np.ndarray[np.float32_t, ndim=2] get_multiple_embeddings(PyLlamafileEmbedding* embedding, vector[string] texts):
        return embedding.get_embeddings([text.decode('utf-8') for text in texts])