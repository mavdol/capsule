"""
Voice of Customer - Example with Module Imports

This example demonstrates:
1. Task registration across multiple imported modules
2. Nested directory structure with tasks/
3. Each @task creates an isolated Wasm instance
"""

from capsule import task
from tasks.fetch import (
    fetch_twitter_messages,
    fetch_zendesk_tickets,
    fetch_appstore_reviews
)
from tasks.analyze import analyze_feedback

def main() -> dict:
    print("🚀 Voice of Customer Engine - Starting...")
    print()

    print("📥 Fetching feedback from multiple channels...")

    twitter_messages = fetch_twitter_messages(limit=50)
    zendesk_tickets = fetch_zendesk_tickets(limit=50)
    appstore_reviews = fetch_appstore_reviews(limit=50)

    print(f"✓ Fetched {len(twitter_messages)} Twitter messages")
    print(f"✓ Fetched {len(zendesk_tickets)} Zendesk tickets")
    print(f"✓ Fetched {len(appstore_reviews)} App Store reviews")
    print()

    all_feedback = twitter_messages + zendesk_tickets + appstore_reviews
    print(f"📊 Total feedback messages: {len(all_feedback)}")
    print()

    print("🔍 Analyzing feedback (each creates isolated Wasm instance)...")
    print()

    results = []
    for msg in all_feedback[:10]:
        result = analyze_feedback(msg)
        results.append(result)

    print(f"✅ Completed: {len(results)} analyses")
    print()

    print("📈 Top Insights:")

    critical_issues = [r for r in results if r["urgency"] == "critical"]
    bugs = [r for r in results if r["category"] == "bug"]

    print(f"  • Critical issues found: {len(critical_issues)}")
    print(f"  • Bugs reported: {len(bugs)}")
    print()

    by_source = {}
    for result in results:
        source = result["source"]
        by_source.setdefault(source, []).append(result)

    print("📊 Results by source:")
    for source, items in by_source.items():
        print(f"  • {source}: {len(items)} analyzed")
    print()

    return {
        "total_fetched": len(all_feedback),
        "total_analyzed": len(results),
        "critical_issues": len(critical_issues),
        "bugs": len(bugs),
        "by_source": {k: len(v) for k, v in by_source.items()}
    }
