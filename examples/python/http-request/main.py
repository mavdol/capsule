"""
HTTP Request Example

This example demonstrates making HTTP requests from a Capsule task.
Uses a public API (JSONPlaceholder) that doesn't require authentication.
"""

import json
import urllib.request
from capsule import task


@task(name="fetch_posts", compute="MEDIUM", allowed_hosts=["jsonplaceholder.typicode.com"])
def fetch_posts():
    """Fetch posts from JSONPlaceholder API."""
    url = "https://jsonplaceholder.typicode.com/posts/1"
    
    with urllib.request.urlopen(url) as response:
        if 200 <= response.status < 300:
            data = json.loads(response.read().decode('utf-8'))
            print(f"✅ Fetched post: {data['title'][:50]}...")
            return data
        else:
            print(f"❌ Request failed with status: {response.status}")
            return {"error": response.status}


@task(name="create_post", compute="MEDIUM", allowed_hosts=["jsonplaceholder.typicode.com"])
def create_post():
    """Create a new post via POST request."""
    url = "https://jsonplaceholder.typicode.com/posts"
    post_data = {
        "title": "Hello from Capsule!",
        "body": "This post was created from a Wasm task.",
        "userId": 1
    }
    
    req = urllib.request.Request(
        url, 
        data=json.dumps(post_data).encode('utf-8'),
        headers={'Content-Type': 'application/json'},
        method='POST'
    )

    with urllib.request.urlopen(req) as response:
        if 200 <= response.status < 300:
            data = json.loads(response.read().decode('utf-8'))
            print(f"✅ Created post with id: {data['id']}")
            return data
        else:
            print(f"❌ Request failed with status: {response.status}")
            return {"error": response.status}

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
