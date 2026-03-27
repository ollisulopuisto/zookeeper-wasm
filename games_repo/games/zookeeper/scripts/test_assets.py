from download_assets import EMOJIS


def test_emoji_count():
    assert len(EMOJIS) == 25


def test_emoji_codes():
    for name, code in EMOJIS.items():
        assert len(code) >= 4
        assert code.isalnum()
