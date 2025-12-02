import asyncio
from src.tasks.fetch import (
    fetch_twitter_messages,
    fetch_zendesk_tickets,
    fetch_appstore_reviews
)
from src.tasks.analyze import analyze_feedback

async def main():
    print("🚀 Voice of Customer Engine - Starting...")
    print()

    print("📥 Fetching feedback from multiple channels...")

    twitter_messages = await fetch_twitter_messages(limit=3000)
    zendesk_tickets = await fetch_zendesk_tickets(limit=6000)
    appstore_reviews = await fetch_appstore_reviews(limit=1000)

    print(f"✓ Fetched {len(twitter_messages)} Twitter messages")
    print(f"✓ Fetched {len(zendesk_tickets)} Zendesk tickets")
    print(f"✓ Fetched {len(appstore_reviews)} App Store reviews")
    print()

    all_feedback = twitter_messages + zendesk_tickets + appstore_reviews
    print(f"📊 Total feedback messages: {len(all_feedback)}")
    print()

    print("🔍 Analyzing feedback (this runs in parallel)...")
    print("💡 Tip: Run 'capsule list' in another terminal to watch progress")
    print()

    tasks = [analyze_feedback(msg) for msg in all_feedback]
    results = await asyncio.gather(*tasks, return_exceptions=True)

    successful = [r for r in results if not isinstance(r, Exception)]
    failed = [r for r in results if isinstance(r, Exception)]

    print(f"✅ Completed: {len(successful)}/{len(all_feedback)}")
    if failed:
        print(f"❌ Failed: {len(failed)} (malformed input or timeout)")
    print()

    print("📈 Top Insights:")

    critical_issues = [r for r in successful if r["urgency"] == "critical"]
    bugs = [r for r in successful if r["category"] == "bug"]

    print(f"  • Critical issues found: {len(critical_issues)}")
    print(f"  • Bugs reported: {len(bugs)}")
    print()

    by_source = {}
    for result in successful:
        source = result["source"]
        by_source.setdefault(source, []).append(result)

    print("📊 Results by source:")
    for source, items in by_source.items():
        print(f"  • {source}: {len(items)} analyzed")
    print()

    print("✨ Pipeline complete! Check 'traces.jsonl' for detailed metrics.")
    print("💡 Tip: Run 'capsule list' to see resource consumption per task")

if __name__ == "__main__":
    asyncio.run(main())
