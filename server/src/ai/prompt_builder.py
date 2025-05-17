def build_prompt(tweet_text: str) -> str:
    return f"""
You are an AI assistant that analyzes tweets to determine if they are related to a crypto airdrop.

Your task is to:
1. Determine whether the tweet is about an airdrop.
2. Extract:
    - a list of relevant keywords,
    - token name (if any), like $SOL, $TOKENNAME,
    - mentioned users (like @someone),
    - and all links in the tweet.
3. Output ONLY a strict JSON object, nothing else.

If the tweet is not about an airdrop, return:
{{ "deepness": 1, "keywords": [], "tokenName": "", "mentionedUsers": [], "links": [] }}
Notice: deepness should always be one
Tweet to analyze:
{tweet_text}

Your response:
"""
