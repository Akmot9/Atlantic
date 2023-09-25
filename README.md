# Atlantic

## Introduction

Atlantic est une application destinée à la capture et à l'analyse de paquets réseau en temps réel. Elle est développée en Rust et utilise la bibliothèque `pnet` pour la capture des paquets. L'outil permet la surveillance de plusieurs interfaces réseau et offre des fonctionnalités telles que le filtrage des paquets, l'enregistrement dans un fichier CSV et la possibilité d'une analyse en temps réel.

## Fonctionnalités

- Capture des paquets en temps réel
- Supporte IPv4 et IPv6
- Enregistre les données dans un fichier CSV
- Filtre les paquets en fonction de divers critères
- Supporte la surveillance de plusieurs interfaces réseau (via multi-threading)
- Interface graphique (En développement)

## Installation

```bash
# Clone the repository
git clone https://github.com/akmot9/Atlantic.git

# Move to the directory
cd Atlantic

# Build the project
cargo build --release
```

## Utilisation

Lancer l'application :

```bash
sudo ./target/release/Atlantic
```

Vous pouvez spécifier diverses options via la ligne de commande (consultez la section "Options" pour plus de détails).

## Options

```
--interface   Choisissez une interface réseau pour la surveillance
--filter      Appliquez un filtre pour capturer des paquets spécifiques
--timeout     Durée en secondes avant l'arrêt automatique de la capture
```

## Dépendances

- Rust 1.5+ 
- `pnet` library
- `csv` library

## Licence

Ce projet est sous licence MIT.
