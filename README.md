# Duel Game

**Duel Game** est un mini-jeu de duel développé en Rust, dans lequel deux joueurs (ou plus en mode multi‑joueurs) s'affrontent à tour de rôle sur des objectifs aléatoires. Chaque tour consiste à arrêter un compteur le plus près possible d'une valeur cible générée aléatoirement. Le score de chaque objectif est calculé en fonction de la précision, du nombre de dépassements (miss) et de la force du joueur. À la fin de chaque manche, le joueur avec le meilleur score gagne et peut infliger une pénalité (poison) au perdant.

Ce projet met en pratique de nombreux concepts avancés de Rust, tels que :

- **Modularité** : Découpage du code en modules (`player`, `objectives`, `turn`, `score`, `game`, `bonus_objectives`, `bonus_turn`, `score_bonus`) pour une meilleure lisibilité.
- **Ownership et Borrowing** : Gestion explicite des ressources et sécurité mémoire.
- **Smart Pointers et Concurrence** : Utilisation de `Arc`, `Mutex` et de threads pour la synchronisation.
- **Gestion des Erreurs** : Propagation des erreurs avec `Result` et l'opérateur `?`.
- **Itérateurs et Closures** : Manipulation de collections de manière idiomatique.
- **Bonus** :
  - **Bonus 1** : Objectifs bonus générés sous forme de paires (clé, valeur) où la clé est une lettre, avec vérification de la touche appuyée.
  - **Bonus 2** : Mode multi‑joueurs permettant de jouer avec plus de 2 joueurs via l’option `--players`.

---

## Prérequis

- [Rust](https://www.rust-lang.org/tools/install) (version stable recommandée)
- Cargo (inclus avec Rust)

---

## Installation

1. Clonez le dépôt :
   ```bash
   git clone <URL_DU_DEPOT>
   cd duel_game
   ```


2. Installez les dépendances avec Cargo :

```bash
cargo build
```
## Utilisation
Mode Classique (2 joueurs)
Pour lancer le jeu en mode 2 joueurs avec les valeurs par défaut, exécutez :

```bash
cargo run
```
Vous pouvez personnaliser les paramètres en passant les options suivantes :

--name1 : Nom du premier joueur (par défaut "Michel")

--name2 : Nom du deuxième joueur (par défaut "Jacque")

--vitality : Vitalité initiale (par défaut 50)

--speed : Vitesse (détermine le délai d'incrémentation en millisecondes, par défaut 50)

--strength : Force (par défaut 50)

--objectifs : Nombre d'objectifs par manche (par défaut 5)

Exemple :

```bash
cargo run -- --name1 Alice --name2 Bob --vitality 60 --speed 40 --strength 55 --objectifs 5
Mode Multi‑Joueurs
Pour jouer en mode multi‑joueurs, utilisez le flag --multi (sans valeur) et passez la liste des noms avec l'option --players (noms séparés par des virgules). Utilisez également le double tiret (--) pour transmettre ces arguments à votre programme :
```
```bash
cargo run -- --multi --players Alice,Bob,Charlie
```
## Architecture du Projet
Le projet est structuré de la manière suivante :

src/player.rs : Définition de la structure Player et de ses méthodes.

src/objectives.rs : Génération d'objectifs aléatoires (valeurs numériques).

src/turn.rs : Gestion d’un objectif, incluant le compteur et la détection d'appui sur ENTER, avec utilisation de threads et smart pointers.

src/score.rs : Calcul du score d’un objectif en fonction de la différence entre la cible et le compteur, du nombre de "miss" et de la force.

src/game.rs : Gestion du déroulement d'une manche : tour de chaque joueur, comparaison des scores, application des pénalités et choix du poison.

## Bonus :

src/bonus_objectives.rs : Génération d'objectifs bonus sous forme de paires (clé, valeur) où la clé est une lettre.

src/bonus_turn.rs : Variante de la gestion d’un objectif qui demande au joueur d'appuyer sur une touche spécifique et vérifie la correspondance.

src/score_bonus.rs : Calcul du score bonus qui annule le score en cas de mauvaise touche.

src/main.rs : Point d’entrée du programme, analyse des arguments en ligne de commande (via Clap) et lancement du mode classique ou multi‑joueurs.

