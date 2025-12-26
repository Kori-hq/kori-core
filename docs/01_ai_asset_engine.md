# AI Asset Generation Engine

Goal: Turn unstructured chat into a structured, searchable business page at the point of creation.

Flow:
- Trigger check: message contains '#'
- Entity extraction: parse the hashtag token until space/end
- API call: send extracted string as query to Places Text Search
- Confirmation: user clicks Yes/No on suggested place
- Creation: create a business page; allow later edits/corrections

This matches the deck's "Simple Regex → Places API → Confirmation → Creation" pipeline.
