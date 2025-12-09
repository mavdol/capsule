from capsule import task

@task(
    name="fetch_twitter",
    compute="HIGH",
    ram="128MB",
    timeout="10s"
)
def fetch_twitter_messages(limit: int = 3000) -> list:
    """
    Simulates fetching messages from Twitter/X API.
    In production, this would call the real Twitter API.
    """
    messages = []
    print(f"📥 Fetching {limit} Twitter messages...")
    for i in range(limit):
        messages.append({
            "id": f"tw_{i}",
            "source": "twitter",
            "content": f"Sample tweet message {i} about product feedback",
            "metadata": {"likes": i * 10}
        })
    return messages

@task(
    name="fetch_zendesk",
    compute="HIGH",
    ram="256MB",
    timeout="15s"
)
def fetch_zendesk_tickets(limit: int = 6000) -> list:
    """
    Simulates fetching support tickets from Zendesk API.
    In production, this would call the real Zendesk API.
    """
    messages = []
    priorities = ["low", "medium", "high"]
    for i in range(limit):
        messages.append({
            "id": f"zd_{i}",
            "source": "zendesk",
            "content": f"Support ticket {i}: Customer reported issue with feature X. Details: Problem description goes here.",
            "metadata": {"ticket_priority": priorities[i % 3]}
        })
    return messages

@task(
    name="fetch_appstore",
    compute="HIGH",
    ram="128MB",
    timeout="10s"
)
def fetch_appstore_reviews(limit: int = 1000) -> list:
    """
    Simulates fetching reviews from App Store API.
    In production, this would call the real App Store Connect API.
    """
    messages = []
    for i in range(limit):
        messages.append({
            "id": f"as_{i}",
            "source": "appstore",
            "content": f"App Store review {i}: User feedback about the application experience.",
            "metadata": {"rating": (i % 5) + 1}
        })
    return messages
