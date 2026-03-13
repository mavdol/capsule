# Capsule Python Adapter

Execute Python and JavaScript code securely inside Capsule sandboxes from your Python applications.

## Installation

```bash
pip install capsule-run-adapter
```

## Usage

### Execute Python Code

```python
from capsule_adapter import run_python_sync

result = run_python_sync("""
print("Hello from Python!")
x = 5 + 3
x * 2
""")

print(result)  # "Hello from Python!\n16"
```

### Execute JavaScript Code

```python
from capsule_adapter import run_javascript_sync

result = run_javascript_sync("""
console.log("Hello from JavaScript!");
const x = 5 + 3;
x * 2;
""")

print(result)  # "Hello from JavaScript!\n16"
```

### Async API

For async applications, use the async versions:

```python
import asyncio
from capsule_adapter import run_python, run_javascript

async def main():
    result = await run_python("print('Hello!')")
    print(result)

asyncio.run(main())
```

### Preload Sandboxes (Optional)

The first execution of a sandbox has a cold start (~1 second). You can preload sandboxes to warm them up for faster subsequent executions (~10ms):

```python
from capsule_adapter import load_sandboxes_sync, run_python_sync

# Preload both sandboxes in parallel
load_sandboxes_sync()

# Now executions will be faster
result = run_python_sync('print("Fast!")')
```

Or preload individually:

```python
from capsule_adapter import load_python_sandbox_sync, load_javascript_sandbox_sync

load_python_sandbox_sync()      # Warm up Python only
load_javascript_sandbox_sync()  # Warm up JavaScript only
```

For async applications:

```python
import asyncio
from capsule_adapter import load_sandboxes, run_python

async def main():
    # Preload sandboxes
    await load_sandboxes()

    # Fast execution
    result = await run_python('print("Fast!")')
    print(result)

asyncio.run(main())
```

## How It Works

The adapter compiles Python and JavaScript sandboxes into WebAssembly modules during the build step. When you call `run_python()` or `run_javascript()`, the adapter invokes these pre-built sandboxes using Capsule's runner with the code you provide.

Learn more about [Capsule](https://github.com/mavdol/capsule).

## License

Apache-2.0
