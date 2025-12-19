# Mini-Guardian

Un outil en ligne de commande ecrit en Rust pour scanner vos depots GitHub a la recherche de secrets exposes (cles API, tokens, mots de passe, etc.).

---

## Table des matieres

1. [Presentation](#presentation)
2. [Fonctionnalites](#fonctionnalites)
3. [Prerequis](#prerequis)
4. [Installation](#installation)
5. [Configuration](#configuration)
6. [Utilisation](#utilisation)
7. [Secrets detectes](#secrets-detectes)
8. [Structure du projet](#structure-du-projet)
9. [Contribution](#contribution)
10. [Licence](#licence)

---

## Presentation

Mini-Guardian est un scanner de secrets leger concu pour analyser vos depots GitHub. Il detecte automatiquement les informations sensibles qui pourraient etre exposees accidentellement dans votre code source, comme des cles d'API, des tokens d'authentification ou des mots de passe.

---

## Fonctionnalites

- Lister tous vos depots GitHub (publics et prives)
- Scanner un depot specifique sur toutes ses branches
- Scanner tous vos depots en une seule commande
- Filtrer les scans aux depots prives uniquement
- Sortie en format JSON pour integration avec d'autres outils
- Detection de plus de 20 types de secrets differents
- Masquage automatique des secrets detectes dans les rapports

---

## Prerequis

Avant de commencer, assurez-vous d'avoir :

- **Rust** installe (version 1.70 ou superieure recommandee)
  - Installation : https://rustup.rs/
- **Un token GitHub** avec les permissions de lecture sur vos depots
  - Creez-en un ici : https://github.com/settings/tokens
  - Permissions requises : `repo` (acces complet aux depots prives)

---

## Installation

### Cloner le projet

```bash
git clone https://github.com/votre-utilisateur/mini-guardian.git
cd mini-guardian
```

### Compiler le projet

```bash
cargo build --release
```

L'executable se trouve dans `target/release/mini_guardian`.

### Installation globale (optionnel)

```bash
cargo install --path .
```

---

## Configuration

### Variable d'environnement

Creez un fichier `.env` a la racine du projet :

```bash
GITHUB_TOKEN=ghp_votre_token_github_ici
```

Ou exportez la variable dans votre terminal :

```bash
export GITHUB_TOKEN=ghp_votre_token_github_ici
```

### Verifier la configuration

Listez vos depots pour verifier que le token fonctionne :

```bash
cargo run -- repos
```

---

## Utilisation

### Commandes disponibles

| Commande | Description |
|----------|-------------|
| `repos` | Liste tous vos depots GitHub |
| `scan <depot>` | Scanne un depot specifique |
| `scan-all` | Scanne tous vos depots |
| `patterns` | Affiche les patterns de secrets detectes |

### Exemples

**Lister vos depots :**

```bash
cargo run -- repos
```

**Scanner un depot specifique :**

```bash
# Par nom complet (owner/repo)
cargo run -- scan utilisateur/mon-projet

# Par nom simple (si c'est votre depot)
cargo run -- scan mon-projet
```

**Scanner tous vos depots :**

```bash
cargo run -- scan-all
```

**Scanner uniquement les depots prives :**

```bash
cargo run -- scan-all --private-only
```

**Obtenir les resultats en JSON :**

```bash
cargo run -- scan mon-projet --json
cargo run -- scan-all --json
```

**Voir les patterns de detection :**

```bash
cargo run -- patterns
```

---

## Secrets detectes

Mini-Guardian detecte les types de secrets suivants :

| Type | Description |
|------|-------------|
| AWS Access Key ID | Cles d'acces Amazon Web Services |
| AWS Secret Key | Cles secretes AWS |
| GitHub Token | Tokens d'acces personnel GitHub |
| GitHub OAuth | Tokens OAuth GitHub |
| Private Key | Fichiers de cles privees (RSA, EC, DSA, etc.) |
| Generic API Key | Cles API generiques |
| JWT Token | JSON Web Tokens |
| Slack Token | Tokens API Slack |
| Slack Webhook | URLs de webhooks Slack |
| Google API Key | Cles API Google |
| Stripe Secret Key | Cles secretes Stripe |
| Stripe Publishable Key | Cles publiques Stripe |
| Discord Token | Tokens de bots Discord |
| Password in URL | Mots de passe dans les URLs |
| Generic Password | Mots de passe dans le code |
| Heroku API Key | Cles API Heroku |
| SendGrid API Key | Cles API SendGrid |
| Twilio API Key | Cles API Twilio |
| npm Token | Tokens d'acces npm |
| Vite Token | Tokens API Vite |
| Supabase Keys | Cles Supabase (anon et service) |

---

## Structure du projet

```
mini-guardian/
├── Cargo.toml          # Configuration du projet et dependances
├── Cargo.lock          # Verrou des versions de dependances
├── regex.json          # Patterns de detection des secrets (modifiable !)
├── .env                # Token GitHub (a creer, non versionne)
├── .gitignore          # Fichiers ignores par Git
├── README.md           # Ce fichier
└── src/
    ├── main.rs         # Point d'entree et logique CLI
    ├── github.rs       # Client API GitHub
    ├── scanner.rs      # Moteur de scan
    ├── patterns.rs     # Loader des patterns depuis regex.json
    └── reporter.rs     # Formatage et affichage des resultats
```

### Description des modules

- **main.rs** : Gere les arguments de ligne de commande avec Clap et orchestre les differentes commandes.
- **github.rs** : Encapsule les appels a l'API GitHub via Octocrab (listing depots, branches, fichiers, contenu).
- **scanner.rs** : Applique les expressions regulieres sur le contenu des fichiers pour detecter les secrets.
- **patterns.rs** : Charge les patterns de secrets depuis le fichier `regex.json`.
- **reporter.rs** : Formate et affiche les resultats du scan (texte colore ou JSON).
- **regex.json** : Fichier JSON contenant tous les patterns de detection. Facile a modifier !

---

## Contribution

### Comment contribuer

1. **Forkez** le projet
2. **Creez** une branche pour votre fonctionnalite
   ```bash
   git checkout -b feature/ma-fonctionnalite
   ```
3. **Commitez** vos modifications
   ```bash
   git commit -m "Ajout de ma fonctionnalite"
   ```
4. **Poussez** votre branche
   ```bash
   git push origin feature/ma-fonctionnalite
   ```
5. **Ouvrez** une Pull Request

### Ajouter un nouveau pattern de secret

Pour ajouter un nouveau type de secret a detecter, c'est tres simple !

1. Ouvrez le fichier `regex.json` a la racine du projet
2. Ajoutez un nouvel objet dans le tableau `patterns` :

```json
{
    "name": "Mon Service API Key",
    "regex": "myservice_[a-zA-Z0-9]{32}",
    "description": "Cle API pour Mon Service"
}
```

3. Testez votre pattern :
   ```bash
   cargo run -- patterns
   cargo run -- scan votre-depot-de-test
   ```

**Note** : Les patterns sont charges dynamiquement depuis `regex.json`, donc pas besoin de recompiler le projet !

### Standards de code

- Formatez le code avec `cargo fmt`
- Verifiez les erreurs avec `cargo clippy`
- Assurez-vous que le projet compile sans erreurs avec `cargo build`

### Types de fichiers scannes

Le scanner analyse automatiquement les fichiers avec ces extensions :
- Code : `.rs`, `.py`, `.js`, `.ts`, `.jsx`, `.tsx`, `.go`, `.java`, `.rb`, `.php`, `.cs`, `.cpp`, `.c`, `.h`, `.hpp`, `.swift`, `.kt`, `.scala`
- Scripts : `.sh`, `.bash`
- Configuration : `.env`, `.yml`, `.yaml`, `.json`, `.toml`, `.xml`, `.ini`, `.cfg`, `.conf`, `.properties`
- Infrastructure : `.dockerfile`, `.tf`, `.tfvars`
- Documentation : `.md`, `.txt`, `.sql`

### Repertoires ignores

Ces repertoires sont automatiquement ignores lors du scan :
- `node_modules`, `.git`, `vendor`, `target`, `dist`, `build`
- `__pycache__`, `.venv`, `venv`, `.idea`, `.vscode`
- `coverage`, `.next`, `.nuxt`, `out`, `bin`, `obj`, `packages`

---

## Dependances

| Crate | Version | Utilisation |
|-------|---------|-------------|
| clap | 4.5 | Gestion des arguments CLI |
| octocrab | 0.48 | Client API GitHub |
| regex | 1.12 | Expressions regulieres |
| tokio | 1.48 | Runtime asynchrone |
| serde / serde_json | 1.0 | Serialisation JSON |
| colored | 3.0 | Couleurs dans le terminal |
| dotenv | 0.15 | Lecture du fichier .env |
| base64 | 0.22 | Decodage du contenu des fichiers |

---

## Licence

Ce projet est distribue sous licence MIT. Voir le fichier LICENSE pour plus de details.

---

## Support

Si vous rencontrez un probleme ou avez une question :

1. Consultez les issues existantes sur GitHub
2. Ouvrez une nouvelle issue en decrivant votre probleme
3. Fournissez les informations suivantes :
   - Version de Rust (`rustc --version`)
   - Systeme d'exploitation
   - Message d'erreur complet
   - Etapes pour reproduire le probleme
