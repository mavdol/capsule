import asyncio
from capsule_adapter import run_javascript

async def main():
    print("Executing JavaScript code inside Capsule sandbox...")

    result = await run_javascript("""
console.log("Hello from JavaScript!");
let x = 5 + 3;
x * 2;
""")

    print("\n--- Sandbox Output ---")
    print(result)

if __name__ == "__main__":
    asyncio.run(main())
