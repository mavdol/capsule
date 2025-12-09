from capsule import task

@task(
    name="analyze_feedback",
    compute="HIGH",
    ram="512MB",
    timeout="30s"
)
def analyze_feedback(message: dict) -> dict:
    source = message["source"]
    content = message["content"]

    sentiment = _simulate_sentiment_analysis(content)
    category = _simulate_categorization(content, source)
    urgency = _simulate_urgency_detection(content, category)
    summary = _simulate_summarization(content)

    return {
        "message_id": message["id"],
        "source": source,
        "sentiment": sentiment,
        "category": category,
        "urgency": urgency,
        "summary": summary
    }


def _simulate_sentiment_analysis(content: str) -> str:
    keywords_negative = ["bug", "crash", "broken", "awful", "hate"]
    keywords_positive = ["great", "love", "amazing", "excellent"]

    content_lower = content.lower()
    if any(word in content_lower for word in keywords_negative):
        return "negative"
    elif any(word in content_lower for word in keywords_positive):
        return "positive"
    return "neutral"

def _simulate_categorization(content: str, source: str) -> str:
    content_lower = content.lower()

    if "bug" in content_lower or "crash" in content_lower:
        return "bug"
    elif "feature" in content_lower or "add" in content_lower:
        return "feature_request"
    elif "slow" in content_lower or "performance" in content_lower:
        return "performance"
    else:
        return "general_feedback"

def _simulate_urgency_detection(content: str, category: str) -> str:
    content_lower = content.lower()

    if "critical" in content_lower or "urgent" in content_lower:
        return "critical"
    elif category == "bug" and "crash" in content_lower:
        return "high"
    elif category == "bug":
        return "medium"
    else:
        return "low"

def _simulate_summarization(content: str) -> str:
    words = content.split()[:15]
    return " ".join(words) + "..."
