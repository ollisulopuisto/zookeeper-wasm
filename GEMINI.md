# Monorepo Structure

This project is a monorepo for WASM-based games written in Rust.

## Workspaces

Managed by root `Cargo.toml`:
- `games_repo/games/jetpac/`: A WASM implementation of Jetpac.
- `games_repo/games/zookeeper/`: A match-3 style game.

## Key Files
- `games_repo/index.html`: The main games portal.
- `.github/workflows/deploy.yml`: Root deployment workflow for all games and the portal.
- `scripts/`: Project-wide utility scripts.

## Tech Stack
- **Language**: Rust
- **Target**: WebAssembly (WASM)
- **Deployment**: GitHub Pages (automatically via GH Actions)


---

Workflow: PR-Based Development & Safety

Tämä projekti noudattaa tiukkaa PR-pohjaista (Pull Request) kehitysmallia vikasietoisuuden ja koodin laadun varmistamiseksi, vaikka kehittäjiä on vain yksi.

1. Haaroitus (Branching)
 - Kielto: Älä koskaan tee muutoksia tai committeja suoraan main-haaraan.
 - Käytäntö: Luo jokaiselle tehtävälle (feature, bug fix, refactor) uusi haara main-haarasta käsin.
 - Nimeäminen: Käytä etuliitteitä: feat/lyhyt-kuvaus, fix/bugin-nimi tai refactor/kohde.

2. Kehitys ja Validointi (Execution & Validation)
 - Nouda standardia Research -> Strategy -> Execution -sykliä.
 - Ennen commitointia, aja aina projektikohtaiset testit, linterit ja tyyppitarkistukset lokaalisti (uv run ruff check ., npm test, jne.).
 - Varmista, että kaikki muutokset ovat idiomaattisia ja noudattavat projektin tyyliä.

3. Commit ja Push
 - Tee atomisia ja selkeitä committeja, jotka kuvaavat muutoksen tarkoitusta.
 - Puskemisen jälkeen (push), agentin tulee raportoida onnistunut puskeminen ja antaa ohje PR:n avaamiseen (tai avata se, jos työkalut sallivat).

4. Pull Request (PR) ja Itsekatselmointi
 - PR-vaihe on kriittinen "viimeinen tarkistus" ennen koodin päätymistä staging-jonoon.
 - Status Checkit: CI:n (GitHub Actions) on mentävä läpi PR-haarassa ennen mergeä.
 - Squash Merge: Suosi "Squash and merge" -toimintoa, jotta main-haaran historia pysyy siistinä ja jokainen PR näkyy yhtenä kokonaisuutena.

5. Vikasietoisuuden tavoite
 - Main-eheys: main-haaran on oltava aina julkaisukelpoinen. Jos CI epäonnistuu PR-haarassa, main ei saastu.
 - Rollback-valmius: Koska käytössä on blue/green-julkaisu, jokaisen mergetyn PR:n on oltava helposti peruttavissa (git revert) ilman sivuvaikutuksia muihin ominaisuuksiin.
