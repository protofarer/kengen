```mermaid
---
Game Run Mode State Machine
---
stateDiagram-v2
    [*] --> Stopped
    state In-Game {
        Running --> Paused
    }
    Stopped --> Running: resume
    Running --> Stopped
    Paused --> Running
    Paused --> Stopped
    Stopped --> Exiting
    Exiting --> [*]
```