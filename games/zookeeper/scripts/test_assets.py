from download_assets import EMOJIS

def test_emoji_count():
    assert len(EMOJIS) == 7

def test_emoji_codes():
    for name, code in EMOJIS.items():
        assert len(code) >= 5
        assert code.isalnum()
