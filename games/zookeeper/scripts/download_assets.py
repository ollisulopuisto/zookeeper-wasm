import requests
import os

EMOJIS = {
    "monkey": "1f435",
    "penguin": "1f427",
    "tiger": "1f42f",
    "elephant": "1f418",
    "giraffe": "1f992",
    "panda": "1f43c",
    "frog": "1f438",
    "speaker_on": "1f50a",
    "speaker_off": "1f507",
    "pause": "23f8",
    "play": "25b6"
}

def download_assets():
    os.makedirs("../assets", exist_ok=True)
    headers = {
        "User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
    }
    for name, code in EMOJIS.items():
        url = f"https://abs.twimg.com/emoji/v2/72x72/{code}.png"
        path = f"../assets/{code}.png"
        if not os.path.exists(path) or os.path.getsize(path) < 500:
            print(f"Downloading emoji: {name}...")
            r = requests.get(url, headers=headers)
            with open(path, "wb") as f:
                f.write(r.content)

if __name__ == "__main__":
    download_assets()
