# AI/ML Club - Block Evolution

**Academic Year:** 2025-2026 Fall Semester  
**Project Duration:** September 2025 - December 2025

## Project Description

Use a genetic algorithm to evolve a population of graphs of neural network layers to achieve the lowest training loss in a limited amount of time

**Key Objectives:**
- Represent a graph of neural network layers as a gene-like string that remains valid under simple mutation and crossover operations
- Apply a genetic algorithm to neural network structure discovery
- Find an interesting alternative structure a human expert would be unlikely to guess for learning some dataset

## Lead Contact Information

**Project Lead:** Aili Nierengarten  
📧 Email: aili.nierengarten@gmail.com
💼 LinkedIn: https://www.linkedin.com/in/aili-nierengarten

## Contributors

*For detailed member information including LinkedIn profiles and Discord handles, see [docs/members.csv](docs/members.csv)*

| Name | Role | Email | GitHub |
|------|------|-------|--------|
| Aili Nierengarten | Project Lead | aili.nierengarten@sjsu.edu | [@8000thCube](https://github.com/8000thCube) |
| Alder Wang | Project Member | alder.wang@sjsu.edu | [@alderwang06](https://github.com/alderwang06) |
| Aysha Mujeeb | Project Member | aysha.mujeeb@sjsu.edu | [ayshamujeeb2007-cell](https://github.com/ayshamujeeb2007-cell) |
| Reema Karvir | Project Member | reema.karvir@sjsu.edu | [@github-username](https://github.com/username) |
| Thalia Hseih | Project Member | thalia.hsieh@sjsu.edu | [@LaplaceTransfem](https://github.com/LaplaceTransfem) |

## Repository Structure

```
block-evolution/
├── Cargo.toml
├── README.md
├── .gitignore
├── LICENSE
├── docs/
│   ├── members.csv             # Team member details with LinkedIn & Discord
│   ├── info.json               # Project metadata for website automation
│   ├── thumbnail.webp          # Project thumbnail image
│   └── pitch_slides.pdf        # Project presentation slides
├── src/
│   └── main.rs
```

## Quick Start Guide

### Prerequisites
- Cargo
- Rust

### Installation Guide

1. **Clone the repository:**
   ```bash
   git clone https://github.com/aiml-club/project-fa25-block-evolution.git
   cd project-fa25-block-evolution
   ```
2. **Make a copy of the build in the project directory:**
   ```bash
   cargo build --release
   cp target/release/block-evolution ./block-evolution
   ```

3. **Run the program:**
   ```bash
   cargo run --release
   ```

## Technology Stack

- **Programming Language:** Rust
- **ML/AI Libraries:** block-graph, burn
- **Version Control:** GitHub

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**Last Updated:** 2025-10-27
**Next Review:** [Date]

---

*This README follows the AI/ML Club standard template. For questions about the template or suggestions for improvements, contact the club leadership team.*
