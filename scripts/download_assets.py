import requests
import os

EMOJIS = {
    "monkey": "1f435",
    "penguin": "1f427",
    "tiger": "1f42f",
    "elephant": "1f418",
    "giraffe": "1f992",
    "panda": "1f43c",
    "frog": "1f438"
}

# Public domain sounds from Wikimedia Commons or similar open sources
SOUNDS = {
    "swap": "https://upload.wikimedia.org/wikipedia/commons/0/05/Beep-07.wav",
    "match": "https://upload.wikimedia.org/wikipedia/commons/4/4c/Beep-09.wav",
    "fall": "https://upload.wikimedia.org/wikipedia/commons/2/21/Beep-02.wav",
    "game_over": "https://upload.wikimedia.org/wikipedia/commons/e/e5/Beep-04.wav"
}

def download_assets():
    os.makedirs("../assets", exist_ok=True)
    for name, code in EMOJIS.items():
        url = f"https://abs.twimg.com/emoji/v2/72x72/{code}.png"
        path = f"../assets/{code}.png"
        if not os.path.exists(path):
            print(f"Downloading emoji: {name}...")
            r = requests.get(url)
            with open(path, "wb") as f:
                f.write(r.content)
    
    for name, url in SOUNDS.items():
        path = f"../assets/{name}.wav"
        if not os.path.exists(path):
            print(f"Downloading sound: {name}...")
            r = requests.get(url)
            with open(path, "wb") as f:
                f.write(r.content)

if __name__ == "__main__":
    download_assets()
