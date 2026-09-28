# LiveKit/SFU capacity digest — round 1

Accessed: 2026-09-22. Primary/current sources.

## Findings

- LiveKit reports a synthetic 150-publisher/150-subscriber, 720p single-room result at 85% CPU on a 16-core GCP node. A room fits on one node and work scales with tracks, subscribers and forwarded bytes. Source: [LiveKit benchmark](https://docs.livekit.io/transport/self-hosting/benchmark/), confidence high for the stated lab result, low for VIDA client capacity, class performance/scale.
- Official `lk load-test` uses synthetic Go clients and supports publishers/subscribers, speaker/3×3/4×4/5×5 layouts, simulcast, resolution and ramp rate. It does not exercise Flutter decode/render, thermals, TURN, E2EE or long-call stability. Source: [livekit-cli](https://github.com/livekit/livekit-cli#load-testing), confidence high, class test methodology.
- LiveKit Flutter lists Android/iOS/Windows and exposes adaptive stream/dynacast. Adaptive stream reduces hidden/off-screen subscriptions; dynacast pauses unused publication layers. Sources: [Flutter SDK](https://github.com/livekit/client-sdk-flutter), [media subscription](https://docs.livekit.io/transport/media/subscribe/), [dynacast API](https://docs.livekit.io/reference/client-sdk-flutter/livekit_client/RoomOptions/dynacast.html), confidence high, class compatibility.
- WebRTC SVC availability is the intersection of codec, endpoint and SFU capability; negotiation may differ from the requested mode. Source: [W3C WebRTC-SVC](https://www.w3.org/TR/webrtc-svc/), 2026-09-14, confidence high, class standards.
- An older official Flutter issue reports large client rendering CPU differences on one iPhone model. It is not a current scale benchmark, but confirms endpoint rendering must be measured. Source: [LiveKit Flutter issue 364](https://github.com/livekit/client-sdk-flutter/issues/364), 2023–2024, confidence medium/old, class client risk.

## Recommendation

Release-1 provisional cap: **8 total participants**, all permitted to publish audio/video. Validate eight physical Flutter clients for 60 minutes with E2EE, 720p simulcast, adaptive stream, dynacast, Wi-Fi/cellular/TURN, reconnects and low/mid/high device classes. Until this passes, wording is “designed for 8”; afterward, “supports 8 on tested devices/networks.”

No official LiveKit source publishes a guaranteed Flutter participant count or cross-device thermal/decode matrix. Server capacity must not set the client product promise.

