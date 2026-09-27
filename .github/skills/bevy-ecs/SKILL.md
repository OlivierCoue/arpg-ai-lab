## Bevy API research

Before implementing Bevy ECS code:

1. Search `/workspaces/arpg-ai-lab/bevy/examples/` for relevant patterns.
2. If a filesystem search does not find the examples, verify the directory with `ls`/filesystem tools before assuming it is absent.
3. Do not fall back to remembered Bevy APIs because an example search failed.
4. Inspect the matching example(s) and adapt their current API usage.
5. Verify unclear APIs against the local Bevy source.
6. Do not rely on memory or web examples from other Bevy versions.

Prefer existing local examples over remembered or generic Bevy patterns.