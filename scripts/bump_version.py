import datetime
import os
import re
import subprocess


def get_commit_count():
    try:
        result = subprocess.run(
            ["git", "rev-list", "--count", "HEAD"],
            capture_output=True,
            text=True,
            check=True,
        )
        return int(result.stdout.strip())
    except Exception:
        return 0


def update_file(path, pattern, replacement, count=0):
    if not os.path.exists(path):
        return False
    with open(path, "r") as f:
        content = f.read()
    new_content = re.sub(pattern, replacement, content, count=count, flags=re.MULTILINE)
    if content != new_content:
        with open(path, "w") as f:
            f.write(new_content)
        return True
    return False


def main():
    now = datetime.datetime.now()
    commit_count = get_commit_count()
    # Next commit will be commit_count + 1 if we are about to commit.
    # But often this is run before committing, so we use commit_count + 1.
    n = commit_count + 1

    # YY.M.D.N format as per user preference (or YY.MM.DD.N)
    version_cargo = f"{now.year % 100}.{now.month}.{now.day}"
    version_short = f"{now.year % 100}.{now.month}.{now.day}.{n}"
    version_long = f"{now.year % 100}.{now.month:02}.{now.day:02}.{n}"

    root_dir = os.path.join(os.path.dirname(__file__), "..")
    games_dir = os.path.join(root_dir, "games_repo", "games")

    changed_files = []

    # Update Jetpac
    jetpac_cargo = os.path.join(games_dir, "jetpac", "Cargo.toml")
    if update_file(jetpac_cargo, r'^version = ".*"', f'version = "{version_cargo}"'):
        changed_files.append(jetpac_cargo)

    # Update Lumines
    lumines_cargo = os.path.join(games_dir, "lumines", "Cargo.toml")
    if update_file(lumines_cargo, r'^version = ".*"', f'version = "{version_cargo}"'):
        changed_files.append(lumines_cargo)
    lumines_main = os.path.join(games_dir, "lumines", "src", "main.rs")
    if update_file(
        lumines_main,
        r'const VERSION: &str = ".*"',
        f'const VERSION: &str = "{version_long}"',
    ):
        changed_files.append(lumines_main)

    # Update Zookeeper main.rs version
    zookeeper_main = os.path.join(games_dir, "zookeeper", "src", "main.rs")
    if update_file(
        zookeeper_main,
        r'const VERSION: &str = ".*"',
        f'const VERSION: &str = "{version_long}"',
    ):
        changed_files.append(zookeeper_main)

    # Update Cargo.toml for other games
    for game in ["zookeeper", "bubbles", "music_editor"]:
        cargo_path = os.path.join(games_dir, game, "Cargo.toml")
        if update_file(cargo_path, r'^version = ".*"', f'version = "{version_cargo}"'):
            changed_files.append(cargo_path)

    # Update Music Editor Makefile
    music_editor_makefile = os.path.join(games_dir, "music_editor", "Makefile")
    if update_file(music_editor_makefile, r"^VERSION=.*", f"VERSION={version_short}"):
        changed_files.append(music_editor_makefile)

    # Update CHANGELOG.md (only the first header found)
    changelog = os.path.join(root_dir, "games_repo", "CHANGELOG.md")
    if update_file(changelog, r"## \[.*\] - ", f"## [{version_short}] - ", count=1):
        changed_files.append(changelog)

    for path in changed_files:
        subprocess.run(["git", "add", path], check=True)
        print(
            f"Bumped and staged {os.path.relpath(path, root_dir)} to {version_short} or equivalent"
        )


if __name__ == "__main__":
    main()
