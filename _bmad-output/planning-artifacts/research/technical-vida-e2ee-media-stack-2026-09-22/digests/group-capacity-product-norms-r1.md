# Small-team product norms digest — round 1

Accessed: 2026-09-22. Eight distinct sources.

## Comparison

| Product/evidence | Ceiling | Relevant small-team/display behavior |
|---|---:|---|
| [Signal](https://support.signal.org/hc/en-us/articles/360052977792-Group-Video-Calling) | 75 | Ringing notification only for groups up to 16; ceiling is not a grid benchmark. |
| [Slack Huddles](https://slack.com/help/articles/4402059015315-Use-huddles-in-Slack) | 50 paid | At most 25 video; contextual, drop-in, camera opt-in. [Bandwidth guide](https://slack.com/help/articles/115003538426-Troubleshoot-audio-and-video-issues-in-Slack) gives 2 Mbps down/600 kbps up for 5+ video. |
| [Teams limits](https://learn.microsoft.com/pt-br/microsoftteams/limits-specifications-teams) | 20 chat call; much higher meetings | [Gallery](https://support.microsoft.com/en-us/teams/meetings/use-video-in-microsoft-teams) uses 4/9/16/49 on desktop, roughly 8–10 featured on mobile and may reduce tiles for resource pressure. |
| [Google Meet limits](https://support.google.com/meet/answer/10620582?hl=en) | 100/150/500 | [Layouts](https://support.google.com/meet/answer/9292748?co=GENIE.Platform%3DDesktop&hl=en) decouple occupancy from visible tiles and warn about CPU/freezing. |
| [LiveKit benchmark](https://docs.livekit.io/transport/self-hosting/benchmark/) | 150×150 synthetic | Server result only; no mobile/client usability proof. |
| [Online focus-group study](https://journals.sagepub.com/doi/10.1177/14687941221110161) | n/a | 4–6 directional literature range; authors found 4–5 effective in their context. Not a universal technical cap. |

## Synthesis

- Marketing/attendance ceilings consistently exceed simultaneous visible-video defaults.
- Small-team UX should optimize conversation for **4–6**, then use active speaker, pinning and selective subscriptions.
- Independent recommendation: Release-1 hard cap **8 total concurrent people**, with a one-page desktop grid and active-speaker + filmstrip/swipe on mobile. Treat 12/16 as later stretch targets after physical-device/network validation.
- No current independent Element/Matrix/LiveKit end-client usability benchmark was found; older full-mesh limits are obsolete after SFU migration.
