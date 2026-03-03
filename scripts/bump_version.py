import datetime
import os
import re

def main():
    now = datetime.datetime.now()
    # CalVer (SemVer-compatible): YY.M.DHHMM
    version = f"{now.year % 100}.{now.month}.{now.day}{now.hour:02}{now.minute:02}"
    
    cargo_path = os.path.join(os.path.dirname(__file__), "..", "games_repo", "games", "jetpac_wasm", "Cargo.toml")
    
    with open(cargo_path, "r") as f:
        content = f.read()
    
    new_content = re.sub(r'^version = ".*"', f'version = "{version}"', content, flags=re.MULTILINE)
    
    with open(cargo_path, "w") as f:
        f.write(new_content)
    
    import subprocess
    subprocess.run(["git", "add", cargo_path], check=True)
    print(f"Bumped and staged version {version}")

if __name__ == "__main__":
    main()
