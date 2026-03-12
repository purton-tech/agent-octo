If we ask a model a question that requires external data, it cannot actually solve it on its own.

## Why?

```sh
curl https://api.openai.com/v1/chat/completions \
  -s \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -d '{
    "model": "gpt-5",
    "messages": [
      {
        "role": "user",
        "content":
        "Give me the average tempature day by day in london for the last 7 days."
      }
    ]
  }' | jq -r '.choices[0].message.content'
```

The model tells us it cannot access real data.

This is expected. LLMs do not have internet access, and they should not execute arbitrary code.

But now we introduce a tool.

Instead of answering directly, the model can generate code that we run in a sandbox.

```sh
curl https://api.openai.com/v1/chat/completions \
  -s \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -d '{
    "model": "gpt-5",
    "messages": [
      {
        "role": "user",
        "content": "Give me the average tempature day by day in london for the last 7 days."
      }
    ],
    "tools": [
      {
        "type": "function",
        "function": {
          "name": "run_sandbox_code",
          "description": "Execute code in a sandboxed environment with internet access.",
          "parameters": {
            "type": "object",
            "properties": {
              "language": {
                "type": "string",
                "enum": ["python"]
              },
              "code": {
                "type": "string"
              }
            },
            "required": ["language", "code"]
          }
        }
      }
    ]
  }'
```

The model responds with Python code that fetches historical weather data and computes the averages.

```python
import json
from datetime import date, timedelta

import requests


# Compute last 7 complete days (excluding today)
end_date = date.today() - timedelta(days=1)
start_date = end_date - timedelta(days=6)

lat, lon = 51.5074, -0.1278  # London
timezone = "Europe/London"

url = (
    "https://archive-api.open-meteo.com/v1/archive?"
    f"latitude={lat}&longitude={lon}"
    f"&start_date={start_date}&end_date={end_date}"
    f"&daily=temperature_2m_mean&timezone={timezone}"
)

resp = requests.get(url, timeout=20)
resp.raise_for_status()
js = resp.json()

if (
    "daily" not in js
    or "time" not in js["daily"]
    or "temperature_2m_mean" not in js["daily"]
):
    raise RuntimeError("Unexpected response format from Open-Meteo")

dates = js["daily"]["time"]
avgs = js["daily"]["temperature_2m_mean"]
unit = js.get("daily_units", {}).get("temperature_2m_mean", "°C")

rows = [
    {
        "date": d,
        "average_temperature": round(t, 1) if isinstance(t, (int, float)) else None,
        "unit": unit,
    }
    for d, t in zip(dates, avgs)
]

result = {
    "city": "London",
    "start_date": str(start_date),
    "end_date": str(end_date),
    "daily": rows,
}

print(json.dumps(result))
```

We take that code, execute it in a sandbox, and return the result.

```json
{
  "city": "London",
  "start_date": "2026-03-05",
  "end_date": "2026-03-11",
  "daily": [
    { "date": "2026-03-05", "average_temperature": 11.6, "unit": "°C" },
    { "date": "2026-03-06", "average_temperature": 9.1, "unit": "°C" },
    { "date": "2026-03-07", "average_temperature": 8.1, "unit": "°C" },
    { "date": "2026-03-08", "average_temperature": 8.8, "unit": "°C" },
    { "date": "2026-03-09", "average_temperature": 9.7, "unit": "°C" },
    { "date": "2026-03-10", "average_temperature": 10.1, "unit": "°C" },
    { "date": "2026-03-11", "average_temperature": 10.0, "unit": "°C" }
  ]
}
```

Now the model can solve problems that require:

- APIs
- computation
- data processing

This pattern is sometimes called a Code Interpreter, Sandbox Tool, or Agent Tool Execution.

But the moment you do this, a new problem appears.

You are now executing code written by an LLM.

That means you need a sandbox.

## Just Use a Docker Container

A common first idea is: just run the code inside Docker.

There are even projects like:

- <https://github.com/agent-infra/sandbox>

A simple sandbox image might look like this:

```Dockerfile
FROM python:3.11-slim

WORKDIR /sandbox

RUN pip install --no-cache-dir \
    requests \
    numpy \
    pandas \
    matplotlib \
    scipy \
    scikit-learn \
    beautifulsoup4 \
    lxml

RUN useradd -m sandbox
USER sandbox

CMD ["python"]
```

Then we execute the generated script.

```sh
docker build -t python-sandbox .
docker run --rm -v "$PWD/script.py:/sandbox/script.py:ro" python-sandbox python /sandbox/script.py
```

For a single-user demo, this works perfectly.

But once you move beyond a demo, things change quickly.

## The Multi-User Reality

As soon as multiple users are involved, the problem becomes architectural.

You cannot just call `docker run` anymore.

You now need to manage:

- one container per execution
- request queues
- timeouts
- resource limits
- network restrictions
- container cleanup
- per-user quotas

The system quickly turns into something like this:

```text
request -> queue -> worker -> container -> result -> destroy container
```

At this point you are no longer just running a container.

You are building a distributed job execution system.

This is the moment many teams realise they are reinventing infrastructure.

## Who Has Already Solved This?

Several projects are already exploring this space:

- <https://github.com/vercel-labs/just-bash>
- <https://github.com/pydantic/monty>
- <https://cloud.google.com/blog/products/containers-kubernetes/agentic-ai-on-kubernetes-and-gke>

These systems manage things like:

- sandbox lifecycle
- resource isolation
- job scheduling
- scaling execution environments

Which brings us to the next step.

## Sandboxing on Kubernetes

Many teams eventually run these sandboxes on Kubernetes.

Why?

Because Kubernetes already solves several problems we just described:

- scheduling workloads
- isolating containers
- scaling execution workers
- managing resource limits
- cleaning up completed jobs

Instead of writing your own orchestration layer, you can create ephemeral jobs or pods that execute sandboxed code.

![Sandboxing on Kubernetes](k8s-sandboxing.jpg "Sandboxing on Kubernetes")

## The Takeaway

Adding a sandbox tool looks simple.

But once real users are involved, you are designing:

- a sandbox
- a scheduler
- a job execution system
- and sometimes a multi-tenant security boundary

This is why many modern AI systems build on top of container orchestration or purpose-built sandbox infrastructure rather than calling `docker run` directly.
