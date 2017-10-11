from apistar import Include, Route, annotate, render_template
from apistar.frameworks.wsgi import WSGIApp as App
from apistar.handlers import docs_urls, static_urls
from apistar.renderers import HTMLRenderer
import random
import yaml

with open('things-to-check.yml', 'r') as things_file:
    things = yaml.safe_load(things_file)

@annotate(renderers=[HTMLRenderer()])
def random_thing(item: int = None):
    if item is None:
        item = random.randrange(len(things))
    return render_template('index.html',
        item=item,
        thing=things[item],
    )


routes = [
    Route('/', 'GET', random_thing),
    Include('/docs', docs_urls),
    Include('/static', static_urls),
]

settings = {
    'TEMPLATES': {
        'ROOT_DIR': 'templates',     # Include the 'templates/' directory.
        'PACKAGE_DIRS': ['apistar']  # Include the built-in apistar templates.
    }
}

app = App(
    routes=routes,
    settings=settings,
)
