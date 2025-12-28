```mermaid
flowchart TB
  T["Tourist (pays in USD)"] --> P["Payment initiated\n(card / wallet)"]
  P --> E["Escrow (value locked)"]

  E --> V{"Experience completed?"}

  V -- Yes --> C["Mutual confirmation (tap / time / location)"]
  C --> S["Settlement trigger (smart contract event)"]
  S --> G["Local guide paid (settled to KRW)"]

  V -- Dispute --> D["Dispute window (time-limited)"]
  D --> R["Resolution (predefined rules)"]
  R --> G

  class E,S Aqua
  class V,D,R Sky

  classDef Aqua stroke-width:1px,stroke-dasharray:none,stroke:#46EDC8,fill:#DEFFF8,color:#378E7A
  classDef Sky  stroke-width:1px,stroke-dasharray:none,stroke:#374D7C,fill:#E2EBFF,color:#374D7C
