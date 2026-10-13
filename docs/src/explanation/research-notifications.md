# Event-driven research reporting (personal fork)

Version 0.2.8-research.2, branch `research/event-progress`. This is not an
upstream release or a replacement model. It cannot guarantee scientific quality.

Set top-level `research_notifications = true` in the chamber's `cryo.toml`.
Other chambers retain the upstream per-wake fallback/TODO defaults.
Use matching binaries and restart the daemon deliberately at a safe boundary.
IPC v11 rejects v10 clients; v11 alone does not distinguish research.1 from
research.2, so check the build/version as well. Updating a runner or source checkout
does not update the daemon already running.

## What gets sent

The agent interprets its results and writes a scientific explanation:
the question, why this method helps, the result's meaning and limits, the next
useful test. Use `cryo-agent progress --stdin` for that synthesis. It sends
immediately, including at night: no interval, daily quota, count limit, exact-text
filter or pending queue. Two valuable stages in two hours can produce two reports.
Combine commands and intermediate checks into their scientific result, not whole
days of results into one message. A useful negative result is progress too.

No result/change means no unsolicited message. The daemon cannot judge scientific
significance and does not rewrite messages. Explicit user quiet-hour preferences
belong in the project plan; there is no default nighttime embargo.

Use `send` for direct human replies and urgent alerts (it can also carry results).
A received batch must be answered with `send`; `progress` refuses while such a
reply is owed. Missing replies and actual crashes retain fallback handling.
Successful unchanged wakes may stay silent.

## Plan, memory, and waiting

Native `plan.md` holds the scientific plan, `NOTES.md` the current understanding
and evidence links. Neither needs to become a pointer to a separate daily plan
or duplicate memory. A workflow can add stable background and a resource envelope;
Cryochamber itself does not authorize scientific computation or submit HPC jobs.

Continue useful ready work in a session. For a real computation wait, schedule a
TODO and hibernate; an inbox-only wait can hibernate without a dummy TODO in this
profile. `pause --summary ...` stops dispatch after the agent exits without
declaring the project complete or cancelling external calculations. Claimed
unfinished TODOs receive fresh pending identities. Explicit operator restart is
needed; new mail does not restart a stopped daemon.

## Upgrade from research.1

`progress_interval_secs` is deprecated: accepted on read but ignored and omitted
when config is serialized. Remove it deliberately during deployment to avoid
misleading operators. Existing `.cryo/progress-state.json` is left untouched,
never loaded or replayed. Review any unsent findings against the current report;
do not blindly send stale queued text. No automatic file migration is performed.

Before deployment, test an isolated mock chamber: two immediate results, nighttime
results, silent unchanged wake, human reply, real failure, pause and restart.
Only these mechanics are tested; clarity/significance require real scientific use.
