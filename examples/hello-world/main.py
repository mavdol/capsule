from capsule import task


@task(name="First_task", compute="MEDIUM", ram="128MB")
def weird() -> str:
    """Simple main task that just returns a message."""
    return "Pre task like! 1"

@task(name="de", compute="MEDIUM", ram="128MB")
def main() -> str:
    """Simple main task that just returns a message."""
    weirdo = weird()
    print(weirdo)
    return "Hello from caps! 🚀"


