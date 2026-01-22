import { task } from "@capsule-run/sdk";

interface Post {
  id: number;
  title: string;
  body: string;
  userId: number;
}

/**
 * Fetch posts from JSONPlaceholder API.
 */
export const fetchPosts = task(
  {
    name: "fetchPosts",
    compute: "MEDIUM"
  },
  async (): Promise<Post | { error: number }> => {
    const response = await fetch("https://jsonplaceholder.typicode.com/posts/1");

    if (response.ok) {
      const data: Post = await response.json();
      console.log(`✅ Fetched post: ${data.title.slice(0, 50)}...`);
      return data;
    } else {
      console.log(`❌ Request failed with status: ${response.status}`);
      return { error: response.status };
    }
  }
);

/**
 * Create a new post via POST request.
 */
export const createPost = task(
  {
    name: "createPost",
    compute: "MEDIUM"
  },
  async (): Promise<Post | { error: number }> => {
    const response = await fetch("https://jsonplaceholder.typicode.com/posts", {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify({
        title: "Hello from Capsule!",
        body: "This post was created from a Wasm task.",
        userId: 1
      })
    });

    if (response.ok) {
      const data: Post = await response.json();
      console.log(`✅ Created post with id: ${data.id}`);
      return data;
    } else {
      console.log(`❌ Request failed with status: ${response.status}`);
      return { error: response.status };
    }
  }
);

/**
 * Main function to run HTTP examples.
 */
export const main = task(
  {
    name: "main",
    compute: "MEDIUM"
  },
  async (): Promise<{ fetched: Post | { error: number }; created: Post | { error: number } }> => {
    console.log("🌐 HTTP Request Example");
    console.log("========================================");
    console.log();

    console.log("📥 Fetching a post...");
    const postData = await fetchPosts();
    if ("title" in postData) {
      console.log(`   Title: ${postData.title}`);
    }
    console.log();

    console.log("📤 Creating a new post...");
    const newPost = await createPost();
    if ("id" in newPost) {
      console.log(`   New post ID: ${newPost.id}`);
    }
    console.log();

    console.log("✨ Done!");
    return { fetched: postData, created: newPost };
  }
);
