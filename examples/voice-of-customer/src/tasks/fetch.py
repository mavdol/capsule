import os
from capsule import task
from faker import Faker
from src.models.feedback import FeedbackMessage

fake = Faker()

@task(
    name="fetch_twitter",
    compute="LOW",
    ram="128MB",
    timeout="10s"
)
def fetch_twitter_messages(limit: int = 3000) -> list[dict]:
    """
    Simulates fetching messages from Twitter/X API.
    In production, this would call the real Twitter API.
    """
    messages = []
    for i in range(limit):
        messages.append(
            FeedbackMessage(
                id=f"tw_{i}",
                source="twitter",
                content=fake.sentence(nb_words=10),  # Short, tweet-like
                metadata={"likes": fake.random_int(0, 1000)}
            ).model_dump()
        )
    return messages

@task(
    name="fetch_zendesk",
    compute="LOW",
    ram="256MB",
    timeout="15s"
)
def fetch_zendesk_tickets(limit: int = 6000) -> list[dict]:
    """
    Simulates fetching support tickets from Zendesk API.
    In production, this would call the real Zendesk API.
    """
    messages = []
    for i in range(limit):
        # Zendesk tickets are longer and more technical
        content = " ".join([fake.paragraph() for _ in range(3)])
        messages.append(
            FeedbackMessage(
                id=f"zd_{i}",
                source="zendesk",
                content=content,
                metadata={"ticket_priority": fake.random_element(["low", "medium", "high"])}
            ).model_dump()
        )
    return messages

@task(
    name="fetch_appstore",
    compute="LOW",
    ram="128MB",
    timeout="10s"
)
def fetch_appstore_reviews(limit: int = 1000) -> list[dict]:
    """
    Simulates fetching reviews from App Store API.
    In production, this would call the real App Store Connect API.
    """
    messages = []
    for i in range(limit):
        messages.append(
            FeedbackMessage(
                id=f"as_{i}",
                source="appstore",
                content=fake.paragraph(),
                metadata={"rating": fake.random_int(1, 5)}
            ).model_dump()
        )
    return messages
