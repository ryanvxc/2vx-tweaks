# Model contributions

Which Claude models did what in the 0.4 redesign. The owner asked for three models with separate jobs, and for this
record to be kept.

| Model | ID reported by the agent | Job |
| --- | --- | --- |
| Opus 5.5 | `claude-opus-5-5` | Lead architect: coordination, plans, the shared layer, backend work, reviews |
| Fable 5.1 | `claude-fable-5-1` | Independent product and UX critic: compared the app with the references and challenged decisions |
| Sonnet 5.5 | `claude-sonnet-5-5` | Focused implementation of single pages, with tests |

## How the agents ran

The role files are in `.claude/agents/`. Project agent types don't load into a session that's already running, so each
agent ran as a built-in type with a model override, and its prompt told it to read and follow its role file:

| Role file | Ran as |
| --- | --- |
| `product-ux-critic.md`, `qa-reviewer.md` | `Plan` (read-only) with `model: fable` |
| `systems-architect.md` | `Plan` with `model: opus` for audits, `general-purpose` with `model: opus` for the backend |
| `ui-implementation-worker.md` | `general-purpose` with `model: sonnet` |

Each agent reported the model it ran on; those are the IDs above.

## Who did what

| Area | Model | What |
| --- | --- | --- |
| Coordination and shared layer | Opus 5.5 (lead session) | Audits, navigation, tweak placement, `TweakGrid`, filters and sorts, presets, `PresetCard`, tweak details, shell and page title clean-up, design system doc, integration, real-app tours |
| Backend | Opus 5.5 (worker) | Hardware details, service states, ping, Adapter Tuner and its policy, Windows tools, report export, auto launch, scheduled startup tasks, the engine fixes, the guard tests |
| Backend review | Opus 5.5 (independent read-only agent) | Reviewed the whole backend diff; found the adapter undo defect and the weak guards |
| Backend fixes | Opus 5.5 (worker) | Fixed that review's findings, including the journal's device column |
| Product critique | Fable 5.1 | Compared the plan and the app with the references before and after building, and challenged decisions |
| QA | Fable 5.1 | Reviewed real-app screenshots against the references and looked for weaknesses |
| Home, Resources, Settings, Account, Restore Centre | Sonnet 5.5 | Page rebuilds with tests |
| Hardware | Sonnet 5.5 | Real facts, device chip, GPU profiles |
| PC Cleaner | Sonnet 5.5 | System Cleaner progress and summary, Services read-out, Autoruns |
| Network and Advanced | Sonnet 5.5 | Adapter Tuner, Bufferbloat, Timer Resolution, MSI Mode, Devices |

Where the models disagreed, the lead made the call and moved on. Fable's "Expected effect" filter stayed because the brief
asks for impact. Flat rows stayed where EXM uses taller ones, because the window is fixed. Fable's final QA asked for a cap on
the sidebar's gaps; that was declined, because the owner asked for the sidebar to stretch down. The rest of its must-fix list
was done, and its nice-to-haves except the same icon repeated in long lists, which the owner's request for icons outranks.
