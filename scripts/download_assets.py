import requests
import os

EMOJIS = {
    "monkey": "1f435",
    "lion": "1f981",
    "tiger": "1f42f",
    "elephant": "1f418",
    "giraffe": "1f992",
    "panda": "1f43c",
    "frog": "1f438"
}

def download_assets():
    os.makedirs("../assets", exist_ok=True)
    for name, code in EMOJIS.items():
        url = f"https://abs.twimg.com/emoji/v2/72x72/{code}.png"
        path = f"../assets/{code}.png"
        if not os.path.exists(path):
            print(f"Downloading {name}...")
            r = requests.get(url)
            with open(path, "wb") as f:
                f.write(r.content)

if __name__ == "__main__":
    download_assets()
