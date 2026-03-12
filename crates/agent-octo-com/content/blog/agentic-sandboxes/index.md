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

## Just use a Docker Container

## The Sanboxing Landscape



## Sandboxing on Kubernetes


![Sandboxing on Kubernetes](k8s-sandboxing.jpg "Sandboxing on Kubernetes")
