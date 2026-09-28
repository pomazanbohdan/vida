# Direct-conversation ownership: decision brief

**Status: recommendation only.** VIDA already makes `Space` the top-level membership, key, policy and sync boundary. A person's Personal Space has one Owner and is not implicitly made shared. The direct-chat choice must preserve those rules; it is not settled by a transport library.

| Option | Example and benefit | Cost or conflict |
|---|---|---|
| A. Lightweight Conversation Space (recommended) | Богдан starts a private chat with Олена. VIDA creates a separate Shared Space of conversation type, visible as one Messenger thread rather than a workspace. Both use ordinary Space membership, grants, key epochs and sync; neither Personal Space opens. | A large number of small Spaces; genesis Owner is the creator, so the later role of the second participant and departure/removal behavior still need an explicit choice. |
| B. Shared Conversation resource in the initiator's Personal Space | Богдан creates a Chat resource in his Personal Space and gives Олена a narrow Guest grant, similar to sharing a note. Fewer Spaces. | Ownership is asymmetric: Олена's messages live under Богдан's Personal Space, with its Owner-governance and recovery boundary. Must prove that her other personal data and hidden backlinks do not leak. |
| C. New pairwise security domain outside Space | A direct chat is a separate Core object with its own membership, encryption and sync. It need not inherit Space Owner semantics. | Duplicates the current Core membership/key/revocation model and creates a second authority path for every message/file/call. Highest specification and audit burden. |

Project Chat and Forum remain inside their owning Shared Project Space under the already approved product model. A one-to-one discussion **about** a task could be either a link to a separate direct chat or a narrowly scoped resource inside Project Space; those have different visibility to Project Owners and must not be labeled equally private without a decision.

## Reference signals, not VIDA decisions

- [Matrix Direct Messaging](https://spec.matrix.org/latest/client-server-api/#direct-messaging) models a direct chat as a room marked direct; membership belongs to the room. This supports Option A's shape but does not import Matrix's server, room authorization or encryption model.
- [Signal Private Group System](https://signal.org/blog/signal-private-group-system/) shows why shared membership consistency and privacy require explicit design. Its service-backed encrypted group state is not evidence that VIDA must use a server.
- [SimpleX Chat protocol](https://simplex.chat/docs/protocol/simplex-chat.html) demonstrates a distinct pairwise-connection model; copying it would move VIDA toward Option C rather than reuse existing Space controls.

## Decision needed

1. For a normal 1:1 Messenger chat independent of a Project, choose A, B or C.
2. If A, should both participants become equal Space Owners after invitation acceptance, or should the creator remain Owner and the recipient get another role? Either choice changes who can remove whom under VIDA's existing Owner rules.
3. For a Project-linked 1:1 chat, should the chat remain outside the Project Space with only a permission-checked link, or live inside the Project Space and inherit its governance?
