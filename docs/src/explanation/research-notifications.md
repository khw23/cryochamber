# Quiet research workflow (personal fork)

This opt-in feature does not replace the scientific model or guarantee scientific
correctness. It separates routine progress from human replies and failures.

Set top-level `research_notifications = true` in the chamber's `cryo.toml`.
Optionally set `progress_interval_secs = 86400` (the default, 24 hours).
Other chambers retain the upstream defaults. Use matching binaries from this
fork: IPC v11 intentionally rejects a v10 daemon/client mixture. Updating only
the Python runner or only `cryo-agent` is not sufficient.

The agent writes a plain-language scientific synthesis and calls
`cryo-agent progress --stdin`. Ordinary delivery is allowed no more often than
the configured interval, within 08:00–22:00 of the daemon's local clock.
Choose the daemon's timezone explicitly during deployment if the operator is
elsewhere. Identical text is suppressed. Pending/latest text and last delivery
are local to `.cryo/progress-state.json`, never a shared scientific source.
There is no timer-driven delivery: at the next real wake the agent must update
its interpretation and call progress again. A day with no meaningful progress
need not produce a message. Zero interval removes the interval restriction but
not quiet hours or exact-duplicate suppression.

Replies to the operator, important corrections, major findings, real blockers
and urgent risks use `send` immediately. A received human message cannot be
answered by a suppressed progress call. Missing replies and actual crashes
retain daemon fallback handling. These exceptions are based on the agent's
judgment; this small patch does not semantically police misuse of `send`.

Successful unchanged wakes may remain silent. Plain hibernate may wait only
for inbox activity without a dummy TODO; timed computation follow-up still
needs a TODO. `pause --summary ...` stops the chamber cleanly after the current
agent exits. It does not mark the plan complete or kill external computations.
Claimed unfinished TODOs are consumed and copied to fresh pending IDs without
crash backoff. Other TODOs remain intact. Resume with an explicit operator
`cryo restart` after repair; new Zulip messages alone do not restart a paused
daemon. Standard daemon services restart on failure, not on this successful exit.

Before deployment, test in an isolated mock chamber: silent wake, ordinary
progress coalescing, human reply, real crash, idle wait, pause and restart.
No deployment or provider/credential migration is performed by changing source.
