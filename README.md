# AI Harness

A small coding-agent harness for open-weight models. The weights stay outside this repo. Any chat model that is served behind an OpenAI-style HTTP API can be plugged in by setting a base URL and a model name.

An agent is a model plus a harness. The model proposes the next step. The harness decides what the model can see, which tools exist, which calls actually run, and how the results come back. This repository is the harness.

## Idea

Closed models are reached through someone else's API. They come with rate limits, and the weights never leave that company's machines. An open-weight model can be downloaded and served locally. Once the harness speaks a normal chat API, swapping models is a configuration change: another name, another context length.

The inference server is a separate program. Ollama, llama.cpp, and vLLM already load weights, apply the chat template, and sample tokens. This project talks to that server. It does not load weight files itself.

The first aim is to show that the harness works. Daily use on a stronger model can wait until there is hardware for it. The client stays the same. Only the server address changes.

## What the harness is

One loop:

1. Send the conversation to the model server.
2. Read a tool call from the reply.
3. Check the call. Run it only when it is valid and allowed.
4. Append the tool result and take another step.
5. Stop when the model is done, or when a step limit is hit.

The first tool set is small: read a file, search, replace a string, run a command. A permission switch covers read-only use, and asking before a write or a shell command.

A working harness, on one machine, means all of the following:

- An 8B or 14B model answers through a local server.
- One tool runs, the result comes back, and the model takes another step.
- A malformed tool call is refused.
- A second model name uses the same loop with no code change.

Context stays on the order of 8k–16k tokens, so the weights and the chat cache both fit in VRAM. Each tool round sends the growing transcript through the model again, which is why a short context matters as much as raw tokens per second.

## This machine

Development machine: Windows, RTX 5060 Ti with 16 GB of VRAM, 16 GB of system RAM. Windows leaves about 15 GB of VRAM usable.

Models that sit entirely in that budget are the ones to test with. Published figures for this card at 4-bit weights:

| Model | Fit | Speed (short context) |
| --- | --- | --- |
| 8B | Comfortable | about 50–70 tokens/s |
| 14B | Comfortable if context stays shorter | about 30–40 tokens/s |
| ~20B mixture-of-experts | Often fits, and stays fast because few weights run per token | varies by model |
| Dense 32B | About 20 GB, so part of it spills into system RAM | a few tokens/s |
| 70B | Does not fit | — |

An 8B or 14B coder model is enough to prove the loop and to help with small jobs: chat, short edits, search, a few tool calls. Long “read the whole repo” runs squeeze the same 15 GB that the weights occupy. Quality on multi-step tool use is the ceiling of this card. Speed inside the sizes above is already fine.

## Later hardware

A stronger model is a server change. One office machine with one or two GPUs in the 48–96 GB range can hold a 32B model, or a larger mixture-of-experts model, at a speed a few people can share. Each person runs this harness locally and points it at that server.

A rack of GPUs matters when many people generate at once, or when 70B-class models need to stay fast for a whole office. That is an office-capacity choice. Finishing this project does not depend on it.

## Status

Not started. This file is the project description, so the work can be picked up after the current project.

## License

Add a license before the first public push. MIT is a straightforward choice for a small tool like this.
