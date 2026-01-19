from capsule import task


@task(name="First_task", compute="MEDIUM", ram="128MB")
def hello() -> str:
    """Simple main task that just returns a message."""
    return "Pre task like! 1"

@task(name="de", compute="MEDIUM", ram="128MB")
def main() -> str:
    """Simple main task that just returns a message."""
    hellomessage = hello()
    print(hellomessage)
    return "Hello from caps! 🚀"


