# French Hunspell dictionary

`fr.aff` and `fr.dic` are the French dictionary "toutes variantes" (comprehensive: classic and
1990 reformed spellings both accepted) version 7.7 by Olivier R. and the Dicollecte contributors,
licensed under the **Mozilla Public License 2.0** (`LICENSE-MPL-2.0.txt`). Source:
<https://grammalecte.net>, taken unchanged from the Debian package
`hunspell-fr-comprehensive_7.7-1_all.deb` (`/usr/share/hunspell/fr.aff` and `fr.dic`).

| File | SHA-256 |
|---|---|
| `fr.aff` | `254756d4fd286a923b3f51dae65965bfc8619aa96db8528326bf31fcdc0840e4` |
| `fr.dic` | `140c5296b514bc55af7433f8913a73245b9b18116830a7db48e2aabfa85fcbed` |

Only the dictionary data comes from that project; no Grammalecte code or rule is used (ADR-0009).
The files are not modified. If they ever are, the modified files must be published under the MPL,
as the licence requires. The apps ship the licence text with the files (ADR-0025).
