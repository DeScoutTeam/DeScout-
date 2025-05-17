import json
from llama_cpp import Llama
from src.ai.prompt_builder import build_prompt

llm = Llama(model_path="D:/Models/mistral.Q4_K_M.gguf")


def query_llm(tweet_text: str) -> dict:
    prompt = build_prompt(tweet_text)

    output = llm(
        prompt,
        max_tokens=512,
        stop=["}"],
        temperature=0.1,
        echo=False
    )

    full_output = output["choices"][0]["text"].strip() + "}"

    print(full_output)
    try:
        return json.loads(full_output)
    except Exception as e:
        print("⚠️ LLM returned malformed JSON:", full_output)
        return {
            "deepness": 1,
            "keywords": [],
            "tokenName": "",
            "mentionedUsers": [],
            "links": []
        }
