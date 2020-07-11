from django.urls import path
from django.urls import re_path

from frontend.views import index_view

app_name = 'frontend'
urlpatterns = [
    re_path(r'^(?:.*)/?$', index_view, name='index'),
]
