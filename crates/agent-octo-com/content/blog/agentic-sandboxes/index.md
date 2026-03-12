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
I don't have real-time access to current weather data. However, you can check 
the average daily temperatures for London over the past week using weather websites such as:

- [Weather.com](https://weather.com/)
- [AccuWeather](https://www.accuweather.com/)
- [BBC Weather](https://www.bbc.com/weather)

Alternatively, you can use weather APIs like OpenWeatherMap or Meteostat to get 
historical temperature data programmatically. If you need help with that or want me to 
guide you through obtaining this data, let me know!
```

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

```json
{"code":"import requests\nfrom datetime import date, timedelta\nimport json\n\n# Compute last 7 complete days (excluding today)\nend_date = date.today() - timedelta(days=1)\nstart_date = end_date - timedelta(days=6)\n\nlat, lon = 51.5074, -0.1278 # London\ntimezone = \"Europe/London\"\n\nurl = (\n \"https://archive-api.open-meteo.com/v1/archive?\"\n f\"latitude={lat}&longitude={lon}\"\n f\"&start_date={start_date}&end_date={end_date}\"\n f\"&daily=temperature_2m_mean&timezone={timezone}\"\n)\n\nresp = requests.get(url, timeout=20)\nresp.raise_for_status()\njs = resp.json()\n\nif 'daily' not in js or 'time' not in js['daily'] or 'temperature_2m_mean' not in js['daily']:\n raise RuntimeError(\"Unexpected response format from Open-Meteo\")\n\ndates = js['daily']['time']\navgs = js['daily']['temperature_2m_mean']\nunit = js.get('daily_units', {}).get('temperature_2m_mean', '°C')\n\nrows = [\n {\"date\": d, \"average_temperature\": round(t, 1) if isinstance(t, (int, float)) else None, \"unit\": unit}\n for d, t in zip(dates, avgs)\n]\n\nresult = {\n \"city\": \"London\",\n \"start_date\": str(start_date),\n \"end_date\": str(end_date),\n \"daily\": rows,\n}\n\nprint(json.dumps(result))\nresult"}
```

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
result
```

```
{"city": "London", "start_date": "2026-03-05", "end_date": "2026-03-11", "daily": [{"date": "2026-03-05", "average_temperature": 11.6, "unit": "\u00b0C"}, {"date": "2026-03-06", "average_temperature": 9.1, "unit": "\u00b0C"}, {"date": "2026-03-07", "average_temperature": 8.1, "unit": "\u00b0C"}, {"date": "2026-03-08", "average_temperature": 8.8, "unit": "\u00b0C"}, {"date": "2026-03-09", "average_temperature": 9.7, "unit": "\u00b0C"}, {"date": "2026-03-10", "average_temperature": 10.1, "unit": "\u00b0C"}, {"date": "2026-03-11", "average_temperature": 10.0, "unit": "\u00b0C"}]}
```

Here’s a **tightened version** that keeps your example but adds the **architectural reality check** without becoming long.

---

## Just Use a Docker Container

A common first idea is: *just run the code inside a Docker container.*

There are even projects like

* [https://github.com/agent-infra/sandbox](https://github.com/agent-infra/sandbox)

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

Then run the user code:

```sh
docker build -t python-sandbox .
docker run --rm -v "$PWD/script.py:/sandbox/script.py:ro" python-sandbox python /sandbox/script.py
```

This works well for **single-user demos**.

---

## The Multi-User Reality

Once multiple users are involved, things get more complicated.

You typically need:

* **one container per execution**
* a **queue** to control concurrency
* **CPU / memory limits**
* **timeouts**
* **network restrictions**
* **filesystem isolation**
* **cleanup of finished containers**
* **rate limiting per user**

Very quickly the architecture becomes something like:

```
request → queue → worker → container → collect result → destroy container
```

At this point you’re essentially building a **job execution system**.

Which is why many systems eventually move to things like:

* **Kubernetes Jobs**
* **gVisor / Firecracker sandboxes**
* purpose-built sandbox infrastructure

The important takeaway:

> Calling `docker run` looks simple, but supporting multi-user sandbox execution quickly turns into building a scheduling, isolation, and resource management system.

## Who's already built this and how can we use it?

- https://github.com/vercel-labs/just-bash
- https://github.com/pydantic/monty
- https://cloud.google.com/blog/products/containers-kubernetes/agentic-ai-on-kubernetes-and-gke

## Sandboxing on Kubernetes

https://cloud.google.com/blog/products/containers-kubernetes/agentic-ai-on-kubernetes-and-gke

![Sandboxing on Kubernetes](k8s-sandboxing.jpg "Sandboxing on Kubernetes")
