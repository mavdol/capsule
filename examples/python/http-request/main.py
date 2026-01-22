"""
HTTP Request Example

This example demonstrates making HTTP requests from a Capsule task.
Uses a public API (JSONPlaceholder) that doesn't require authentication.
"""

from capsule import task
from capsule.http import get, post


@task(name="fetch_posts", compute="MEDIUM")
def fetch_posts():
    """Fetch posts from JSONPlaceholder API."""
    response = get("https://jsonplaceholder.typicode.com/posts/1")

    if response.ok():
        data = response.json()
        print(f"✅ Fetched post: {data['title'][:50]}...")
        return data
    else:
        print(f"❌ Request failed with status: {response.status_code}")
        return {"error": response.status_code}


@task(name="create_post", compute="MEDIUM")
def create_post():
    """Create a new post via POST request."""
    response = post(
        "https://jsonplaceholder.typicode.com/posts",
        json_data={
            "title": "Hello from Capsule!",
            "body": "This post was created from a Wasm task.",
            "userId": 1
        }
    )

    if response.ok():
        data = response.json()
        print(f"✅ Created post with id: {data['id']}")
        return data
    else:
        print(f"❌ Request failed with status: {response.status_code}")
        return {"error": response.status_code}

@task(name="main")
def main():
    """Run HTTP examples."""
    print("🌐 HTTP Request Example")
    print("=" * 40)
    print()

    print("📥 Fetching a post...")
    post_data = fetch_posts()
    print(f"   Title: {post_data.get('title', 'N/A')}")
    print()

    print("📤 Creating a new post...")
    new_post = create_post()
    print(f"   New post ID: {new_post.get('id', 'N/A')}")
    print()

    print("✨ Done!")
    return {"fetched": post_data, "created": new_post}
