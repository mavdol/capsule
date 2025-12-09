from .decorator import task
from . import app
from .app import TaskRunner

# Export the TaskRunner instance for componentize-py
exports = app.exports

