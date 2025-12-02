from typing import Literal
from pydantic import BaseModel

FeedbackSource = Literal["twitter", "zendesk", "appstore"]

class FeedbackMessage(BaseModel):
    """Raw feedback from a customer channel."""
    id: str
    source: FeedbackSource
    content: str
    metadata: dict = {}

class AnalysisResult(BaseModel):
    """Analysis output for a single feedback message."""
    message_id: str
    source: FeedbackSource
    sentiment: Literal["positive", "negative", "neutral"]
    category: str
    urgency: Literal["low", "medium", "high", "critical"]
    summary: str
