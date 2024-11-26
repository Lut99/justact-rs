# juastact-rs: JustAct Core Ontology
This crate defines the core ontology for the JustAct framework, as presented in the paper [\[1\]](#references).

For a concrete, running instance of a JustAct-compatible system, see <https://github.com/Lut99/justact-prototype-rs>.


## Ontology
The currently implemented ontology is a little simpler, yet more concrete, than the one in the paper.

In particular, this crate defines three types of sets:
- `Statements` defines an _asynchronized_ set of `Message`s and `Action`s which, individually, may or may not have been _stated_ or _enacted_. Being asynchronized, every agent can have a local view on this set that may disagree with other agents.
- `Agreements` defines a _synchronized_ set of `Agreement`s that are messages upon which agents agree. At all times, exactly one of these is _active_, and which depends on the _timestamp_ associated with each agreement. Being synchronized, all agents must agree on all of the contents of this set.
- `Times` defines a _synchronized_ set of all known `Timestamp`s. One of these is the "active" timestamp, which determines which agreement is active. Being synchronized, all agents must agree on all of the contents of this set.

Further, this crate also defines:
- `Agent`s, which abstractly represent an agent in a JustAct framework;
- `Policy`s, which abstractly represent something that has a validity associated with it and is extracted from messages; and
- `Extractor`s, which are functions that map message sets to policy.


## Features
This crate has no features.


## Contribution
Contributions to this crate are welcome! If you have any suggestions, fixes or ideas, please feel free to [leave an issue](/Lut99/justact-rs/issues) or [create a pull request](/Lut99/justact-rs/pulls).


## License
This project is licensed under Apache 2.0. See [LICENSE](./LICENSE) for more details.


## References
\[1\] Esterhuyse, C.A., Müller, T., van Binsbergen, L.T. (2024). _JustAct: Actions Universally Justified by Partial Dynamic Policies._ In: Castiglioni, V., Francalanza, A. (eds) Formal Techniques for Distributed Objects, Components, and Systems. FORTE 2024. Lecture Notes in Computer Science, vol 14678. Springer, Cham. <https://doi.org/10.1007/978-3-031-62645-6_4>
