### HOW TO LAUNCH

## Express.js part

# 1. npm install
# 2. npm run dev
# 3. to fetch posts go to: [localhost:4000/api/](http://localhost:4000/api/airdrop-alerts/fetch)

## Python + AI part

# 1. python -m venv venv | python3 -m venv venv | py -m venv venv
# 2. .\venv\Scripts\activate
# 3. pip install -r src/ai/requirements.txt | python -m pip install -r src/ai/requirements.txt | py -m pip install -r src/ai/requirements.txt | python3 -m pip install -r src/ai/requirements.txt
# 4. Go to src/ai/llama_client.py and specify your model path
# 5. python -m src.ai.main | py -m src.ai.main | python3 -m src.ai.main

## Rust Profile Scraper
# 1. Go to rust_services/deepness_level_2
# 2. cargo run --release

### Commits

## 1. Express Part, make sure, to not inlcude src/ai and rust_services
## 2. Python Part, make sure, to not include rust_services
## 3. Rust part


### Commit messages

1. Integrate Twitter/X api, for fetching posts, storing teh fetched posts in two redis databases, one will be used to create a queue for AI processing, teh second is used, to store already feched posts id, to prevent deduplication and have a quick access to it

2. Integratted Llama model to analyze tweets, extract keywords, users' metions and specific token mentions.
Implemented a batch and multi-thread data processing, while adding the AI analyze data to MongoDb, with deepness level 1, and clearing redis queue

3. Started to implement a level 2 deepnes, which will exemine user profile and fetch and proces sal of his posts, his bio. Made wit rust
 
