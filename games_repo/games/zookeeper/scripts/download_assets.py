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
    "hippo": "1f99b",
    "lion": "1f981",
    "zebra": "1f993",
    "pig": "1f437",
    "koala": "1f428",
    "rabbit": "1f430",
    "cat": "1f431",
    "dog": "1f436",
    "mouse": "1f42d",
    "sheep": "1f411",
    "chick": "1f424",
    "fox": "1f98a",
    "cow": "1f404",
    "speaker_on": "1f50a",
    "speaker_off": "1f507",
    "pause": "23f8",
    "play": "25b6",
    "snail": "1f40c",
}


def download_assets():
    # Assets are in ../assets relative to scripts/
    os.makedirs("../assets", exist_ok=True)
    headers = {
        "User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
    }
    for name, code in EMOJIS.items():
        url = f"https://abs.twimg.com/emoji/v2/72x72/{code}.png"
        path = f"../assets/{code}.png"
        if not os.path.exists(path) or os.path.getsize(path) < 500:
            print(f"Downloading emoji: {name} ({code})...")
            r = requests.get(url, headers=headers)
            if r.status_code == 200:
                with open(path, "wb") as f:
                    f.write(r.content)
            else:
                print(f"Failed to download {name}: {r.status_code}")


if __name__ == "__main__":
    download_assets()
