# Group-call capacity — official-source digest, round 1

Accessed: 2026-09-22.

## Claims

1. LiveKit's published single-node synthetic benchmark reports a 150-publisher/150-subscriber 720p room on a 16-core `c2-standard-16` at 85% CPU. This demonstrates SFU headroom, not Flutter/mobile usability or E2EE client capacity. Source: [LiveKit Benchmarking](https://docs.livekit.io/transport/self-hosting/benchmark), publisher LiveKit, publication date not stated, confidence high, class performance/scale.
2. LiveKit's official load tool supports simulcast publishers, subscribers and 3×3/4×4/5×5 layouts; its docs warn that bandwidth and host tuning materially affect results. Source: [livekit-cli](https://github.com/livekit/livekit-cli), publisher LiveKit, continuously updated, confidence high, class compatibility/testing.
3. LiveKit Flutter exposes adaptive stream and dynacast; dynacast pauses unused video layers and therefore is directly relevant to small-team grids. Source: [LiveKit Flutter SDK](https://github.com/livekit/client-sdk-flutter) and [advanced media docs](https://docs.livekit.io/transport/media/advanced/), publisher LiveKit, continuously updated, confidence high, class compatibility.
4. Signal currently permits 75 participants in an encrypted group video call, but limits ringing notification to small groups of up to 16. This is a product/notification boundary, not a published quality benchmark. Source: [Signal Group Video Calling](https://support.signal.org/hc/en-us/articles/360052977792-Group-Video-Calling), publisher Signal, publication date not stated, confidence high, class product limit.
5. Slack paid huddles allow 50 participants and at most 25 with video. Slack specifies 2 Mbps download and 600 kbps upload for video huddles with 5+ participants. These are service limits/requirements, not VIDA device proof. Sources: [Slack Huddles](https://slack.com/help/articles/4402059015315-Use-huddles-in-Slack) and [Slack A/V troubleshooting](https://slack.com/help/articles/115003538426-Troubleshoot-audio-and-video-issues-in-Slack), publisher Slack, publication date not stated, confidence high, classes product limit/network requirement.
6. Teams desktop lets users select 4, 9, 16 or 49 visible videos; 49 requires stronger hardware and Teams may reduce visible video under bandwidth or memory pressure. Teams mobile documents roughly 8–10 featured simultaneous videos depending on view/device. Source: [Microsoft Teams video](https://support.microsoft.com/en-US/teams/meetings/use-video-in-microsoft-teams), publisher Microsoft, publication date not stated, confidence high, class client UX/capability.

## Decision implication

- Published SFU maxima are far above small-team needs and must not set the product promise.
- A **16-participant room target** is a defensible small-team envelope: it aligns with Signal's small-group ringing boundary and a standard Teams desktop gallery tier while remaining below Slack's 25-video boundary.
- Client rendering must be decoupled from room membership: propose at most **9 simultaneous remote video tiles on mobile** and up to **16 on Windows**, using active-speaker/pin/adaptive subscription. All participants remain in the call and may publish; off-screen tracks may be paused/downscaled.
- This is a research recommendation only. VIDA must prove the target with E2EE enabled on representative physical devices; no cited vendor source proves VIDA's Flutter/Rust/E2EE implementation at 16 participants.

## Missing evidence

- No current primary source found a comparable end-to-end Flutter mobile benchmark with 16 E2EE publishers/receivers.
- Vendor maximums do not report thermal, battery, decode count or 30-minute stability on low/mid-range phones.
